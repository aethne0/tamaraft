use std::time::Duration;

use crate::{
    storage::{PersistentState, Storage},
    transport::Transport,
};

pub struct NodeConfig {
    pub id: u64,
    /// NOT including nodeself
    pub peers: Vec<u64>,

    /// Election timeout for a node will be +/- 35% of this
    pub election_timeout: Duration,
    pub heartbeat_timeout: Duration,
}

pub struct RaftNode<S, T> {
    config: NodeConfig,
    persistent: PersistentState,
    storage: S,
    transport: T,
}

impl<S: Storage, T: Transport> RaftNode<S, T> {
    pub async fn new(config: NodeConfig, storage: S, transport: T) -> Self {
        let loaded_state = storage
            .load_state()
            .await
            .expect("Loading initial state failed");

        Self {
            config,
            persistent: loaded_state.unwrap_or_default(),

            storage,
            transport,
        }
    }
}
