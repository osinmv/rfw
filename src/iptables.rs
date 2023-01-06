use exitcode;
use std::io;
use std::process::Command;
const IPTABLES: &str = "iptables";
//const IPTABLES_SAVE: &str = "iptables-save";
//const IPTABLES_RESTORE: &str = "iptables-restore";

pub fn run(arguments: Vec<&str>) -> Result<(), io::Error> {
    let output = Command::new(&IPTABLES).args(arguments.iter()).output();
    match output {
        // TODO refactor using if let
        // messy code, but should work for now
        Ok(out) => {
            if exitcode::is_error(out.status.code().unwrap_or(exitcode::OSERR)) {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    String::from_utf8(out.stderr)
                        .unwrap_or("internal error".to_string())
                        .as_str(),
                ));
            }
            return Ok(());
        }
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "iptables is not found",
                ))
            }
            _ => return Err(e),
        },
    }
}
/*
pub fn save_iptables_rules() -> Result<(), io::Error> {
    run_cmd(&IPTABLES_SAVE.to_string(), &"".to_string())?;
    return Ok(());
}

pub fn restore_iptables_rules() -> Result<(), io::Error> {
    run_cmd(&IPTABLES_RESTORE.to_string(), &"".to_string())?;
    return Ok(());
}
 */
