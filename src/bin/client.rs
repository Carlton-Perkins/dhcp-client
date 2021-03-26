use dchp_client::dhcp::{DHCPOption, DHCPPacket, Serialize, TransactionToken};
use mac_address::MacAddress;
use rand::random;
use std::{
    net::{Ipv4Addr, UdpSocket},
    time::Duration,
};

fn main() {
    // Setup sockets
    let wsock = UdpSocket::bind("0.0.0.0:67").expect("Cannot bind to outbound UDP socket");
    wsock
        .set_broadcast(true)
        .expect("Unable to set outbound socket broadcast state");
    wsock
        .connect("255.255.255.255:67")
        .expect("Cannot connect to UDB broadcast");
    let rsock = UdpSocket::bind("0.0.0.0:68").expect("Cannot bind to inbound UDP socket");

    // Send DISCOVERY message
    let transaction_token: TransactionToken = random();
    let discovery_packet = DHCPPacket::new()
        .with_transaction(&transaction_token)
        .with_mac_address(&MacAddress::new([0x10, 0x7b, 0x44, 0x93, 0xe6, 0xd0]))
        .with_option(DHCPOption::new(53, vec![0x1]))
        .with_option(DHCPOption::new(
            50,
            Ipv4Addr::new(192, 168, 1, 99).octets().to_vec(),
        ))
        .with_option(DHCPOption::new(55, vec![1, 3, 15, 6]));
    wsock
        .send(&discovery_packet.serialize())
        .expect("Send discovery packet failed");

    // Wait for OFFER message
    rsock.set_read_timeout(Some(Duration::new(10, 0))).unwrap();
    loop {
        let mut r_buffer = [0x0; 1024];
        rsock
            .recv(&mut r_buffer)
            .expect("No OFFER message received");
        println!("{:?}", r_buffer);
    }
    // Send REQUEST message

    // Wait for ACK message
}
