use exitcode;
use std::io;
use std::process::{Command, Output};
const IPTABLES: &str = "iptables";
//const IPTABLES_SAVE: &str = "iptables-save";
//const IPTABLES_RESTORE: &str = "iptables-restore";

fn handle_iptables_errorcode(out: Output) -> Result<(), io::Error> {
    let errorcode = out.status.code().unwrap_or(exitcode::OSERR);
    let error = exitcode::is_error(errorcode);
    if error {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            String::from_utf8(out.stderr)
                .unwrap_or("internal error".to_string())
                .as_str(),
        ));
    }
    return Ok(());
}
fn handle_start_error(err: io::Error) -> Result<(), io::Error> {
    match err.kind() {
        io::ErrorKind::NotFound => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "iptables is not found",
        )),
        _ => return Err(err),
    }
}
pub fn run(arguments: Vec<&str>) -> Result<(), io::Error> {
    let output = Command::new(&IPTABLES).args(arguments.iter()).output();
    match output {
        Ok(out) => handle_iptables_errorcode(out),
        Err(e) => handle_start_error(e),
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
