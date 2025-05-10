use nix::{
    sys::wait::wait,
    unistd::{ForkResult, fork},
};

fn main() {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => match wait() {
            Ok(status) => println!("PARENT: wait() returned: {:?}", status),
            Err(err) => println!("PARENT: wait() error: {}", err),
        },
        Ok(ForkResult::Child) => match wait() {
            Ok(status) => println!("CHILD: wait() returned: {:?}", status),
            Err(err) => println!("CHILD: wait() error: {}", err),
        },
        Err(_) => println!("Fork failed"),
    }
}
