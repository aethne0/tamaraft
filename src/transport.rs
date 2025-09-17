use std::sync::Arc;

use crate::messages::RaftMessage;

#[derive(Debug)]
pub enum TransportError {
    Unspecified,
    DeserializationError(String),
    SerializationError(String),
    SendError(String),
}
pub type TransportResult<T> = Result<T, TransportError>;

/// Abstract trait for message transport, to be implemented by user constructing a RaftNode
///
/// Node owns RaftMessages, Transport only borrows and creates them.
///
/// The transport implementation decides on the serialization method of RaftMessage as well.
pub trait Transport {
    #[must_use]
    fn new(iptc: &Arc<IntraProcessTransportCenter>) -> Self;
    fn send_raft(&mut self, msg: &RaftMessage) -> impl Future<Output = TransportResult<()>> + Send;
    fn recv_raft(&mut self) -> impl Future<Output = TransportResult<Option<RaftMessage>>> + Send;
}

// For testing / example

/// Process-local message bus for testing and impl example
///
/// This uses serialization as an example and as a method of cloning
pub struct IntraProcessTransport {
    iptc: Arc<IntraProcessTransportCenter>,
}

impl Transport for IntraProcessTransport {
    fn new(iptc: &Arc<IntraProcessTransportCenter>) -> Self {
        Self { iptc: iptc.clone() }
    }

    async fn send_raft(&mut self, msg: &RaftMessage) -> TransportResult<()> {
        match serde_json::to_value(msg) {
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
    sender: tokio::sync::mpsc::UnboundedSender<serde_json::Value>,
    receiver: tokio::sync::mpsc::UnboundedReceiver<serde_json::Value>,
}

impl IntraProcessTransportCenter {
    #[must_use]
    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        Self { sender, receiver }
    }
}
