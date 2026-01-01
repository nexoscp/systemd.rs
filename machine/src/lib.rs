use systemd::id128::ID128;
use std::fmt::Debug;
use std::num::ParseIntError;
use std::{fs, io};

// https://www.freedesktop.org/software/systemd/man/machine-info.html#
pub struct Machine {
    id: ID128,
}

impl Machine {
    //https://www.freedesktop.org/software/systemd/man/machine-id.html
    pub fn read() -> Result<Machine, Error> {
        let id = fs::read_to_string("/etc/machine-id")?;
        let m = id.trim();
        Ok(Machine { id: ID128::from(u128::from_str_radix(m, 16)? ) })
    }
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ParseInt(ParseIntError),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParseInt(value)
    }
}
