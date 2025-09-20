use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RaftRpcReq {
    AppendEntriesReq,
    RequestVoteReq,
}

#[derive(Serialize, Deserialize)]
pub enum RaftRpcResp {
    AppendEntriesResp,
    RequestVoteResp,
}

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
pub trait Transport: Send + Sync {
    /// This should work fine
    /// ```
    ///      S --> T     R   S sends REQ to T
    ///      S ... T     R   S now waiting...
    ///      S ... T --> R   T forwards to R
    ///      S ... T ... R   T waits for R's RESP
    ///      S ... T <-- R   R sends RESP, T receives
    ///      S <-- T     R   T returns/forwards RESP to S
    /// ```
    fn send_rpc(
        &self,
        destination_id: u64,
        rpc: RaftRpcReq,
        // to be clear: this does not return until the rpc has been responded to
    ) -> impl Future<Output = TransportResult<RaftRpcResp>> + Send;

    /// todo:
    /// 
    /// This however will have to be solved.
    /// The sender method makes sense but i am not sure what the interface will be from
    /// the view of the receiver node.
    /// It could either just be a message with a "handle" attached, such that the <impl Transport>
    /// receives a resp with that handle (asynchronously) it will return from the `send_rpc` call.
    /// Otherwise the <impl Transport> would need its own task ("""thread""") where it listens and
    /// then call some method of the node.
    /// I am leaning towards the handle option but maybe the second one has some better revision.
    fn recv_rpc(&self, node_id: u64) -> impl Future<Output = Option<RaftRpcReq>> + Send;
}

