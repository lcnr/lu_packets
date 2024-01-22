use endio::{Deserialize, Serialize};
use lu_packets_derive::{VariantTests};

use crate::common::{LuWString33, ObjId};
pub use super::{GeneralChatMessage, PrivateChatMessage};

#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[non_exhaustive]
#[post_disc_padding = 9]
#[repr(u32)]
pub enum ChatMessage {
	GeneralChatMessage(GeneralChatMessage) = 1,
	PrivateChatMessage(PrivateChatMessage) = 2,
	AchievementNotify(AchievementNotify) = 59,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct AchievementNotify {
	#[padding = 5]
	pub sender_name: LuWString33,
	pub sender: ObjId,
	pub source_id: u16,
	pub sender_gm_level: u8,
	pub target_group: u32,        // todo: type?
	pub mission_message_key: u32, // todo: type?
	pub requesting_player: ObjId,
	pub recipient_name: LuWString33,
	pub recipient_gm_level: u8,
}
