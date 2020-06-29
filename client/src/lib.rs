//! # Naia Client
//! A cross-platform client that can send/receive events to/from a server, and has a pool of in-scope entities that are synced with the server.

#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
)]

extern crate log;

pub use naia_shared::{find_my_ip_address, Config, EntityType};

mod client_connection_state;
mod client_entity_manager;
mod client_entity_message;
mod client_event;
mod error;
mod naia_client;
mod server_connection;

pub use {
    //client_entity_manager::ClientEntityManager, client_entity_message::ClientEntityMessage,
    client_event::ClientEvent, naia_client::NaiaClient, naia_client_socket::Packet,
};
