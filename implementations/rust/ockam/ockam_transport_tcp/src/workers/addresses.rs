use crate::workers::ConnectionRole;
use ockam_core::Address;

#[derive(Clone, Debug)]
pub(crate) struct Addresses {
    /// Sender internal address to receive messages from the Receiver (about the connection drop)
    sender_internal_address: Address,
    /// Used to receive messages from other workers which are then serialized and sent over the wire
    sender_address: Address,
    /// Receiver Processor Address
    receiver_address: Address,
    /// Receiver Processor Internal Address (to send messages to the Sender)
    receiver_internal_address: Address,
}

impl Addresses {
    pub(crate) fn generate(role: ConnectionRole) -> Self {
        let role_str = role.str();

        let sender_address = Address::random_tagged(&format!("TcpSendWorker_tx_addr_{}", role_str));
        let sender_internal_address =
            Address::random_tagged(&format!("TcpSendWorker_int_addr_{}", role_str));
        let receiver_address = Address::random_tagged(&format!("TcpRecvProcessor_{}", role_str));
        let receiver_internal_address =
            Address::random_tagged(&format!("TcpRecvProcessor_int_addr_{}", role_str));

        Self {
            sender_address,
            sender_internal_address,
            receiver_address,
            receiver_internal_address,
        }
    }
    pub fn sender_internal_address(&self) -> &Address {
        &self.sender_internal_address
    }
    pub fn sender_address(&self) -> &Address {
        &self.sender_address
    }
    pub fn receiver_address(&self) -> &Address {
        &self.receiver_address
    }
    pub fn receiver_internal_address(&self) -> &Address {
        &self.receiver_internal_address
    }
}
