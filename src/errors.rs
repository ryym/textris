use failure;
use getopts;
use std::io;
use std::sync::mpsc::RecvError;

error_chain! {
    foreign_links {
        Io(io::Error);
        RecvDisconnected(RecvError);
        Opts(getopts::Fail);
    }

    errors {
        Fail(err: failure::Error) {
            description("failed to do something")
            display("{}", err)
        }
    }
}

// Define conversion from failure to error_chain temporarily
// to easily replace error_chain with failure.
impl From<failure::Error> for Error {
    fn from(err: failure::Error) -> Error {
        ErrorKind::Fail(err).into()
    }
}
