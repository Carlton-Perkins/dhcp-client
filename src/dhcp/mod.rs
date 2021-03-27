pub mod packet;
pub use packet::{DhcpMessageType, DhcpOption, DhcpOptionType, DhcpPacket, TransactionToken};

pub mod traits;
pub use traits::{Deserialize, Serialize};
