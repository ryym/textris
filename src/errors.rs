use std::io;
use std::sync::mpsc::RecvError;

error_chain! {
    foreign_links {
        Io(io::Error);
        RecvDisconnected(RecvError);
    }
}
