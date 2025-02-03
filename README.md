# Minimal Rust Ping Demo

It is a minimal ICMP ping demonstration tool for CLI written with rust.

## Features
- Sends single ICMP packet
- Supports Ipv4
- No external libaries except libc

## Prerequisites
- Rust & Cargo
- macOS (tested on macOS systems) - currently working on implementing this to linux
- Priveleged user - you should use this tool with either root user or sudo it is required because of raw socket usage

## Installation
### From Source
git clone https://github.com/Kaito643/pinger.git 

cd pinger

### Install Local User Wide 
This part is optional but if you do not do this you need to locate pingers directory everytime

cargo build --release

sudo cp target/release/pinger /usr/local/bin/

## Usage
sudo cargo run -- <IPv4_ADDRESS>
### Example:
sudo cargo run -- 8.8.8.8

### If installed in environment
sudo pinger <IPv4_ADDRESS>
# Example:
sudo pinger 8.8.8.8

## Limitations 
It is a demonstration tool I want to remind you again so maybe in the future I will complete this project to an advanced ping tool
- No response handling
- No DNS Resolution
- No error recovery
- MacOS specific socket implementation
- Checksum is a little buggy right now

## Technical Details & How it works
- I used raw sockets utilizing libc library thats why the tool requires elevated priveleges
- Implemented ICMP Checksum Calculation
- Sent Packets are 8 byte ICMP Echo Packets

I will explain the initial commit in this blog writing if anyone is further interested

umut-yildiz-blogs.notion.site/Pinger-How-it-works-18f2e945179b80beb665c77f45613838?pvs=74

## Troubleshooting
Possible errors are;
- "Permission denied", you can try changing to a privileged user like root or run the command with sudo
- "Invalid IPv4 address", check destination target format should be like this {xxx.xxx.xxx.xxx}
- "Failed to create socket", your firewall or antivirus can be blocking the program to use raw sockets

## Conclusion
Please contact me if I could be of any help or if you have suggestions to improve. I will be developing this project in the next month also 
