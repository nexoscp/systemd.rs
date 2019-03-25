// pure idiomatic rust replacement of udevlib (see https://github.com/systemd/systemd/tree/master/src/libudev )
// and the under laying library sd-device
// (see https://github.com/systemd/systemd/tree/master/src/libsystemd/sd-device )

pub type Syspath = String;

trait ToSyspath {
    fn to_syspath(&self) -> Result<Syspath, Error>;
}

impl ToSyspath for str {
    fn to_syspath(&self) -> Result<String, Error> {
       create_syspath(&self.into_string())
    }
}

// see https://github.com/systemd/systemd/blob/master/src/libsystemd/sd-device/sd-device.c#L134
pub fn create_syspath(value: &str) -> Result<Syspath, Error> {
    Err(Erro::NotSubdirectoryOfSys(String::from(vslue)))
}

// see https://github.com/systemd/systemd/blob/master/src/libsystemd/sd-device/device-internal.h
pub struct Device {
    syspath: Syspath
}

pub enum Error {
    NotSubdirectoryOfSys(String)
}

impl Device {
}

// entry point is https://github.com/libusb/libusb/blob/master/libusb/os/linux_udev.c
// see https://github.com/systemd/systemd/blob/master/src/libudev/libudev-enumerate.c
pub fn enumerate () -> Result<(), Error> {
    Ok(())
}

// Hier gehts weiter: https://github.com/systemd/systemd/blob/master/src/libsystemd/sd-device/sd-device.c#L232

