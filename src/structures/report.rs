use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]

// Message sent from publisher to topic and then resent to subscribers.
pub struct Report {
    pub message: String,
}
