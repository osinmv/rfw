## RFW
It is the simplest firewall frontend for iptables.

There are only 2 features: **ban** and **unban**

## Usage:

`rfw ban 192.168.1.2` - to ban a particular ip address. All packets FROM and TO it will be **dropped**

`rfw unban 192.168.1.2` - to unban a particular ip address. All packets FROM and TO it will receive as usual

## Installation
**cargo** is required for building it from sources
 
 - Download the sources
 - Run `cargo build --release`
 - rfw binary will be in `target/release` folder

## Currently Under development
