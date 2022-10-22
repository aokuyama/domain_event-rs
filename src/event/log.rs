use super::{Base, Event, LogEvent};

impl LogEvent {
    pub fn new(body: &str) -> Self {
        LogEvent {
            base: Base::new(body),
        }
    }
}
impl Event for LogEvent {
    fn body(&self) -> String {
        self.base.body()
    }
    fn name(&self) -> &'static str {
        "LogEvent"
    }
}
