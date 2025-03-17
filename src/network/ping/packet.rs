
use pnet::packet::icmp::echo_reply::EchoReplyPacket;
/*
  0                   1                   2                   3  
  0 1 2 3 4 5 6 7 8 9 A B C D E F 0 1 2 3 4 5 6 7 8 9 A B C D E F  
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  
|     Type      |     Code      |          Checksum             |  
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  
|       Identifier (ID)         |      Sequence Number          |  
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  
|                       Payload (Optional)                      |  
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
*/
use pnet::packet::icmp::{echo_request::MutableEchoRequestPacket, IcmpTypes, IcmpType, IcmpPacket, Icmp, MutableIcmpPacket, checksum as calc_checksum};
use pnet::transport::{icmp_packet_iter, IcmpTransportChannelIterator};
use pnet::transport::{transport_channel, TransportChannelType::Layer3, TransportSender, TransportReceiver};
use pnet::packet::icmp::IcmpCode;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::{Ipv4Packet, MutableIpv4Packet};
use pnet::packet::{MutablePacket, Packet};
use std::net::{IpAddr, Ipv4Addr};
use std::time::{Duration, SystemTime};
use std::thread;

// ICMP header size
const ICMP_PACKET_SIZE : usize = 8;
// IPv4 header size
const IPV4_HEADER_SIZE: usize = 20;

pub struct ICMP {
    pub packet: MutableIcmpPacket<'static>,
    pub transport : (TransportSender, TransportReceiver)
}

impl ICMP {
    pub fn new() -> ICMP {
        let mut buffer = vec![0u8; 64]; // 8 bytes for ICMP header + 56 bytes payload (adjust as needed)
        let mut packet = MutableIcmpPacket::owned(buffer).expect("");

        packet.set_icmp_type(IcmpTypes::EchoRequest);
        packet.set_icmp_code(IcmpCode(0));
        packet.set_payload(&vec![0;0]);
        
        let checksum = calc_checksum(&packet.to_immutable());
        
        packet.set_checksum(checksum);

        let (mut sender, mut receiver) = transport_channel(1024, Layer3(IpNextHeaderProtocols::Icmp))
        .expect("");

        ICMP {
            packet,
            transport : (sender, receiver),

        }
    }

    pub fn send(&mut self, destination : IpAddr) -> Result<usize, String>{
        
        println!("Sending ICMP packet to: {:?}", destination);
        // Send the packet
        let resp = &self.transport.0
            .send_to(&self.packet, destination)
            .map_err(|e| format!("Failed to send ICMP packet: {:?}", e))?;
        Ok(resp.to_owned())

    }

    pub fn listen_for_response(&mut self) -> Result<usize, String> {
        let mut buffer = [0u8; 1024]; // Large enough buffer for receiving packets


        loop {

            let mut icmp_iterator = icmp_packet_iter(&mut self.transport.1);
            match icmp_iterator.next() {
                Ok(packet_data) => {
                    // Check if the packet is an IPv4 packet (you can also check for IPv6 here if needed)
                    
                    let ip_packet = Ipv4Packet::new(packet_data.0.packet());
                    
                    match ip_packet {
                        Some(ip_packet) => {
                                match packet_data.0.get_icmp_type() {
                                    IcmpTypes::EchoReply => {
                                        if let Some(echo_reply) = EchoReplyPacket::new(packet_data.0.packet()) {
                                            println!(
                                                "Received ICMP Reply from {}: identifier={} sequence={}",
                                                packet_data.1,
                                                echo_reply.get_identifier(),
                                                echo_reply.get_sequence_number()
                                            );
                                            return Ok(0); // Success
                                        }
                                    }
                                    IcmpTypes::DestinationUnreachable => {
                                        println!("Received ICMP Destination Unreachable from {}", packet_data.1);
                                        return Err("Destination Unreachable".to_string());
                                    }

                                    IcmpTypes::RouterAdvertisement => {
                                        println!("Received ICMP Router Advertisement from {}", packet_data.1);
                                    }
                                    _ => {

                                        
                                        println!("Address: {:?}", packet_data.1);
                                        println!("Received unexpected ICMP type: {:?}", packet_data.0.get_icmp_type());
                                    }

                                }
                            
                        }
                            None => {
                                // It's not an IPv4 packet, so ignore
                                println!("Received non-IPv4 packet, ignoring.");
                            }
                    }
                }
                Err(e) => {
                    return Err(format!("Failed to receive ICMP packet: {:?}", e));
                }
            }
            // Avoid busy waiting, small sleep
            thread::sleep(Duration::from_millis(50));
        }
    }

}

