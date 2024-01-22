//! Server-received general messages.
use endio::{Deserialize, Serialize};
use lu_packets_derive::VariantTests;

#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[post_disc_padding = 1]
#[repr(u32)]
pub enum GeneralMessage {
	Noop,
}

/**
	Provides the client's network version.

	Allows to identify outdated clients and disconnect them early.

	### Trigger
	Establishment of raknet connection (receipt of [`ConnectionRequestAccepted`](crate::raknet::client::ConnectionRequestAccepted)).

	### Handling
	Check if [`network_version`](Self::network_version) matches the version you expect. Otherwise, disconnect the client, ideally with a [`DisconnectNotify::InvalidGameVersion`](super::client::DisconnectNotify::WrongGameVersion) specifying the expected version.

	### Response
	Respond with a server-sent [`Handshake`](super::client::Handshake) providing the server's network version and service ID.

	### Notes
	This packet should not be seen as proof that the client's network version is actually what they report it to be. The client can provide any value, and malicious clients can deviate from the protocol in any way they like. Therefore, proper length and value checking is still required for packet parsing, and care should be taken that your server does not crash on invalid input. If you're using the parsing functionality of this library, this will be taken care of for you.
*/
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[trailing_padding = 33]
pub struct Handshake {}
