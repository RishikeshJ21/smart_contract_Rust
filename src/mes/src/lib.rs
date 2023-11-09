use candid::{CandidType, Decode, Encode};
use ic_cdk::api::time;
use std::collections::HashMap;

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
    /// Create a new `Message` instance with the given parameters.
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

/// In-memory storage structure to manage messages.
struct MessageStorage {
    messages: HashMap<u64, Message>,
}

impl MessageStorage {
    /// Create a new instance of `MessageStorage`.
    fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }

    /// Add a new message to the storage.
    fn add_message(&mut self, message: Message) -> Option<Message> {
        self.messages.insert(message.id, message.clone());
        Some(message)
    }

    /// Update an existing message.
    fn update_message(
        &mut self,
        id: u64,
        payload: MessagePayload,
    ) -> Result<Message, MessageError> {
        if let Some(message) = self.messages.get_mut(&id) {
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

    /// Delete a message from the storage.
    fn delete_message(&mut self, id: u64) -> Result<Message, MessageError> {
        if let Some(message) = self.messages.remove(&id) {
            Ok(message)
        } else {
            Err(MessageError::NotFound {
                msg: format!("Message with id={} not found.", id),
            })
        }
    }

    /// Retrieve a message by its ID.
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

/// Payload struct for creating or updating a message.
#[derive(CandidType, Serialize, Deserialize)]
struct MessagePayload {
    title: String,
    body: String,
    attachment_url: String,
}

/// Custom error enum for message-related errors.
#[derive(CandidType, Deserialize, Serialize)]
enum MessageError {
    NotFound { msg: String },
}

// Instantiate the message storage globally.
thread_local! {
    static STORAGE: MessageStorage = MessageStorage::new();
}

// Methods accessible to the canister

/// Add a new message.
#[update]
fn add_message(message: MessagePayload) -> Option<Message> {
    let id = time(); // Using time as an ID for simplicity.
    let new_message = Message::new(id, message.title, message.body, message.attachment_url);
    STORAGE.with(|storage| storage.add_message(new_message))
}

/// Update an existing message.
#[update]
fn update_message(id: u64, payload: MessagePayload) -> Result<Message, MessageError> {
    STORAGE.with(|storage| storage.update_message(id, payload))
}

/// Delete a message by ID.
#[update]
fn delete_message(id: u64) -> Result<Message, MessageError> {
    STORAGE.with(|storage| storage.delete_message(id))
}

/// Retrieve a message by ID.
#[query]
fn get_message(id: u64) -> Result<Message, MessageError> {
    STORAGE.with(|storage| storage.get_message(id)).map(|m| m.clone())
}

// Expose the candid interface.
ic_cdk::export_candid!();
