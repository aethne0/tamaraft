use serde::{Deserialize, Serialize};

use crate::persist::Storage;

pub struct NodeConfig {
    pub id: u64,
    pub peers: Vec<u64>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PersistentState {
    pub current_term: u64,
    pub voted_for: Option<u64>,
    pub log: u64, // todo
}

pub struct RaftNode<S> {
    config: NodeConfig,
    persistent: PersistentState,
    storage: S,
}

impl<S> RaftNode<S>
where
    S: Storage,
{
    pub fn new(config: NodeConfig, storage: S) -> Self {
        Self {
            config,
            storage,
            persistent: PersistentState {
                current_term: 1,
                voted_for: None,
                log: 0,
            },
        }
    }

    pub fn client_cmd(&mut self, cmd: &[u8]) {
        todo!()
    }
}
