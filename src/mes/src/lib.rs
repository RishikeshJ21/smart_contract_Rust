use candid::{CandidType, Principal};
use ic_cdk::api::{time, caller};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use ic_cdk::{update, query};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory}; // Custom memory management structures
use ic_stable_structures::{ Cell, DefaultMemoryImpl}; // Custom data structures
use std::cell::RefCell;



#[derive(CandidType, Clone, Serialize, Deserialize)]
struct Message {
    id: u64,
    owner: Principal,
    title: String,
    body: String,
    attachment_url: String,
    created_at: u64,
    updated_at: Option<u64>,
}

impl Message {
    fn new(id: u64, owner: Principal, title: String, body: String, attachment_url: String) -> Self {
        Self {
            id,
            owner,
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
            assert!(message.owner.to_string() == caller().to_string(), "Not owner of message");
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
        let message = self.messages.get(&id).expect( &format!("Message with id={} not found.", id));
        assert!(message.owner.to_string() == caller().to_string(), "Not owner of message");
        if let Some(message) = self.messages.remove(&id) {
            Ok(message.clone())
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

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );
    static STORAGE: RefCell<MessageStorage> = RefCell::new(MessageStorage::new());

}

// Function to add a message to the canister
#[update]
fn add_message(message: MessagePayload) -> Option<Message> {
    let id = ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");
    let new_message = Message::new(id, caller(),message.title, message.body, message.attachment_url);
    STORAGE.with(| storage| {
        let mut storage = storage.borrow_mut();
        // save message
        storage.add_message(new_message)
    })
}

// Function to update a message to the canister
#[update]
fn update_message(id: u64, payload: MessagePayload) -> Result<Message, MessageError> {
    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        storage.update_message(id, payload)
    })
}

// Function to delete a message to the canister
#[update]
fn delete_message(id: u64) -> Result<Message, MessageError> {
    STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        storage.delete_message(id)
    })
}

// Function to get a message to the canister
#[query]
fn get_message(id: u64) -> Result<Message, MessageError> {
    STORAGE.with(|storage| {
        let storage = storage.borrow();
        storage.get_message(id).map(|m| m.clone())
    })
}

ic_cdk::export_candid!();
