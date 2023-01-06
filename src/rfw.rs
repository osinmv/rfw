use crate::iptables;
use std::io;

pub fn run(action: &String, ip_address: &String) -> Result<(), io::Error> {
    if action == "unban" {
        unban(ip_address)?;
    } else {
        ban(ip_address)?;
    }
    return Ok(());
}
fn unban(ip_address: &String) -> Result<(), io::Error> {
    iptables::run(&format!("-D INPUT -s {} -j DROP", ip_address))?;
    iptables::run(&format!("-D OUTPUT -s {} -j DROP", ip_address))?;
    return Ok(());
}
fn ban(ip_address: &String) -> Result<(), io::Error> {
    iptables::run(&format!("-A INPUT -s {} -j DROP", ip_address))?;
    iptables::run(&format!("-A OUTPUT -s {} -j DROP", ip_address))?;
    return Ok(());
}
