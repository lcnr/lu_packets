








use endio::{Deserialize, Serialize};
use lu_packets_derive::MessageFromVariants;

#[derive(Debug, Deserialize, Serialize, PartialEq, MessageFromVariants)]
#[non_exhaustive]
#[repr(u8)]
pub enum Message {
	Noop,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, MessageFromVariants)]
#[repr(u16)]
pub enum UserMessage {
	Noop,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, MessageFromVariants)]
#[non_exhaustive]
#[post_disc_padding = 1]
#[repr(u32)]
pub enum AnyClientMessage {
	Noop,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, MessageFromVariants)]
#[non_exhaustive]
#[post_disc_padding = 9]
#[repr(u32)]
pub enum AnyChatMessage {
	Noop,
}
