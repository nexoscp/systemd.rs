// https://www.freedesktop.org/software/systemd/man/sd_notify.html
// https://www.freedesktop.org/software/systemd/man/systemd-notify.html
use std::os::unix::net::UnixDatagram;
use std::net::Shutdown;
use std::env;
use std::io::Error;
use std::env::VarError;
use crate::Service;

pub enum NotifyError {
    Enviroment(VarError),
    Socket(Error),
}

impl NotifyError {
    fn new(&self) -> Result<NotifySocket, NotifyError> {
        match env::var("NOTIFY_SOCKET") { //Default is /run/systemd/journal/socket
            Ok(socketName) => {
                match UnixDatagram::bind(socketName) {
                    Ok(socket) => Ok(NotifySocket { socket }),
                    Err(error) => Err(NotifyError::Socket(error))
                }
            },
            Err(e) => Err(NotifyError::Enviroment(e)),
        }
    }
}

pub struct NotifySocket {
    socket: UnixDatagram,
}

pub trait NotifyService : Service  {
    //fn send<I>(&self, entries: I) where I: Iterator<Item=NotifyEntry>;
    //TODO make a iterable of entries or so
    fn send(&self, entries: NotifyEntry);
}

impl Service for NotifySocket {}

// http://stackoverflow.com/questions/32947606/calling-a-sd-notify0-watchdog-1-in-a-service
impl NotifyService for NotifySocket {
    //TODO add varagr or iterable to allow more than one message per package
    fn send(&self, entries: NotifyEntry) {
        //for e in entries {
            let buffer = entries.to_string();
        //}
        self.socket.send(buffer.as_bytes());
    }
}

impl Drop for NotifySocket {
    fn drop(&mut self) {
        self.socket.shutdown(Shutdown::Both);
    }
}

pub enum NotifyEntry {
    READY,
    RELOADING,
    STOPPING,
    STATUS(String),
    ERRNO(u32),
    BUSERROR(String),
    MAINPID(u32),//TODO also env?
    WATCHDOG,
    FDSTORE,
    FDNAME(String),
    WATCHDOG_USEC(u32),//TODO env
}

impl ToString for NotifyEntry {
    fn to_string(&self) -> String {
        let buffer = match &self {
            NotifyEntry::READY => "READY=1\n".to_string(),
            NotifyEntry::RELOADING => "RELOADING=1\n".to_string(),
            NotifyEntry::STOPPING => "STOPPING=1\n".to_string(),
            NotifyEntry::STATUS(s) => format!("STATUS={}\n", &s),
            NotifyEntry::ERRNO(e) => format!("ERRNO={}\n", &e.to_string()),
            NotifyEntry::BUSERROR(b) => format!("BUSERROR={}\n", &b),
            //NotifyEntry::MAINPID(m) => "MAINPID=",
            NotifyEntry::WATCHDOG => "WATCHDOG=1\n".to_string(),
            //NotifyEntry::FDSTORE => "FDSTORE=1",
            //NotifyEntry::FDNAME(n) => "FDNAME=",
            //NotifyEntry::WATCHDOG_USEC(u) => "WATCHDOG_USEC=", //TODO is env
            NotifyEntry::MAINPID(_) => "".to_string(),
            NotifyEntry::FDSTORE => "".to_string(),
            NotifyEntry::FDNAME(_) => "".to_string(),
            NotifyEntry::WATCHDOG_USEC(_) => "".to_string()
        };
        return buffer;
    }
}