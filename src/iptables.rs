use std::io;
use std::process::Command;
const IPTABLES: &str = "iptables";
//const IPTABLES_SAVE: &str = "iptables-save";
//const IPTABLES_RESTORE: &str = "iptables-restore";

pub fn run(arguments: &String) -> Result<(), io::Error> {
    let output = Command::new(&IPTABLES).arg(arguments).output();
    match output {
        Ok(_) => return Ok(()),
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
