use nix::unistd::{ForkResult, close, fork, write};
use std::os::fd::AsRawFd;

fn main() {
    if let Ok(ForkResult::Child) = unsafe { fork() } {
        close(std::io::stdout().as_raw_fd()).expect("failed to close stdout");
        match write(std::io::stdout(), b"writing from child\n") {
            Ok(_) => (),
            Err(e) => {
                write(std::io::stderr(), format!("{}\n", e).as_bytes())
                    .expect("failed to write error message");
            }
        }
        unsafe { nix::libc::_exit(0) };
    }
}
