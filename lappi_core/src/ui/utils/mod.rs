pub mod songs_list;
pub mod ui_data;

use std::sync::Arc;

use serde::Serialize;
use amina_core::events::EventEmitter;
use amina_core::service::{Service, ServiceApi, ServiceInitializer, Context};

use crate::ui::utils::ui_data::UiDataBinding;

pub struct UiUtils {
    events_emitter: Service<EventEmitter>,
}

impl UiUtils {

    pub fn create_binding<T>(&self, key: &str) -> UiDataBinding<T> where
        T: Default + Serialize + Send + Sync + 'static
    {
        UiDataBinding::create(self.events_emitter.clone(), key)
    }

}

impl ServiceApi for UiUtils {

}

impl ServiceInitializer for UiUtils {
    fn initialize(context: &Context) -> Arc<Self> {
        Arc::new(UiUtils {
            events_emitter: context.get_service::<EventEmitter>(),
        })
    }
}
