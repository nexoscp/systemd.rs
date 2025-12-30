/*
    https://www.freedesktop.org/software/systemd/man/os-release.html#
*/

use std::path::Path;

struct OSRelease {
    name: String,
    version: String,
    id: String,
}

impl OSRelease {
    //TODO use Rust-Result
    fn read() -> OSRelease {
        let pathEtc = Path::new("/etc/os-releas");
        let pathLib = Path::new("/usr/lib/os-release");
        /*  let mut f = File::open("/etc/os-releas")?;
      ///usr/lib/os-release
      let mut s = String::new();
      f.read_to_string(&mut s)?;*/
        OSRelease { name: "linux".to_string(), version: "".to_string(), id: "".to_string() }
    }
}
