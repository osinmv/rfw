use exitcode;
use std::env;
use std::io;
use std::net::Ipv4Addr;
use std::process::{self, Command};
use std::str::FromStr;
const IPTABLES: &str = "iptables";
const IPTABLES_SAVE: &str = "iptables-save";
const IPTABLES_RESTORE: &str = "iptables-restore";

fn run_cmd(command: &String, arguments: &String) -> Result<process::Output, std::io::Error> {
    Command::new(command).arg(arguments).output()
}

fn save_iptables_rules() -> Result<(), io::Error> {
    run_cmd(&IPTABLES_SAVE.to_string(), &"".to_string())?;
    return Ok(());
}

fn restore_iptables_rules() -> Result<(), io::Error> {
    run_cmd(&IPTABLES_RESTORE.to_string(), &"".to_string())?;
    return Ok(());
}

fn run_iptables(arguments: &String) -> Result<(Vec<u8>, i32), std::io::Error> {
    let output = run_cmd(&IPTABLES.to_string(), &arguments)?;
    let exit_code = output
        .status
        .code()
        .expect("Process must have an exit status code");
    if output.status.success() {
        return Ok((output.stdout.to_ascii_lowercase(), exit_code));
    }
    return Ok((output.stderr.to_ascii_lowercase(), exit_code));
}

fn validate_arguments(args: &Vec<String>) {
    // IT is a very bad idea to exit with process::exit since nothing is cleaned up
    // Should comeup with something better
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

fn perform_action(action: &String, ip_address: &String) -> Result<(), std::io::Error> {
    if action == "ban" {
        run_iptables(&format!("-A INPUT -s {} -j DROP", ip_address))?;
        run_iptables(&format!("-A OUTPUT -d {} -j DROP", ip_address))?;
    } else if action == "unban" {
        run_iptables(&format!("-D INPUT -s {} -j DROP", ip_address))?;
        run_iptables(&format!("-D OUTPUT -d {} -j DROP", ip_address))?;
    }
    save_iptables_rules()?;
    return Ok(());
}

fn main() {
    // the code is so terrible, using 3 types of handling errors
    // exiting process without cleaning up
    // using expect and Result<T,E>
    // break my hands for it
    let args: Vec<String> = env::args().collect();
    validate_arguments(&args);
    let action_result = perform_action(&args[1], &args[2]);
    match action_result {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            restore_iptables_rules().expect("Couldn't restore previous iptables rules");
            eprintln!("Restored previous iptables rules");
        }
    }
}
