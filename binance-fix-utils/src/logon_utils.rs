use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use binance_fix44_order_entry::Logon;
use binance_fix44_order_entry::field_id::{self, USERNAME};
use hmac::{Hmac, Mac};
use quickfix::*;
use sha2::Sha256;

use crate::config::BinanceConfig;

pub fn fill_message(
    msg: &mut Message,
    config: &BinanceConfig,
    sender_comp_id: &str,
) -> Result<(), QuickFixError> {
    // Set sender comp id
    msg.set_field(field_id::SENDER_COMP_ID, sender_comp_id)
        .expect("Fail to set sender comp id");
    // Set target comp id
    msg.set_field(field_id::TARGET_COMP_ID, "SPOT")
        .expect("Fail to set target comp id");
    // Set msg seq num
    msg.set_field(field_id::MSG_SEQ_NUM, "1")
        .expect("Fail to set msg seq num");
    // Set sending time
    // Set password
    msg.set_field(USERNAME, config.api_key.as_str())
        .expect("Fail to set username/API Key");
    Ok(())
}

/// Add signature to a `Logon<A>` message.
///
/// This function is a direct implementation of Binance signature spec.
/// See: <https://developers.binance.com/docs/zh-CN/binance-spot-api-docs/fix-api#logon-main>
pub fn sign(msg: &mut Message, config: &BinanceConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Add few macro to make it easier to work with `Message`.
    macro_rules! read_header {
        ($tag:expr) => {
            msg.with_header(|h| h.get_field($tag))
                .expect("Missing mandatory message header")
        };
    }

    // Build pre-sign message by extracting everything from auto-generated FIX message.
    let pre_sign = [
        Logon::MSG_TYPE_BYTES,
        &read_header!(field_id::SENDER_COMP_ID),
        &read_header!(field_id::TARGET_COMP_ID),
        &read_header!(field_id::MSG_SEQ_NUM),
        &read_header!(field_id::SENDING_TIME),
    ]
    .join("\x01");

    // Generate signature.
    let secret = BASE64.decode(config.sign_pemkey.as_bytes())?;
    let mut mac = Hmac::<Sha256>::new_from_slice(&secret)?;
    mac.update(pre_sign.as_bytes());
    let signature_raw = mac.finalize();
    let signature = BASE64.encode(signature_raw.into_bytes());

    // Append it to outgoing message.
    msg.set_field(field_id::RAW_DATA_LENGTH, signature.len())?;
    msg.set_field(field_id::RAW_DATA, signature)?;

    Ok(())
}
