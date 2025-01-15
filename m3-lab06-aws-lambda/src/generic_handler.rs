use lambda_runtime::{Error, LambdaEvent};
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct IncomingMessage {
    total: String,
    tip: String,
}

#[derive(Serialize)]
pub(crate) struct OutgoingMessage {
    req_id: String,
    tip_amount: String,
    total_plus_tip: String,
}

pub(crate) async fn function_handler(
    event: LambdaEvent<IncomingMessage>,
) -> Result<OutgoingMessage, Error> {
    let total = Decimal::from_str(&event.payload.total).unwrap();
    let tip_percent = Decimal::from_str(&event.payload.tip_percent).unwrap();

    let tip_amount = (total * tip_percent).round_dp(2);
    let total_plus_tip = (total + tip_amount).round_dp(2);

    let resp = OutgoingMessage {
        req_id: event.context.request_id,
        tip_amount: tip_amount.to_string(),
        total_plus_tip: total_plus_tip.to_string(),
    };

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::{Context, LambdaEvent};

    #[tokio::test]
    async fn test_generic_handler() {
        let event = LambdaEvent::new(
            IncomingMessage {
                total: "100.0".to_string(),
                tip: "0.18".to_string(),
            },
            Context::default(),
        );
        let response = function_handler(event).await.unwrap();
        assert_eq!(response.total_plus_tip, "118.00");
    }
}
