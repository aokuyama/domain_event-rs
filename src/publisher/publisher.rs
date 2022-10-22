use super::Publisher;
use crate::{event::Event, subscriber::Subscriber};

impl Publisher {
    pub fn new() -> Self {
        Publisher {
            subscribers: Vec::new(),
        }
    }
    pub fn register<T: Subscriber + 'static>(&mut self, subscriber: T) {
        let b = Box::<T>::new(subscriber);
        self.subscribers.push(b);
    }
    pub async fn publish(&self, event: &dyn Event) {
        for s in &self.subscribers {
            if s.is_subscribe(event) {
                s.subscribe(event.name(), &event.body()).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{event::LogEvent, publisher::Publisher, subscriber::SubscriberLogger};
    #[tokio::test]
    async fn it_registered() {
        let mut publisher = Publisher::new();
        let logger = SubscriberLogger {};
        publisher.register(logger);
        let e = LogEvent::new("ok");
        publisher.publish(&e).await;
    }
}
