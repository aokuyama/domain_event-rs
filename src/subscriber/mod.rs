mod logger;
use crate::event::Event;
use async_trait::async_trait;

#[async_trait]
pub trait Subscriber: Sync {
    fn is_subscribe(&self, event: &dyn Event) -> bool;
    async fn subscribe(&self, name: &str, body: &str);
}
pub struct SubscriberLogger;
