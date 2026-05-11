use crate::error::MessageIntegrityError;
use crate::message::Message;

pub enum ParsedMessage {
    Valid(Message),
    Invalid {
        message: Message,
        reason: InvalidReason,
    },
    Garbled(GarbledReason),
    UnexpectedError(String),
}

impl ParsedMessage {
    pub fn into_message(self) -> Option<Message> {
        match self {
            ParsedMessage::Valid(message) => Some(message),
            ParsedMessage::Invalid { message, .. } => Some(message),
            _ => None,
        }
    }
}

pub enum InvalidReason {
    InvalidField(u32),
    InvalidGroup(u32),
    InvalidOrderInGroup {
        tag: u32,
        group_tag: u32,
    },
    InvalidComponent(String),
    InvalidMsgType(String),
    /// A required field was missing from the message body or from a repeating-group entry.
    /// `group_tag` is `Some` when the missing field belongs to a repeating group.
    RequiredFieldMissing {
        tag: u32,
        group_tag: Option<u32>,
    },
}

#[derive(Debug)]
pub enum GarbledReason {
    Malformed,
    InvalidBeginString,
    InvalidBodyLength,
    InvalidMsgType,
    InvalidChecksum,
}

impl From<MessageIntegrityError> for ParsedMessage {
    fn from(header_error: MessageIntegrityError) -> Self {
        match header_error {
            MessageIntegrityError::InvalidBeginString => {
                ParsedMessage::Garbled(GarbledReason::InvalidBeginString)
            }
            MessageIntegrityError::InvalidBodyLength => {
                ParsedMessage::Garbled(GarbledReason::InvalidBodyLength)
            }
            MessageIntegrityError::InvalidMsgType => {
                ParsedMessage::Garbled(GarbledReason::InvalidMsgType)
            }
            MessageIntegrityError::InvalidCheckSum => {
                ParsedMessage::Garbled(GarbledReason::InvalidChecksum)
            }
        }
    }
}
