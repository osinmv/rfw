use exitcode;
use std::env;
use std::net::Ipv4Addr;
use std::process::{self, Command};
use std::str::FromStr;
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
    if args.len() != 3 {
        eprintln!("Incorrect number of arguments");
        process::exit(exitcode::DATAERR);
    }
    if args[1] != "unban" && args[1] != "ban" {
        eprintln!("Unknown command {}", args[1]);
        process::exit(exitcode::DATAERR);
    }
    match Ipv4Addr::from_str(&args[2]) {
        // on success do nothing
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            process::exit(exitcode::DATAERR);
        }
    };
}
fn form_arguments(args: &Vec<String>) -> String {
    let ip_address = &args[2];
    let argumets_string: String;
    if args[1] == "unban" {
        argumets_string = "-L".to_string();
    } else {
        // dropping all conections from given IP address
        argumets_string = format!("-A INPUT -s {} -j DROP", ip_address);
    }
    return argumets_string;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    validate_arguments(&args);
    let argumets = form_arguments(&args);
    let cmd_result = run_iptables(&argumets);
    let out_string = String::from_utf8(cmd_result.0).expect("Could not read iptable output");
    println!("{out_string}");
}
