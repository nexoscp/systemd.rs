/*
https://www.freedesktop.org/software/systemd/man/index.html

idiomatic rust-systemd api without ffi

idiomatic as far as i know rust. i'm new to the game
ffi like not using systemd-c-api, instead talking directly to sockets and parsing environment variables and files

based on https://github.com/systemd/systemd/tree/master/src/libsystemd

an ffi based systemd integration for rust can be found at https://github.com/jmesmon/rust-systemd

Rust Guides:
https://scribbles.pascalhertleif.de/elegant-apis-in-rust.html
https://pascalhertleif.de/artikel/good-practices-for-writing-rust-libraries/
*/

// see https://github.com/systemd/systemd/tree/master/src/libsystemd/sd-device
pub mod device;

pub mod netlink;

pub mod hotplug;

pub mod booted;
pub mod daemon;
pub mod osrelease;
pub mod machine;
pub mod id128;
pub mod watchdog;
pub mod login;
pub mod bus;
pub mod notify;
pub mod signal {
    /*
    Dealing with signals in a type safe way and providing a documentation
    @see http://man7.org/linux/man-pages/man7/signal.7.html
    */
}
pub trait Service {}
