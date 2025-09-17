use std::{sync::Arc, time::Duration};

use tokio::sync::mpsc;

use crate::core::{
    storage::{PersistentState, Storage},
    transport::Transport,
};

pub struct NodeConfig {
    pub id: u64,
    /// including itself (node 1 would store `[1,2,3]`)
    pub peers: Vec<u64>,

    pub base_election_timeout: Duration,
    pub heartbeat_timeout: Duration,
}

#[derive(Debug)]
pub enum RaftEvent {
    HeartbeatTimeout,
    ElectionTimeout,
    AppendEntriesHandle,
    RequestVoteHandle,
    ShutdownSignal,
}

pub struct RaftNode<S, T> {
    config: NodeConfig,
    persistent: PersistentState,

    event_tx: mpsc::Sender<RaftEvent>,
    event_rx: mpsc::Receiver<RaftEvent>,

    storage: S,
    transport: Arc<T>,
}

impl<S: Storage, T: Transport> RaftNode<S, T> {
    pub async fn new(config: NodeConfig, storage: S, transport: T) -> Self {
        let loaded_state = storage
            .load_state()
            .await
            .expect("Loading initial state failed");

        let (event_tx, event_rx) = mpsc::channel(64); // TODO: max messages

        Self {
            config,
            persistent: loaded_state.unwrap_or_default(),

            event_tx,
            event_rx,

            storage,
            transport: Arc::new(transport),
        }
    }

    /// This initializes the transport listener and begins the node's event loop
    pub async fn run(&mut self) {
        tracing::trace!("Node:{} starting...", self.config.id);

        // Event handler loop
        loop {
            tokio::select! {
                Some(event) = self.event_rx.recv() => {
                    self.handle_event(event).await;
                }

                Some(rpc) = self.transport.recv_rpc(self.config.id)=> {
                    match rpc {
                        crate::core::transport::RaftRpcReq::AppendEntriesReq => {},
                        crate::core::transport::RaftRpcReq::RequestVoteReq => {},
                    }
                }

                else => break
            }
        }

        tracing::trace!("Node:{} stopped.", self.config.id);
    }

    /// Return value is whether or not node should continue running
    async fn handle_event(&mut self, event: RaftEvent) -> bool {
        tracing::trace!("Node:{} handling {:?}", self.config.id, &event);

        match event {
            RaftEvent::ElectionTimeout => {}
            RaftEvent::HeartbeatTimeout => {}
            RaftEvent::AppendEntriesHandle => {}
            RaftEvent::RequestVoteHandle => {}
            RaftEvent::ShutdownSignal => return false,
        }

        true // should continue
    }
}
