use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]

// Structure of messages from new connections (either new publishers or subscribers).
pub struct ConnectionRequest {
    pub method: String,
    pub topic: String,
}
