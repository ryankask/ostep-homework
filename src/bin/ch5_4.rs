use nix::{
    sys::wait::waitpid,
    unistd::{ForkResult, execv, fork},
};

fn main() {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            execv(c"/bin/ls", &[c"ls", c"/tmp"]).unwrap();
        }
        Err(_) => println!("Fork failed"),
    }
}
