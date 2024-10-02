# Wake-on-LAN (WoL) Server

I want to be able to turn on my PC from external network. but it is exteremly annoying because:
* WOL doesn't work from external network. I have to either use VPN or send the WOL packet from another device from my internal network.
* I am using L2TP VPN on my Tp-Link router because it is easy to setup, and no need to install a client app. However WOL packet doesn't work on L2TP VPN.

So, I have to do 3 steps and use SSH command to complete the task.
1. connect to L2TP VPN.
2. SSH to my device in my home network.
3. Use some tool to send WOL packet to my desktop from the SSH terminal.

So I wrote this simple program to listen http request over the internal IP address. Once the request received, the program will send the WOL packet to my target desktop.
Now I can simply connect VPN and send http request to a specific url from the browser.

I use it on Raspberry pi. If you want to use it on a different platform, you could build it for your own target.

## Usage

### Prerequisites

- Rust installed on your system

### Build the project
The target for Raspberry Pi 64bit is: aarch64-unknown-linux-gnu
```
cargo build --release --target [Your target]
```

### Running the Server
```
./WolOnPi --ip <IP_ADDRESS> --port <PORT>
```
e.g.
```
./WolOnPi --ip 192.168.0.33 --port 333
```

### Sending a Wake-on-LAN Request
```
http://<IP_ADDRESS>:<PORT>/wol/<TARGET_IP>/<TARGET_MAC>
```
e.g.
```
http://192.168.0.33:333/wol/192.168.0.255/EE:EE:EE:EE:EE:EE
```

You can send it to the specific IP of the machine as a Unicast WOL. You can also send it to a broadcast IP as a Broadcast WOL.
