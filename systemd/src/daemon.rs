// https://www.freedesktop.org/software/systemd/man/daemon.html#
use std::os::unix::io::RawFd;
//TODO socket activated
const SD_LISTEN_FDS_START: RawFd = 3;

pub fn socketActivated() {
    //env::key("LISTEN_FDS");
    //env::key("LISTEN_PID");
}

//TODO bus activated
//TODO timer activated
//TODO path activated
//TODO device activated
//
