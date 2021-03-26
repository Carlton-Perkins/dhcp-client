use crate::dhcp::traits::{Serialize, Deserialize};

#[derive(Eq, PartialEq, Debug)]
struct DHCPPacket {
    op: u8,
    htype: u8,
    hlen: u8,
    hops: u8,
    xid: u32,
    secs: u16,
    flags: u16,
    ciaddr: u32,
    yiaddr: u32,
    siaddr: u32,
    giaddr: u32,
    chaddr: [u32; 6],
    options: Vec<DHCPOption>,
}

#[derive(Eq, PartialEq, Debug)]
struct DHCPOption {
    id: u8,
    body: Vec<u8>,
}

impl Serialize for DHCPOption {
    fn serialize(self) -> Vec<u8> {
        todo!()
    }
}

impl Deserialize for DHCPOption {
    fn deserialize(data: Vec<u8>) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod dhcp_packet {
    use super::*;

    #[test]
    fn test_serialize_option() {
        // Options should serialize to <type><length><body>
        // where length is the byte size of the body

        assert_eq!(
            DHCPOption {
                id: 53,
                body: vec![0x01]
            }
            .serialize(),
            vec![0x35, 0x01, 0x01]
        ); // Type message
        assert_eq!(
            DHCPOption {
                id: 61,
                body: vec![0x01, 0x10, 0x7b, 0x44, 0x93, 0xe6, 0xd0]
            }
            .serialize(),
            vec![0x3d, 0x07, 0x01, 0x10, 0x7b, 0x44, 0x93, 0xe6, 0xd0]
        ); // Client identifer
    }

    #[test]
    fn test_deserialize_option() {
        // Vec<u8> should deserialize to <type><length><body>
        // where length is the byte size of the body

        assert_eq!(
            DHCPOption {
                id: 53,
                body: vec![0x01]
            },
            DHCPOption::deserialize(vec![0x35, 0x01, 0x01])
        ); // Type message
        assert_eq!(
            DHCPOption {
                id: 61,
                body: vec![0x01, 0x10, 0x7b, 0x44, 0x93, 0xe6, 0xd0]
            },
            DHCPOption::deserialize(vec![0x3d, 0x07, 0x01, 0x10, 0x7b, 0x44, 0x93, 0xe6, 0xd0]),
        ); // Client identifer
    }
}
