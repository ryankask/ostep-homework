use nix::{
    libc,
    unistd::{ForkResult, fork, write},
};
use std::time::Duration;

fn main() {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => {
            std::thread::sleep(Duration::from_millis(5));
            println!("goodbye");
        }
        Ok(ForkResult::Child) => {
            write(std::io::stdout(), "hello\n".as_bytes()).ok();
            unsafe { libc::_exit(0) };
        }
        Err(_) => println!("Fork failed"),
    }
}
