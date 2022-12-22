use std::env;
use std::error::Error;
use std::process::Command;

fn run_iptables(arguments: &String) -> (Vec<u8>, i32) {
    let output = Command::new("iptables")
        .arg(arguments)
        .output()
        .expect("iptables should be installed");
    let exit_code = output
        .status
        .code()
        .expect("Process should have exit status code");
    if output.status.success() {
        return (output.stdout.to_ascii_lowercase(), exit_code);
    }
    return (output.stderr.to_ascii_lowercase(), exit_code);
}

fn validate_arguments(args: &Vec<String>) {
    
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[1]);
    let argumets = "-L".to_string();
    let cmd_result = run_iptables(&argumets);
    let out_string = String::from_utf8(cmd_result.0).expect("Could not read iptable output");
    println!("{out_string}");
}
