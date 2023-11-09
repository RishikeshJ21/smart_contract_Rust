# smart_contract_Rust
# Message Canister

This is a simple message canister that allows you to perform various operations such as adding, updating, deleting, and retrieving messages. The canister utilizes in-memory storage and provides error handling for managing messages.

## Functionality

The canister provides the following functionality:

- Add a new message
- Update an existing message
- Delete a message by ID
- Retrieve a message by ID

## Usage

The canister exposes the following methods:

### Add a New Message

```rust
let message_payload = MessagePayload {
    title: "Example Title".to_string(),
    body: "Example Body".to_string(),
    attachment_url: "https://example.com".to_string(),
};

let new_message = add_message(message_payload).unwrap();
println!("Added Message: {:?}", new_message);

### Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_message_board_contract/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = "0.5.6"
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid. 
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```
# Command-line Tests for Message Canister

This project provides a set of command-line tests to verify the functionality of the message canister in Rust. The tests cover adding, updating, deleting, and retrieving messages, along with error cases where messages do not exist or other errors occur during execution.

#test Cases

1. Create a new Rust project using Cargo.
2. Add the canister code to the project.
3. Create a `tests` directory in the project root.
4. Inside the `tests` directory, create a file named `integration_test.rs`.
5. Add the provided content from `integration_test.rs` to your test file.

## Running Tests

To run the tests, execute the following command in the terminal at the root of your project directory:

```bash
cargo test

