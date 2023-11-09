use candid::CandidType;
use ic_cdk::api::time;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use ic_cdk::{update, query};

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

    fn add_message(&mut self, message: Message) -> Option<Message> {
        self.messages.insert(message.id, message.clone());
        Some(message)
    }

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

    fn delete_message(&mut self, id: u64) -> Result<Message, MessageError> {
        if let Some(message) = self.messages.remove(&id) {
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
}

thread_local! {
    static STORAGE: std::cell::RefCell<MessageStorage> = std::cell::RefCell::new(MessageStorage::new());
}

#[update]
fn add_message(message: MessagePayload) -> Option<Message> {
    let id = time();
    let new_message = Message::new(id, message.title, message.body, message.attachment_url);
    STORAGE.with(| storage| {
        let mut storage = storage.borrow_mut();
        storage.add_message(new_message)
    })
}

#[update]
fn update_message(id: u64, payload: MessagePayload) -> Result<Message, MessageError> {
    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        storage.update_message(id, payload)
    })
}

#[update]
fn delete_message(id: u64) -> Result<Message, MessageError> {
    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        storage.delete_message(id)
    })
}

#[query]
fn get_message(id: u64) -> Result<Message, MessageError> {
    STORAGE.with(|storage| {
        let storage = storage.borrow();
        storage.get_message(id).map(|m| m.clone())
    })
}

ic_cdk::export_candid!();
