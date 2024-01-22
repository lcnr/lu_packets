//! Client-received general messages.
use endio::{Deserialize, Serialize};
use lu_packets_derive::VariantTests;


/// Client-received general messages.
#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[post_disc_padding = 1]
#[repr(u32)]
pub enum GeneralMessage {
	Noop,
}

/**
	Completes a version handshake initiated by the client.

	### Trigger
	Receipt of a client-sent [`Handshake`](super::server::Handshake) packet that was acceptable to the server.

	### Handling
	Optionally check if the server's [`network_version`](Self::network_version) matches your own. You can usually assume that the server will check this itself and disconnect if it doesn't match, but you can check again to be sure.

	### Response
	If the server's [`service_id`](Self::service_id) is [`ServiceId::Auth`], respond with a [`LoginRequest`](crate::auth::server::LoginRequest) with your username and password. If it is [`ServiceId::World`], send a [`ClientValidation`](crate::world::server::ClientValidation) with your username and the session key provided by auth.

	### Notes
	As the version confirm process was designed with more than just client-server in mind, it sends the server's network version and service id as well, even though this isn't really needed by the client (even the service id isn't needed, since you usually only connect to auth once, and it's the very first connection). This could be simplified if the protocol is ever revised.
*/
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[trailing_padding = 41]
pub struct Handshake {
	/// The network protocol version of the server. For servers compatible with live, this is `171022`. This was relevant mainly back when LU was actively updated. Server projects making modifications to the network protocol should set this to a different value.
	pub network_version: u32,
}

/**
	Notifies the client when it was actively disconnected by the server.

	### Trigger
	Being disconnected by the server, the exact trigger depends on the variant.

	### Handling
	Display a message to the user.

	### Response
	None. Expect the connection to be closed shortly after, so a response won't even be possible.

	### Notes
	You can be disconnected without receiving this packet, for example when your connection is lost. The server is also not obligated to send this packet and may disconnect you without doing so.
*/
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u32)]
pub enum DisconnectNotify {
	Noop,
}
