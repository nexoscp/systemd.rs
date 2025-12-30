pub mod sysloglevel;
mod keyvalue;

use bytebuffer::ByteBuffer;
use env::current_exe;
use keyvalue::KVBuffer;
use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::fmt::Debug;
use std::net::Shutdown;
use std::os::unix::net::UnixDatagram;
use std::{env, io};
use std::io::Error;
use sysloglevel::SysLogLevel;


// https://lists.freedesktop.org/archives/systemd-devel/2012-November/007359.html
// https://github.com/systemd/systemd/blob/master/src/journal/journal-send.c function sd_journal_sendv line 213
// https://systemd.io/JOURNAL_NATIVE_PROTOCOL/
pub struct Journal {
    //used for SYSLOG_IDENTIFIER
    //see also https://linux.die.net/man/3/program_invocation_short_name
    tag: String,
    socket: UnixDatagram,
}

impl Journal {
    pub fn init(level: LevelFilter) -> Result<(), JournalError> {
        log::set_boxed_logger(Box::new(Journal::open()?))
            .map(|()| log::set_max_level(level))?;
        Ok(())
    }

    pub fn init_tag(level: LevelFilter, tag: &str) -> Result<(), JournalError> {
        log::set_boxed_logger(Box::new(Journal::open_tag(tag)?))
            .map(|()| log::set_max_level(level))?;
        Ok(())
    }

    //open socket with AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC "/run/systemd/journal/socket"
    fn open_tag(tag: &str) -> Result<Journal, io::Error> {
        let socket = UnixDatagram::unbound()?;
        socket.connect("/run/systemd/journal/socket")?;
        Ok(Journal { tag: tag.into(), socket })
    }

    fn open() -> Result<Journal, io::Error> {
        match current_exe()?.file_name() {
            Some(file_name) => {
                match file_name.to_str() {
                    Some(name) => { Journal::open_tag(name) }
                    None => { Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid executable file name")) }
                }}
            None => {Err(io::ErrorKind::NotFound.into())}
        }

    }
}

impl Drop for Journal {
    fn drop(&mut self) {
        self.socket.shutdown(Shutdown::Both);
    }
}

impl Log for Journal {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    // https://www.reddit.com/r/rust/comments/48fmta/seven_ways_to_concatenate_strings_in_rust_the/?st=j059k7k4&sh=000b234a
    //TODO from systemd src/basic/util.c implement string_is_safe(..)
    // Fields: https://www.freedesktop.org/software/systemd/man/systemd.journal-fields.html
    //TODO define feature to turn of code location
    //TODO max size https://stackoverflow.com/questions/56913753/maximal-length-of-a-unix-datagram-in-posix
    fn log(&self, record: &Record) {
            let mut buffer = ByteBuffer::new();
            buffer.write_bytes(b"SYSLOG_IDENTIFIER=");
            buffer.write_bytes(self.tag.as_bytes());
            buffer.write_bytes(b"\n");
             match record.file() {
                Some(file) => {
                    write(&mut buffer, b"CODE_FILE", file.as_bytes());
                }
                 None => {}
            }
            match record.line() {
                Some(line) => {
                    write(&mut buffer, b"CODE_LINE", line.to_string().as_bytes());
                }
                None => {}
            }
            write(&mut buffer, b"MESSAGE", record.args().to_string().as_bytes());
            match record.module_path() {
                Some(module_path) => {
                    write(&mut buffer, b"RUST_MODULE_PATH", module_path.as_bytes());
                }
                None => {}
            }

        write(&mut buffer, b"PRIORITY", Into::<char>::into(SysLogLevel::from(record.metadata().level())).to_string().as_bytes());
        write(&mut buffer, b"RUST_TARGET", record.target().as_bytes());

        //TODO only when feature kv is active
         record.key_values().visit(&mut KVBuffer::new(&mut buffer));

         self.socket.send(buffer.as_bytes());
    }

    fn flush(&self) {}
}

fn write(buffer: &mut ByteBuffer, key:&[u8], value: &[u8]) {
    buffer.write_bytes(key);
    buffer.write_bytes(b"=");
    buffer.write_bytes(value);
    buffer.write_bytes(b"\n");
}

#[derive(Debug)]
pub enum JournalError {
    Io(io::Error),
    SetLoggerError(log::SetLoggerError)
}

impl From<io::Error> for JournalError {
    fn from(e: Error) -> Self {
        JournalError::Io(e)
    }
}

impl From<SetLoggerError> for JournalError {
    fn from(e: SetLoggerError) -> Self {
        JournalError::SetLoggerError(e)
    }
}
