use serde::{Deserialize, Serialize};
mod base;
mod log;
pub trait Event {
    fn name(&self) -> &'static str;
    fn body(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Base {
    body: String,
}

pub struct LogEvent {
    base: Base,
}
