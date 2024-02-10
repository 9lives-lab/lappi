use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum PlaybackSource {
    LocalFile(String),
}
