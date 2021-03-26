pub mod packet;
pub use packet::{DHCPOption, DHCPPacket, TransactionToken};

pub mod traits;
pub use traits::{Deserialize, Serialize};
