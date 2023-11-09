use my_canister::{add_message, update_message, delete_message, get_message, MessagePayload, MessageError};
//Make sure to replace my_canister with the appropriate name of your project.
// Run the tests using the command cargo test in the terminal at the root of your project directory. This will execute the test functions and report the results.
fn main() {
    test_add_message();
    test_update_message();
    test_delete_message();
    test_retrieve_message();
    test_message_not_found();
}

fn test_add_message() {
    // Add your add_message test here.
    // Example:
    let message_payload = MessagePayload {
        title: "Test Title".to_string(),
        body: "Test Body".to_string(),
        attachment_url: "https://test-url.com".to_string(),
    };

    let added_message = add_message(message_payload).unwrap();
    assert_eq!(added_message.title, "Test Title");
}

fn test_update_message() {
    // Add your update_message test here.
    // Example:
    let message_id_to_update = 123; // ID of the message you want to update.

    let updated_payload = MessagePayload {
        title: "Updated Title".to_string(),
        body: "Updated Body".to_string(),
        attachment_url: "https://updated-url.com".to_string(),
    };

    let updated_message = update_message(message_id_to_update, updated_payload).unwrap();
    assert_eq!(updated_message.title, "Updated Title");
}

fn test_delete_message() {
    // Add your delete_message test here.
    // Example:
    let message_id_to_delete = 123; // ID of the message you want to delete.

    let deleted_message = delete_message(message_id_to_delete).unwrap();
    assert_eq!(deleted_message.id, 123);
}

fn test_retrieve_message() {
    // Add your get_message test here.
    // Example:
    let message_id_to_retrieve = 123; // ID of the message you want to retrieve.

    let retrieved_message = get_message(message_id_to_retrieve).unwrap();
    assert_eq!(retrieved_message.id, 123);
}

fn test_message_not_found() {
    // Add your message not found test here.
    // Example:
    let non_existent_message_id = 999; // Non-existent message ID.

    let result = get_message(non_existent_message_id);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        MessageError::NotFound {
            msg: "Message with id=999 not found.".to_string()
        }
    );
}
