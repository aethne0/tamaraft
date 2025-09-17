use std::sync::Arc;

use crate::core::transport::{RaftMessage, Transport, TransportError, TransportResult};

/// Process-local message bus for testing and impl example
///
/// This uses serialization as an example and as a method of cloning
pub struct IntraProcessTransport {
    iptc: Arc<IntraProcessTransportCenter>,
}

impl IntraProcessTransport {
    pub fn new(iptc: &Arc<IntraProcessTransportCenter>) -> Self {
        Self { iptc: iptc.clone() }
    }
}

impl Transport for IntraProcessTransport {
    async fn send_raft(&mut self, msg: &RaftMessage) -> TransportResult<()> {
        match serde_json::to_string(msg) {
            Ok(s) => match self.iptc.sender.send(s) {
                Ok(()) => {
                    return TransportResult::Ok(());
                }

                Err(e) => {
                    return TransportResult::Err(TransportError::SendError(e.to_string()));
                }
            },
            Err(e) => {
                return TransportResult::Err(TransportError::SerializationError(e.to_string()));
            }
        }
    }

    async fn recv_raft(&mut self) -> TransportResult<Option<RaftMessage>> {
        todo!()
    }
}

pub struct IntraProcessTransportCenter {
    sender: tokio::sync::mpsc::UnboundedSender<String>,
    receiver: tokio::sync::mpsc::UnboundedReceiver<String>,
}

impl IntraProcessTransportCenter {
    #[must_use]
    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        Self { sender, receiver }
    }
}

// TODO
// the message should be serialized then wrapped with a {to: addr} field
