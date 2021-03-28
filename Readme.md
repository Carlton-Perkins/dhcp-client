# DHCP Client Implementation

Carlton Perkins

## Goals

- [x] DHCP Client
  - [x] Implement all packet types
    - [x] DHCP DISCOVER
    - [x] DHCP OFFER
    - [x] DHCP REQUEST
    - [x] DHCP ACKNOWLEDGE
  - [x] Progress should be printed to the console
- [ ] Improvements:
  - [ ] Nice CLI for various input arguments
    - [ ] Mac address used
    - [ ] Network interface used
    - [ ] Ip requested
    - [ ] Lease time requested
    - [ ] Renew existing lease
    - [ ] Release existing lease

## Plan

Using design from <https://tools.ietf.org/html/rfc2131>

High level overview from <https://medium.com/@bromiley/full-packet-friday-dhcp-abbc6b7b3c77>

1. Client emits UDP DHCPDISCOVER to local all DHCP servers on the network.
   1. Source Port: 68
   2. Destination Port: 67
   3. Should use Limited broadcast UDP
   4. Contains options 53,61,50
   5. Ends with padding
2. Server will respond with a DHCPOFFER
   1. Message contains an possible IP address with all relevant lease info
   2. Also contains the ip address of the server providing the lease
   3. If not response received, timeout and error
   4. Shares a transaction ID with DISCOVER message
   5. Contains options 53,1,58,59,51,54
   6. Ends with padding
3. Client emits the DHCPREQUEST message to accept the suggested IP address
   1. Gets a new transaction ID in option 50
   2. Contains options 53,61,50,54,55
   3. Ends with padding
4. Server ACK's the REQUEST with a DCHPACKNOWLEDGE
   1. Confirms IP and Lease values
   2. System can now use the IP address
   3. Contains options 53,58,59,51,54,1
   4. Ends with padding

## Notes

- Ports less then 1024 require root to bind too.
