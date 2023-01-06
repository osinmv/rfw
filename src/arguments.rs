use std::io;
use std::net::Ipv4Addr;
use std::str::FromStr;

pub fn validate(args: &Vec<String>) -> Result<(), io::Error> {
    if args.len() != 3 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid number of parameters",
        ));
    }
    if args[1] != "unban" && args[1] != "ban" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid command parameter",
        ));
    }
    match Ipv4Addr::from_str(&args[2]) {
        // on success do nothing
        Ok(_) => return Ok(()),
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid ip address",
            ));
        }
    };
}
