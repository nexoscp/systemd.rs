/*
https://www.freedesktop.org/software/systemd/man/sd_booted.html#
*/
use std::path::Path;
use crate::Service;



//TODO impl Debug, Display
#[derive(Debug)]
pub enum Booted {
    SYSTEMD,
    OTHER,
}

pub struct BootedSystem {
    path : *const Path,
}
impl BootedSystem {
    fn new() -> BootedSystem {
        BootedSystem {path: Path::new("/run/systemd/systemd/") }
    }
}

pub trait BootedService : Service {
    fn booted(&self) -> Booted;
}

impl Service for BootedSystem {

}

impl BootedService for BootedSystem {
    fn booted(&self) -> Booted {
        //implemented as advised in https://www.freedesktop.org/software/systemd/man/sd_booted.html
       /* if &self.path.is_dir() { FIXME
            return Booted::SYSTEMD
        } */
        Booted::OTHER
    }
}
