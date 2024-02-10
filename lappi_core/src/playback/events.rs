use serde::{Serialize, Deserialize};
use amina_core::events::Event;

#[derive(Serialize, Deserialize)]
#[derive(Event)]
#[key = "lappi.playback.OnStateUpdated"]
pub struct OnStateUpdated {
    pub title: String,
    pub is_playing: bool,
    pub is_next_available: bool,
    pub is_previous_available: bool,
    pub progress: i32,
}

impl OnStateUpdated {
    pub fn stopped() -> Self {
        Self {
            title: "".to_string(),
            is_playing: false,
            is_next_available: false,
            is_previous_available: false,
            progress: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Event)]
#[key = "lappi.playback.OnProgressUpdated"]
pub struct OnProgressUpdated {
    pub progress: f32,
}

