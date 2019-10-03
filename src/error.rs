pub enum Kind {
    PlayerError(PlayerErrorKind),
    ConnectionError,
}

pub enum PlayerErrorKind {
    PlayError,
    PauseError,
    StopError,
    StatusError,
}

use Kind::{PlayerError, ConnectionError};

fn handler(err: Kind) -> String {
    match Kind {
        PlayerErrorKind(p) => {

        },
        ConnectionError => {

        }
    }
}
