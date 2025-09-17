use std::time::Duration;

use crate::storage::{PersistentState, Storage};

pub struct NodeConfig {
    pub id: u64,
    /// NOT including nodeself
    pub peers: Vec<u64>,

    /// Election timeout for a node will be +/- 35% of this
    pub election_timeout: Duration,
    pub heartbeat_timeout: Duration,
}

pub struct RaftNode<S> {
    config: NodeConfig,
    persistent: PersistentState,
    storage: S,
}

impl<S: Storage> RaftNode<S> {
    pub async fn new(config: NodeConfig, storage: S) -> Self {
        let loaded_state = storage
            .load_state()
            .await
            .expect("Loading initial state failed");

        Self {
            config,
            storage,
            persistent: loaded_state.unwrap_or_default(),
        }
    }
}
