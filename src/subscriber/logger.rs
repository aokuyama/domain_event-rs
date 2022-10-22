use super::{Subscriber, SubscriberLogger};
use crate::event::Event;
use async_trait::async_trait;

impl SubscriberLogger {
    pub fn new() -> Self {
        SubscriberLogger {}
    }
}
#[async_trait]
impl Subscriber for SubscriberLogger {
    fn is_subscribe(&self, _event: &dyn Event) -> bool {
        true
    }
    async fn subscribe(&self, name: &str, body: &str) {
        println!("{}: {}", name, body)
    }
}
