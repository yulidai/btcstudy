use std::convert::Into;
use crate::network::{Command, NetworkEnvelope};

pub struct VerackMessage {}

impl VerackMessage {
    pub fn new() -> Self {
        Self {}
    }

    pub fn command() -> Command {
        Command::Verack
    }
}

impl Into<NetworkEnvelope> for VerackMessage {
    fn into(self) -> NetworkEnvelope {
        let command = Self::command();
        let payload = vec![];

        NetworkEnvelope::new(command, payload)
    }
}

#[cfg(test)]
mod tests {
    use crate::network::{Command, VerackMessage};

    #[test]
    fn verack_message_command() {
        assert_eq!(VerackMessage::command(), Command::Verack);
    }
}
