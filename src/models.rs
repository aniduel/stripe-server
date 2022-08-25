use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct StripePayload {
    pub user_id: u64,
}

#[derive(Serialize)]
pub struct PaymentBotMessage {
    pub user_id: u64,
}
