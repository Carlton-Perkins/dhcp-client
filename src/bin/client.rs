use dchp_client::dhcp::{
    Deserialize, DhcpMessageType, DhcpOption, DhcpOptionType::*, DhcpPacket, Serialize,
    TransactionToken,
};
use log::{error, info, warn};
use mac_address::MacAddress;
use rand::random;
use simple_logger::SimpleLogger;
use std::{
    net::{Ipv4Addr, UdpSocket},
    time::Duration,
};

fn main() -> Result<(), usize> {
    // Setup logging
    SimpleLogger::new().init().unwrap();

    // Setup sockets
    // ? Should this use the ANY socket? This may pick the wrong interface
    info!("Setting up sockets...");
    let wsock = UdpSocket::bind("0.0.0.0:67").expect("Cannot bind to outbound UDP socket");
    wsock
        .set_broadcast(true)
        .expect("Unable to set outbound socket broadcast state");
    wsock
        .connect("255.255.255.255:67")
        .expect("Cannot connect to UDB broadcast");
    let rsock = UdpSocket::bind("0.0.0.0:68").expect("Cannot bind to inbound UDP socket");
    rsock
        .set_read_timeout(Some(Duration::from_secs(10)))
        .unwrap();

    // Send DISCOVERY message
    let transaction_token: TransactionToken = random();
    info!(
        "Session transaction token {:#x?}",
        u32::from_be_bytes(transaction_token)
    );
    let requested_ip = Ipv4Addr::new(192, 168, 1, 99);
    info!("Requesting Ip {}", requested_ip);
    let mac_address = MacAddress::new([0x10, 0x7b, 0x44, 0x93, 0xe6, 0xd0]); // Desktop
                                                                             // let mac_address = MacAddress::new([0x52, 0x54, 0x00, 0x39, 0x8d, 0xc0]); // VM
    info!("Using MacAddress {}", mac_address);

    let discovery_packet = DhcpPacket::new()
        .with_transaction(&transaction_token)
        .with_mac_address(&mac_address)
        .with_option(DhcpOption::new(
            MessageType as u8,
            vec![DhcpMessageType::Discover as u8],
        ))
        .with_option(DhcpOption::new(
            RequestedIp as u8,
            requested_ip.octets().to_vec(),
        ));

    info!("Sending DHCPDISCOVERY packet");
    wsock
        .send(&discovery_packet.serialize())
        .expect("Send discovery packet failed");

    // Wait for OFFER message
    let offer_packet = loop {
        info!("Waiting for response...");
        let mut rbuffer = [0x0; 1024];
        let rsize = rsock.recv(&mut rbuffer).expect("No OFFER message received");
        // ! TODO if rsize == rbuffer.capacity, we have lost data
        let rbuffer_sliced = &rbuffer[0..rsize];
        let rpacket = DhcpPacket::deserialize(rbuffer_sliced).expect("OFFER packet not parseable");
        let is_correct_packet =
            rpacket.is_type(DhcpMessageType::Offer) && rpacket.is_transaction(&transaction_token);
        if is_correct_packet {
            info!("Received the DHCPOFFER packet");
            break rpacket;
        }
    };

    // Process OFFER message
    let offered_ip = offer_packet.get_client_ip();
    let offered_lease_time = offer_packet.get_lease_time().unwrap();
    let dhcp_server_ip = offer_packet.get_server_ip().unwrap();
    info!(
        "DHCP Server {} offered ip {} with a lease of {}s",
        dhcp_server_ip,
        offered_ip,
        offered_lease_time.as_secs()
    );

    // ? TODO Do we care if the offered_ip is NOT the one we requested ?
    if requested_ip != offered_ip {
        warn!(
            "DHCP server offered ip {} but we requested {}",
            offered_ip, requested_ip
        );
        // ! TODO We could change our requested_ip into the offered_ip to continue, but the user should choose that
        return Err(1);
    }
    // ? TODO Do we care if the lease is less then requested ?

    // Send REQUEST message
    let request_packet = DhcpPacket::new()
        .with_transaction(&transaction_token)
        .with_mac_address(&mac_address)
        .with_option(DhcpOption::new(
            RequestedIp as u8,
            requested_ip.octets().to_vec(),
        ))
        .with_option(DhcpOption::new(
            MessageType as u8,
            vec![DhcpMessageType::Request as u8],
        ))
        .with_option(DhcpOption::new(
            DhcpServerIp as u8,
            dhcp_server_ip.octets().to_vec(),
        ));

    info!("Sending DHCPOFFER packet");
    wsock
        .send(&request_packet.serialize())
        .expect("Failed to send offer packet");

    // Wait for ACK/NAK message
    let ack_packet = loop {
        info!("Waiting for response...");
        let mut rbuffer = [0x0; 1024];
        let rsize = rsock
            .recv(&mut rbuffer)
            .expect("No ACK/NAK message received");
        // ! TODO if rsize == rbuffer.capacity, we have lost data
        let rbuffer_sliced = &rbuffer[0..rsize];
        // println!("rbuffer size: {} \t rbuffer {:?}", rsize, rbuffer_sliced);
        let rpacket =
            DhcpPacket::deserialize(rbuffer_sliced).expect("ACK/NAK packet not parseable");
        // println!("Received Packet: {:?}", rpacket);
        let sender = rpacket.get_server_ip();
        let from_correct_sender = match sender {
            Some(addr) => addr == dhcp_server_ip,
            None => false,
        };
        let is_correct_packet = (rpacket.is_type(DhcpMessageType::Ack)
            || rpacket.is_type(DhcpMessageType::Nak))
            && rpacket.is_transaction(&transaction_token)
            && from_correct_sender;
        // ! TODO Add check to make sure that the ACK/NAK is from the server we sent the request too
        if is_correct_packet {
            info!("Received the ACK/NAK packet");
            break rpacket;
        }
    };

    // Process ACK/NAK message
    let is_ack_message = ack_packet.get_type().unwrap() == DhcpMessageType::Ack;
    let offered_ip = ack_packet.get_client_ip();
    let offered_lease_time = ack_packet.get_lease_time().unwrap();
    let dhcp_server_ip = ack_packet.get_server_ip().unwrap();
    if is_ack_message {
        info!(
            "DHCP Server {} accepted the assigned ip {} with a lease of {}s",
            dhcp_server_ip,
            offered_ip,
            offered_lease_time.as_secs()
        );
    } else {
        error!(
            "DHCP Server {} refused assigned ip {} with a lease of {}s",
            dhcp_server_ip,
            offered_ip,
            offered_lease_time.as_secs()
        );
    }
    Ok(())
}
