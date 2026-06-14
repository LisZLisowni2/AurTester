use pcap::{Device, Capture};
use etherparse::PacketHeaders;

pub fn run_sniffer(container_ip: &str) -> Result<(), Box<dyn std::error::Error>> {
    let device_name = "docker0";

    println!("[-] Sniffer: Connecting to '{}'...", device_name);

    let mut cap = Capture::from_device(device_name)?
        .promisc(true)
        .snaplen(65535)
        .immediate_mode(true)
        .open()?;

    let bpf_filter = format!("src host {}", container_ip);
    cap.filter(&bpf_filter, true)?;
    println!("[+] Sniffer: Activated BPF filter: '{}'", bpf_filter);

    while let Ok(packet) = cap.next_packet() {
        if let Ok(value) = PacketHeaders::from_ethernet_slice(packet.data) {
            if let Some(ip_header) = value.net {
                if let etherparse::NetHeaders::Ipv4(ipv4, _) = ip_header {
                    let dest_ip = std::net::Ipv4Addr::from(ipv4.destination);

                    let dest_port = match value.transport {
                        Some(etherparse::TransportHeader::Tcp(tcp)) => tcp.destination_port,
                        Some(etherparse::TransportHeader::Udp(udp)) => udp.destination_port,
                        _ => 0,
                    };

                    println!(
                        "[NETWORK ALERT] Container trying to connect to: {}:{}",
                        dest_ip, dest_port
                    );

                    if dest_port == 4444 {
                        println!("[!!!] DETECTED ATTEMPT OF HACKING! DANGER!");
                    }
                }
            }
        }
    }

    Ok(())
}