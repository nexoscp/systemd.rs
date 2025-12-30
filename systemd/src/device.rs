// pure idiomatic rust replacement of udevlib (see https://github.com/systemd/systemd/tree/master/src/libudev )
// and the under laying library sd-device
// (see https://github.com/systemd/systemd/tree/master/src/libsystemd/sd-device )

use std::path::Path;

pub struct Syspath {
    path: *const Path
}

impl  From<&Path> for Syspath {
    fn from(path: &Path) -> Syspath{
        Syspath { path }
    }
}

trait ToSyspath {
    fn to_syspath(&self) -> Result<Syspath, Error>;
}

// see https://github.com/systemd/systemd/blob/master/src/libsystemd/sd-device/sd-device.c#L134
pub fn create_syspath(value: &str) -> Result<Syspath, Error> {
    Err(Error::NotSubdirectoryOfSys(String::from(value)))
}

// see https://github.com/systemd/systemd/blob/master/src/libsystemd/sd-device/device-internal.h
pub struct Device {
    syspath: Syspath
}

pub enum Error {
    NotSubdirectoryOfSys(String)
}

// entry point is https://github.com/libusb/libusb/blob/master/libusb/os/linux_udev.c
// see https://github.com/systemd/systemd/blob/master/src/libudev/libudev-enumerate.c
pub fn enumerate() -> Result<(), Error> {
    Ok(())
}

// Hier gehts weiter: https://github.com/systemd/systemd/blob/master/src/libsystemd/sd-device/sd-device.c#L232

struct DevNum {
    major: u8,
    minor: u8,
}

impl DevNum {
    const fn major(&self) -> u8 {
        self.major
    }

    const fn minor(&self) -> u8 {
        self.minor
    }
}

/*
impl <'a> From<DevNum> for Device<'a> {
    fn from(dev_num: DevNum) -> Self {
        let path = format!("{}", dev_num.major());
     //   Device { syspath: Syspath::from(Path::from(path)) } //use From-Trait
    }
}
*/

