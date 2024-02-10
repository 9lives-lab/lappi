use std::sync::{RwLock, Arc};
use std::ops::Deref;

use serde::{Serialize, Deserialize};
use amina_core::service::Service;
use amina_core::events::EventEmitter;

pub struct UiDataBinding<T: Default + Serialize + Send + Sync + 'static> {
    events_emitter: Service<EventEmitter>,
    data: Arc<RwLock<T>>,
    to_ui_key: String,
}

#[derive(Deserialize)]
struct UpdateReq {

}

impl <T: Default + Serialize + Send + Sync + 'static> UiDataBinding<T> {

    pub fn create(events_emitter: Service<EventEmitter>, key: &str) -> Self {
        let data: T = Default::default();
        let data = Arc::new(RwLock::new(data));
        let from_ui_key = "from_ui.".to_owned() + key + ".update_request";
        let to_ui_key = "to_ui.".to_owned() + key + ".update";

        let binding = UiDataBinding {
            events_emitter: events_emitter.clone(),
            data: data.clone(),
            to_ui_key: to_ui_key.clone(),
        };
        events_emitter.clone().on_generic_event_fn(from_ui_key.as_str(),  move |_: &UpdateReq| {
            events_emitter.emit(to_ui_key.as_str(), data.read().unwrap().deref());
        });
        return binding;
    }

    pub fn update(&self, ui_data: T) {
        let mut ui_data_ref = self.data.write().unwrap();
        *ui_data_ref = ui_data;
        self.events_emitter.emit(self.to_ui_key.as_str(), ui_data_ref.deref());
    }

}

pub struct UiDataDynBinding<T: Serialize + Send + Sync + 'static> {
    events_emitter: Service<EventEmitter>,
    supplier_fn: Arc<dyn (Fn() -> T) + Sync + Send + 'static>,
    to_ui_key: String,
}

impl <T: Serialize + Send + Sync + 'static> UiDataDynBinding<T> {

    pub fn create<F>(events_emitter: Service<EventEmitter>, key: &str, supplier: F) -> Self where
            F: Fn() -> T + Send + Sync + 'static
    {
        let from_ui_key = "from_ui.".to_owned() + key + ".update_request";
        let to_ui_key = "to_ui.".to_owned() + key + ".update";
        let supplier_fn = Arc::new(supplier);

        let binding = UiDataDynBinding {
            events_emitter: events_emitter.clone(),
            supplier_fn: supplier_fn.clone(),
            to_ui_key: to_ui_key.clone(),
        };
        events_emitter.clone().on_generic_event_fn(from_ui_key.as_str(), move |_: &UpdateReq| {
            let ui_data = supplier_fn();
            events_emitter.emit(to_ui_key.as_str(), &ui_data);
        });
        return binding;
    }

    pub fn update(&self) {
        let supplier_fn = self.supplier_fn.deref();
        let ui_data = supplier_fn();
        self.events_emitter.emit(self.to_ui_key.as_str(), &ui_data);
    }

}
