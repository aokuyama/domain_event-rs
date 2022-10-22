use crate::subscriber::Subscriber;
mod publisher;
pub struct Publisher {
    subscribers: Vec<Box<dyn Subscriber>>,
}
