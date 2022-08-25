use axum::extract::{Extension, Json};

use crate::{
    models::{PaymentBotMessage, StripePayload},
    State, StripeResult,
};

pub async fn webhook_post(
    Extension(state): Extension<State>,
    Json(json): Json<StripePayload>,
) -> StripeResult<()> {
    let tx = state.0;

    let message_model = PaymentBotMessage {
        user_id: json.user_id,
    };
    let message = serde_json::to_string(&message_model)?;
    tx.send(message)?;

    Ok(())
}
