use dchp_client::dhcp;
use std::net::UdpSocket;

fn main() {
    // Setup sockets
    let wsock = UdpSocket::bind("255.255.255.255:67").expect("Cannot bind to outbound UDP socket");
    let rsock = UdpSocket::bind("255.255.255.255:68").expect("Cannot bind to inbound UDP socket");

    // Send DISCOVERY message

    // Wait for OFFER message

    // Send REQUEST message

    // Wait for ACK message
}
