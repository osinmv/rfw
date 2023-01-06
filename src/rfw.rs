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
    // should add a function in iptables which generates these arguments
    let input_table_args = vec!["-D", "INPUT", "-s", &ip_address, "-j", "DROP"];
    let output_table_args = vec!["-D", "OUTPUT", "-d", &ip_address, "-j", "DROP"];
    iptables::run(input_table_args)?;
    iptables::run(output_table_args)?;
    return Ok(());
}
fn ban(ip_address: &String) -> Result<(), io::Error> {
    // should add a function in iptables which generates these arguments
    let input_table_args = vec!["-A", "INPUT", "-s", &ip_address, "-j", "DROP"];
    let output_table_args = vec!["-A", "OUTPUT", "-d", &ip_address, "-j", "DROP"];
    iptables::run(input_table_args)?;
    iptables::run(output_table_args)?;
    return Ok(());
}
