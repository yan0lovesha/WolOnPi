use clap::Parser;
use std::net::{UdpSocket, Ipv4Addr};
use warp::Filter;


#[derive(Parser)]
struct Args {
    /// The tenant id that host Traffic Manager Profile instance.
    #[clap(long)]
    ip: String,

    /// The Azure subscription id that hosts your resources.
    #[clap(long)]
    port: u16,
}

fn validate_mac(mac: &str) -> Result<Vec<u8>, &'static str> {
    if mac.len() != 17 || mac.matches(':').count() != 5 {
        return Err("Invalid MAC address format");
    }

    let mac_bytes = mac.split(':')
        .map(|s| u8::from_str_radix(s, 16).unwrap())
        .collect::<Vec<u8>>();

    Ok(mac_bytes)
}

fn build_magic_packet(mac: &str) -> Result<Vec<u8>, &'static str> {
    let mac_bytes = validate_mac(mac)?;
    let mut packet = vec![0xFF; 6];
    for _ in 0..16 {
        packet.extend(&mac_bytes);
    }
    Ok(packet)
}

fn send_wol_packet(ip: &str, mac: &str) -> Result<(), &'static str> {
    let ip_addr: Ipv4Addr = ip.parse().map_err(|_| "Invalid IP address format")?;
    let packet = build_magic_packet(mac)?;

    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|_| "Failed to bind to socket")?;
    let broadcast_ip = format!("{}:9", ip_addr);
    socket.set_broadcast(true).map_err(|_| "Failed to set broadcast")?;
    socket.send_to(&packet, broadcast_ip).map_err(|_| "Failed to send packet")?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let ip_to_listen: std::net::IpAddr = args.ip.parse().expect("Invalid IP address");
    let port_to_listen = args.port;

    let wol_route = warp::path!("wol" / String / String)
        .map(|ip: String, mac: String| {
            match send_wol_packet(&ip, &mac) {
                Ok(_) => warp::reply::json(&format!("packet for mac {} sent to {}", mac, ip)),
                Err(e) => warp::reply::json(&format!("Error: {}", e)),
            }
        });

    warp::serve(wol_route)
        .run((ip_to_listen, port_to_listen))
        .await;
}