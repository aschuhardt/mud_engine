//remote_client.rs

use std::net::SocketAddr;

use uuid::Uuid;

#[derive(PartialEq, Eq, Clone)]
pub struct RemoteClient {
    pub user_id: Uuid,
    pub addr: SocketAddr,
    pub user_name: String,
}
