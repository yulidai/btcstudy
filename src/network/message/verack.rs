use std::convert::Into;
use crate::network::NetworkEnvelope;

pub struct VerackMessage {}

impl VerackMessage {
    pub fn command() -> [u8; 12] {
        let mut result = [0u8; 12];
        for (i, byte) in b"verack".iter().enumerate() {
            result[i] = *byte;
        }

        result
    }
}

impl Into<NetworkEnvelope> for Verack {
    fn into(self) -> NetworkEnvelope {
        let command = Self::command();
        let payload = vec![];

        NetworkEnvelope::new(command, payload)
    }
}

#[cfg(test)]
mod tests {
    use super::VerackMessage;

    #[test]
    fn verack_message_command() {
        assert_eq!(hex::encode(VerackMessage::command()), "76657261636b000000000000");
    }
}
