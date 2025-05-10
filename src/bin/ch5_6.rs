use nix::{
    sys::wait::waitpid,
    unistd::{ForkResult, fork},
};

fn main() {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => match waitpid(child, None) {
            Ok(status) => println!("PARENT: waitpid() returned: {:?}", status),
            Err(err) => println!("PARENT: waitpid() error: {}", err),
        },
        Ok(ForkResult::Child) => {
            unsafe { nix::libc::_exit(0) };
        }
        Err(_) => println!("Fork failed"),
    }
}
