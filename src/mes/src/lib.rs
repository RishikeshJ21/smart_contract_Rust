use candid::{CandidType, Decode, Encode};
use ic_cdk::{api::{time, logging, trap}, IDL};
use std::collections::HashMap;

const MAX_MESSAGE_SIZE: usize = 2048; // Example parameterized memory size

#[derive(CandidType, Clone, Serialize, Deserialize)]
struct Message {
    id: u64,
    title: String,
    body: String,
    attachment_url: String,
    created_at: u64,
    updated_at: Option<u64>,
}

impl Message {
    fn new(id: u64, title: String, body: String, attachment_url: String) -> Self {
        Self {
            id,
            title,
            body,
            attachment_url,
            created_at: time(),
            updated_at: None,
        }
    }
}

struct MessageStorage {
    messages: HashMap<u64, Message>,
}

impl MessageStorage {
    fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }

    fn add_message(&mut self, message: Message) -> Result<Message, MessageError> {
        // Input validation
        if message.title.is_empty() || message.body.is_empty() || message.attachment_url.is_empty() {
            return Err(MessageError::InvalidInput {
                msg: "All fields must be non-empty".to_string(),
            });
        }

        // Check size limitation
        if message.title.len() + message.body.len() + message.attachment_url.len() > MAX_MESSAGE_SIZE {
            return Err(MessageError::SizeExceeded);
        }

        // Logging
        logging::info(&format!("Adding message with ID: {}", message.id));

        // Storage
        self.messages.insert(message.id, message.clone());

        Ok(message)
    }

    fn update_message(&mut self, id: u64, payload: MessagePayload) -> Result<Message, MessageError> {
        if let Some(message) = self.messages.get_mut(&id) {
            // Input validation
            if payload.title.is_empty() || payload.body.is_empty() || payload.attachment_url.is_empty() {
                return Err(MessageError::InvalidInput {
                    msg: "All fields must be non-empty".to_string(),
                });
            }

            // Check size limitation
            if payload.title.len() + payload.body.len() + payload.attachment_url.len() > MAX_MESSAGE_SIZE {
                return Err(MessageError::SizeExceeded);
            }

            // Logging
            logging::info(&format!("Updating message with ID: {}", id));

            // Update
            message.attachment_url = payload.attachment_url;
            message.body = payload.body;
            message.title = payload.title;
            message.updated_at = Some(time());
            Ok(message.clone())
        } else {
            Err(MessageError::NotFound {
                msg: format!("Message with id={} not found.", id),
            })
        }
    }

    fn delete_message(&mut self, id: u64) -> Result<Message, MessageError> {
        if let Some(message) = self.messages.remove(&id) {
            // Logging
            logging::info(&format!("Deleting message with ID: {}", id));
            Ok(message)
        } else {
            Err(MessageError::NotFound {
                msg: format!("Message with id={} not found.", id),
            })
        }
    }

    fn get_message(&self, id: u64) -> Result<&Message, MessageError> {
        if let Some(message) = self.messages.get(&id) {
            Ok(message)
        } else {
            Err(MessageError::NotFound {
                msg: format!("Message with id={} not found.", id),
            })
        }
    }
}

#[derive(CandidType, Serialize, Deserialize)]
struct MessagePayload {
    title: String,
    body: String,
    attachment_url: String,
}

#[derive(CandidType, Deserialize, Serialize)]
enum MessageError {
    NotFound { msg: String },
    InvalidInput { msg: String },
    SizeExceeded,
}

thread_local! {
    static STORAGE: MessageStorage = MessageStorage::new();
}

#[update]
fn add_message(message: MessagePayload) -> Result<Message, MessageError> {
    let id = time(); // Using time as an ID for simplicity.
    let new_message = Message::new(id, message.title, message.body, message.attachment_url);
    STORAGE.with(|storage| storage.add_message(new_message))
}

#[update]
fn update_message(id: u64, payload: MessagePayload) -> Result<Message, MessageError> {
    STORAGE.with(|storage| storage.update_message(id, payload))
}

#[update]
fn delete_message(id: u64) -> Result<Message, MessageError> {
    STORAGE.with(|storage| storage.delete_message(id))
}

#[query]
fn get_message(id: u64) -> Result<Message, MessageError> {
    STORAGE.with(|storage| storage.get_message(id)).map(|m| m.clone())
}

ic_cdk::export_candid!();
