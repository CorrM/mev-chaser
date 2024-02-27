#[derive(Debug, Clone)]
pub struct Notification {
    pub message: String,
}

impl Notification {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
