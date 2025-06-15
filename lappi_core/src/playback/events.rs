use serde::{Serialize, Deserialize};
use amina_core::events::Event;

use crate::collection::pictures::PictureId;

#[derive(Serialize, Deserialize)]
pub struct OnStateUpdated<'a> {
    pub current_player_name: &'a str,
    pub title: &'a str,
    pub cover_picture: Option<PictureId>,
    pub is_playing: bool,
    pub is_next_available: bool,
    pub is_previous_available: bool,
    pub progress: i32,
}

impl Event for OnStateUpdated<'_> {
    fn get_key() -> &'static str {
        "lappi.playback.OnStateUpdated"
    }
}

impl Default for OnStateUpdated<'_> {
    fn default() -> Self {
        Self {
            current_player_name: "",
            title: "",
            cover_picture: Option::None,
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

