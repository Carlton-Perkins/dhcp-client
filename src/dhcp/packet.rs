use crate::dhcp::traits::{Deserialize, Serialize};

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

impl Serialize for DHCPPacket {
    fn serialize(&self) -> Vec<u8> {
        todo!()
    }
}

impl Deserialize for DHCPPacket {
    fn deserialize(data: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Serialize for DHCPOption {
    fn serialize(&self) -> Vec<u8> {
        todo!()
    }
}

impl Deserialize for DHCPOption {
    fn deserialize(data: &Vec<u8>) -> Self {
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
        // Vec<u8> should deserialize to options

        assert_eq!(
            DHCPOption {
                id: 53,
                body: vec![0x01]
            },
            DHCPOption::deserialize(&vec![0x35, 0x01, 0x01])
        ); // Type message
        assert_eq!(
            DHCPOption {
                id: 61,
                body: vec![0x01, 0x10, 0x7b, 0x44, 0x93, 0xe6, 0xd0]
            },
            DHCPOption::deserialize(&vec![0x3d, 0x07, 0x01, 0x10, 0x7b, 0x44, 0x93, 0xe6, 0xd0]),
        ); // Client identifer
    }
    #[test]
    fn test_serialize_packet() {
        assert_eq!(
            DHCPPacket {
                op: 0x01,
                htype: 0x01,
                hlen: 0x06,
                hops: 0x00,
                xid: 0x00000000,
                secs: 0x0000,
                flags: 0x0000,
                ciaddr: 0x00000000,
                yiaddr: 0x00000000,
                siaddr: 0x00000000,
                giaddr: 0x00000000,
                chaddr: [0x00000000; 6],
                options: vec![],
            }
            .serialize()
            .len(), // TODO Using u8 len instead of real value for now
            12
        )
    }

    #[test]
    fn test_deserialize_packet() {
        let packet = DHCPPacket {
            op: 0x01,
            htype: 0x01,
            hlen: 0x06,
            hops: 0x00,
            xid: 0x00000000,
            secs: 0x0000,
            flags: 0x0000,
            ciaddr: 0x00000000,
            yiaddr: 0x00000000,
            siaddr: 0x00000000,
            giaddr: 0x00000000,
            chaddr: [0x00000000; 6],
            options: vec![],
        };

        assert_eq!(DHCPPacket::deserialize(&packet.serialize()), packet);
    }
}
