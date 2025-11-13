use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use amina_core::service::{AppContext, Service, ServiceApi, ServiceInitializer};
use amina_core::tasks::{TaskContext, TaskManager};
use amina_core::events::{Event, EventEmitter};
use amina_core::rpc::Rpc;
use amina_core::register_rpc_handler;

#[derive(Serialize, Deserialize)]
#[derive(Event)]
#[key = "lappi.jobs.OnJobStateChanged"]
struct OnJobStateChanged {
    job_id: &'static str,
}

#[derive(Clone, Serialize)]
pub struct JobDescription {
    pub job_id: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
    pub description: &'static str,
}

pub trait JobFactory: Send + Sync {
    fn get_description(&self) -> Box<JobDescription>;
    fn is_always_ready(&self) -> bool;
    fn run(&self, job_ctx: Arc<JobContext>) -> Result<()>;
}

#[derive(Clone, Serialize)]
pub enum JobStage {
    NotReady,
    Ready,
    Started,
    Done,
}

impl JobStage {
    fn is_ready_to_start(&self) -> bool {
        match self {
            Self::Ready => true,
            Self::Done => true,
            _ => false
        }
    }
}

impl Default for JobStage {
    fn default() -> Self { JobStage::NotReady }
}

#[derive(Default, Clone, Serialize)]
pub struct JobState {
    stage: JobStage,
    progress: f32,
    state_text: String,
}

struct JobController {
    job_id: &'static str,
    factory: Arc<dyn JobFactory>,
    state: JobState,
    task_ctx: Option<Arc<TaskContext>>,
    events_emitter: Service<EventEmitter>,
}

impl JobController {
    fn set_ready(&mut self) {
        self.state.stage = JobStage::Ready;
        self.notify_on_state_changed();
    }

    fn try_start(&mut self, task_ctx: Arc<TaskContext>) -> bool {
        if !self.state.stage.is_ready_to_start() {
            return false;
        }

        self.state.stage = JobStage::Started;
        self.state.progress = 0.0;
        self.state.state_text = "Job started".to_string();
        self.task_ctx = Some(task_ctx);
        self.notify_on_state_changed();

        return true;
    }

    fn stop(&self) {
        if let Some(task_ctx) = &self.task_ctx {
            task_ctx.stop();
        }
    }

    fn set_progress(&mut self, progress: f32, state_text: String) {
        self.state.progress = progress;
        self.state.state_text = state_text;
        self.notify_on_state_changed();
    }

    fn finish(&mut self) {
        self.state.stage = JobStage::Done;
        self.task_ctx = None;
        self.notify_on_state_changed();
    }

    fn notify_on_state_changed(&self) {
        self.events_emitter.emit_event(&OnJobStateChanged {
            job_id: self.job_id
        });
    }
}

pub struct JobContext {
    controller: Arc<RwLock<JobController>>,
    task_ctx: Arc<TaskContext>,
}

impl JobContext {
    pub fn set_progress(&self, progress: f32, state_text: String) {
        let mut controller = self.controller.write().unwrap();
        controller.set_progress(progress, state_text);
    }

    pub fn is_interrupted(&self) -> bool {
        self.task_ctx.is_interrupted()
    }

    fn try_start(&self, task_ctx: Arc<TaskContext>) -> bool {
        let mut controller = self.controller.write().unwrap();
        controller.try_start(task_ctx)
    }

    fn finish(&self) {
        let mut controller = self.controller.write().unwrap();
        controller.finish();
    }

    fn get_job_factory(&self) -> Arc<dyn JobFactory> {
        let controller = self.controller.read().unwrap();
        controller.factory.clone()
    }
}

struct DummyJobFactory {
    description: Box<JobDescription>,
}

impl DummyJobFactory {
    fn new(name: &'static str, icon: &'static str, description: &'static str) -> Box<DummyJobFactory> {
        Box::new(DummyJobFactory {
            description: Box::new(JobDescription {
                job_id: name,
                name,
                icon,
                description
            })
        })
    }
}

impl JobFactory for DummyJobFactory {
    fn get_description(&self) -> Box<JobDescription> {
        self.description.clone()
    }

    fn is_always_ready(&self) -> bool {
        true
    }

    fn run(&self, job_ctx: Arc<JobContext>) -> Result<()> {
        let steps_num = 100i32;
        let pause_time = std::time::Duration::from_millis(100);

        for step in 0..steps_num {
            let progress = ((step + 1) as f32)/(steps_num as f32);
            let state_text = format!("Step {}/{}", step + 1, steps_num);
            job_ctx.set_progress(progress, state_text);

            if job_ctx.is_interrupted() {
                return Ok(());
            } else {
                std::thread::sleep(pause_time);
            }
        }

        Ok(())
    }
}

pub struct Jobs {
    jobs: RwLock<BTreeMap<&'static str, Arc<RwLock<JobController>>>>,
    task_manager: Service<TaskManager>,
    events_emitter: Service<EventEmitter>,
}

impl Jobs {
    pub fn register_job(&self, factory: Box<dyn JobFactory>) {
        let description = factory.get_description();
        let mut jobs = self.jobs.write().unwrap();

        let mut controller = JobController {
            job_id: description.job_id,
            factory: Arc::from(factory),
            state: JobState::default(),
            task_ctx: None,
            events_emitter: self.events_emitter.clone(),
        };

        if controller.factory.is_always_ready() {
            controller.set_ready();
        }

        jobs.insert(description.name, Arc::new(RwLock::new(controller)));
    }

    pub fn start_job(&self, job_id: String) -> Result<()> {
        let jobs = self.jobs.read().unwrap();
        let controller = jobs.get(job_id.as_str()).context("Invalid job_id")?;
        let controller = controller.clone();

        self.task_manager.run(move |task_ctx| {
            log::info!("Start '{}'", &job_id);

            let job_ctx = Arc::new(JobContext {
                controller,
                task_ctx: task_ctx.clone(),
            });

            let job_factory = job_ctx.get_job_factory();

            if job_ctx.try_start(task_ctx.clone()) == false {
                return;
            }

            log::info!("Run '{}'", &job_id);
            let result = job_factory.run(job_ctx.clone());
            match result {
                Ok(_) => {
                    if task_ctx.is_interrupted() {
                        log::info!("'{}' stopped", &job_id);
                    } else {
                        log::info!("'{}' finished", &job_id);
                    }
                },
                Err(err) => {
                    log::error!("Job error: {}", err.to_string());
                }
            };
            
            job_ctx.finish();
        });

        Ok(())
    }

    pub fn stop_job(&self, job_id: String) -> Result<()> {
        let jobs = self.jobs.read().unwrap();
        let controller = jobs.get(job_id.as_str()).context("Invalid job_id")?;
        let controller = controller.read().unwrap();
        controller.stop();
        Ok(())
    }

    pub fn get_available_jobs(&self) -> Vec<Box<JobDescription>> {
        let mut result = Vec::new();
        let jobs = self.jobs.read().unwrap();
        for controller in jobs.values() {
            let controller = controller.read().unwrap();
            result.push(controller.factory.get_description());
        }
        result
    }

    pub fn get_job_description(&self, job_id: String) -> Result<Box<JobDescription>> {
        let jobs = self.jobs.read().unwrap();
        let controller = jobs.get(job_id.as_str()).context("Invalid job_id")?;
        let controller = controller.read().unwrap();
        Ok(controller.factory.get_description())
    }

    pub fn get_job_state(&self, job_id: String) -> Result<Box<JobState>> {
        let jobs = self.jobs.read().unwrap();
        let controller = jobs.get(job_id.as_str()).context("Invalid job_id")?;
        let controller = controller.read().unwrap();
        Ok(Box::new(controller.state.clone()))
    }
}

impl ServiceApi for Jobs {

}

impl ServiceInitializer for Jobs {
    fn initialize(context: &AppContext) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let task_manager = context.get_service::<TaskManager>();
        let events_emitter = context.get_service::<EventEmitter>();

        let jobs: Arc<Jobs> = Arc::new(Jobs {
            jobs: RwLock::default(),
            task_manager,
            events_emitter,
        });

        jobs.register_job(DummyJobFactory::new(
            "Dummy job 1", "work_outline", "description")
        );

        jobs.register_job(DummyJobFactory::new(
            "Dummy job 2", "work_outline", "description")
        );

        register_rpc_handler!(rpc, jobs, "lappi.jobs.get_available_jobs", get_available_jobs());
        register_rpc_handler!(rpc, jobs, "lappi.jobs.get_job_description", get_job_description(job_id: String));
        register_rpc_handler!(rpc, jobs, "lappi.jobs.get_job_state", get_job_state(job_id: String));
        register_rpc_handler!(rpc, jobs, "lappi.jobs.start_job", start_job(job_id: String));
        register_rpc_handler!(rpc, jobs, "lappi.jobs.stop_job", stop_job(job_id: String));

        return jobs;
    }
}

