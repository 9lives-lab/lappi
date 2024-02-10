use serde::Serialize;
use amina_core::events::EventEmitter;
use amina_core::service::Service;

use crate::ui::utils::ui_data::UiDataBinding;

#[derive(Serialize)]
pub struct SongItem {
    pub id: i32,
    pub text: String,
}

#[derive(Serialize, Default)]
struct UiData {
    items: Vec<SongItem>,
}

pub struct SongsListBinding {
    ui_binding: UiDataBinding<UiData>,
}

impl SongsListBinding {

    pub fn create(events_emitter: Service<EventEmitter>, key: &str) -> Self {
        Self {
            ui_binding: UiDataBinding::create(events_emitter, key),
        }
    }

    pub fn update(&self, items: Vec<SongItem>) {
        self.ui_binding.update(UiData {
            items
        })
    }

}

