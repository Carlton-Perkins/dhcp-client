use std::convert::TryInto;

use mac_address::MacAddress;

use crate::dhcp::traits::{Deserialize, Serialize};

pub type TransactionToken = [u8; 4];

#[derive(Eq, PartialEq, Debug)]
pub struct DhcpPacket {
    op: u8,
    htype: u8,
    hlen: u8,
    hops: u8,
    xid: [u8; 4],
    secs: [u8; 2],
    flags: [u8; 2],
    ciaddr: [u8; 4],
    yiaddr: [u8; 4],
    siaddr: [u8; 4],
    giaddr: [u8; 4],
    chaddr: [u8; 208],
    cookie: [u8; 4],
    options: Vec<DhcpOption>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct DhcpOption {
    id: u8,
    body: Vec<u8>,
}

impl DhcpPacket {
    pub fn new() -> Self {
        DhcpPacket {
            op: 0x01,
            htype: 0x01,
            hlen: 0x06,
            hops: 0x00,
            xid: [0x00; 4],
            secs: [0x00; 2],
            flags: [0x80, 0x00],
            ciaddr: [0x00; 4],
            yiaddr: [0x00; 4],
            siaddr: [0x00; 4],
            giaddr: [0x00; 4],
            chaddr: [0x00; 208],
            cookie: [0x63, 0x82, 0x53, 0x63],
            options: vec![],
        }
    }

    pub fn with_transaction(mut self, token: &TransactionToken) -> Self {
        self.xid = *token;
        self
    }

    pub fn with_option(mut self, option: DhcpOption) -> Self {
        self.options.push(option);
        self
    }

    pub fn with_mac_address(mut self, maddr: &MacAddress) -> Self {
        self.chaddr = maddr
            .bytes()
            .iter()
            .chain([0; 202].iter())
            .cloned()
            .collect::<Vec<_>>()
            .try_into()
            .expect("Length of chaddr is unexpected");
        self
    }
}
impl Serialize for DhcpPacket {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.push(self.op);
        buffer.push(self.htype);
        buffer.push(self.hlen);
        buffer.push(self.hops);
        buffer.extend_from_slice(&self.xid);
        buffer.extend_from_slice(&self.secs);
        buffer.extend_from_slice(&self.flags);
        buffer.extend_from_slice(&self.ciaddr);
        buffer.extend_from_slice(&self.yiaddr);
        buffer.extend_from_slice(&self.siaddr);
        buffer.extend_from_slice(&self.giaddr);
        buffer.extend_from_slice(&self.chaddr);
        buffer.extend_from_slice(&self.cookie);

        self.options
            .iter()
            .for_each(|x| buffer.extend_from_slice(&x.serialize()));
        // Options list needs to finish with the END option
        buffer.push(0xff);

        // Align buffer to 32 bytes
        let buffer_byte_len = buffer.len();
        (0..buffer_byte_len % 32).for_each(|_| buffer.push(0x00));

        buffer
    }
}

impl Deserialize for DhcpPacket {
    fn deserialize(data: &Vec<u8>) -> Option<Self> {
        todo!()
    }
}

impl DhcpOption {
    pub fn new(id: u8, body: Vec<u8>) -> Self {
        DhcpOption { id, body }
    }
}

impl Serialize for DhcpOption {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.push(self.id);
        buffer.push(self.body.len().try_into().unwrap());
        buffer.extend_from_slice(&self.body);
        buffer
    }
}

impl Deserialize for DhcpOption {
    fn deserialize(data: &Vec<u8>) -> Option<Self> {
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
            DhcpOption {
                id: 53,
                body: vec![0x01]
            }
            .serialize(),
            vec![0x35, 0x01, 0x01]
        ); // Type message
        assert_eq!(
            DhcpOption {
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
            DhcpOption {
                id: 53,
                body: vec![0x01]
            },
            DhcpOption::deserialize(&vec![0x35, 0x01, 0x01]).unwrap()
        ); // Type message
        assert_eq!(
            DhcpOption {
                id: 61,
                body: vec![0x01, 0x10, 0x7b, 0x44, 0x93, 0xe6, 0xd0]
            },
            DhcpOption::deserialize(&vec![0x3d, 0x07, 0x01, 0x10, 0x7b, 0x44, 0x93, 0xe6, 0xd0])
                .unwrap(),
        ); // Client identifer
    }
    #[test]
    fn test_serialize_packet() {
        assert_eq!(
            DhcpPacket::new().serialize().len(), // TODO Using u8 len instead of real value for now
            12
        )
    }

    #[test]
    fn test_deserialize_packet() {
        let packet = DhcpPacket::new();

        assert_eq!(
            DhcpPacket::deserialize(&packet.serialize()).unwrap(),
            packet
        );
    }
}
