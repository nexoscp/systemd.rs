/*
https://www.freedesktop.org/software/systemd/man/sd-id128.html#
https://doc.rust-lang.org/uuid/uuid/index.html
https://doc.rust-lang.org/uuid/src/uuid/lib.rs.html#11-1152
*/


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ID128 {
   content: u128
}

impl ID128 {
    pub fn nil() -> ID128 {
        ID128 {content: 0}
    }

    pub fn random() -> ID128 {
//        let mut rng = rand::thread_rng();
        ID128 { content: 1 }
    }
}

impl From<u128> for ID128 {
    fn from(value: u128) -> Self {
        ID128 { content: value}
    }
}
//TODO impl From<UUID> and other way around https://github.com/uuid-rs/uuid behind feature flag