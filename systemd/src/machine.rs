use crate::id128::ID128;

// https://www.freedesktop.org/software/systemd/man/machine-info.html#
struct Maschine {}

impl Maschine {
    //https://www.freedesktop.org/software/systemd/man/machine-id.html#
    fn id() -> ID128 {
        ID128::nil()
    }

    //todo use Result<Maschine, Error>
    fn info() -> Maschine {
        Maschine {}
    }
}