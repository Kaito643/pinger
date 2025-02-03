use std::env;
use std::net::Ipv4Addr;
use std::process;

use libc::{self, in_addr, sockaddr_in, AF_INET, IPPROTO_ICMP, SOCK_RAW};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <IPv4_ADDRESS>", args[0]);
        process::exit(1);
    }

    let ip: Ipv4Addr = args[1].parse().expect("Invalid IPv4 address");

    // Build raw socket
    let socket = unsafe { libc::socket(AF_INET, SOCK_RAW, IPPROTO_ICMP) };
    if socket == -1 {
        eprintln!("Failed to create socket (try running with sudo)");
        process::exit(1);
    }

    // Build ICMP packet
    let mut packet = [0u8; 8];
    packet[0] = 8;  // ICMP type (Echo Request)
    packet[1] = 0;  // Code
    let checksum = compute_checksum(&packet);
    packet[2..4].copy_from_slice(&checksum.to_be_bytes());

    // Build dst.address
    let dest = sockaddr_in {
        sin_len: std::mem::size_of::<sockaddr_in>() as u8,
        sin_family: AF_INET as u8,
        sin_port: 0u16.to_be(),
        sin_addr: in_addr {
            s_addr: u32::from(ip).to_be(),
        },
        sin_zero: [0; 8],
    };

    // Send packet
    let result = unsafe {
        libc::sendto(
            socket,
            packet.as_ptr() as *const libc::c_void,
            packet.len(),
            0,
            &dest as *const sockaddr_in as *const libc::sockaddr,
            std::mem::size_of::<sockaddr_in>() as libc::socklen_t,
        )
    };

    if result == -1 {
        eprintln!("Failed to send packet");
        unsafe { libc::close(socket) };
        process::exit(1);
    } else {
        println!("Sent ICMP Echo Request to {}", ip);
    }

    unsafe { libc::close(socket) };
}

fn compute_checksum(data: &[u8]) -> u16 {
    let mut sum = 0u32;
    let mut i = 0;
    
    while i < data.len() {
        let word = if i + 1 < data.len() {
            ((data[i] as u16) << 8) | data[i + 1] as u16
        } else {
            (data[i] as u16) << 8
        };
        sum += word as u32;
        i += 2;
    }

    while (sum >> 16) != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }

    !sum as u16
}