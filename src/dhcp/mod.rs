pub mod packet;
pub use packet::{DhcpOption, DhcpPacket, TransactionToken};

pub mod traits;
pub use traits::{Deserialize, Serialize};
