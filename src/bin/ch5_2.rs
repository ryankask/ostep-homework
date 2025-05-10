use nix::{
    fcntl::{OFlag, open},
    sys::stat::Mode,
    unistd::{ForkResult, fork, write},
};

fn main() {
    let fd = open(
        "/tmp/ostep-sample-data.txt",
        OFlag::O_WRONLY | OFlag::O_CREAT,
        Mode::S_IRWXU,
    )
    .unwrap();

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            write(
                fd,
                format!(
                    "Continuing execution in parent process, new child has pid: {}",
                    child
                )
                .as_bytes(),
            )
            .unwrap();
        }
        Ok(ForkResult::Child) => {
            write(fd, "Continuing execution in child process.".as_bytes()).unwrap();
        }
        Err(_) => println!("Fork failed"),
    }
}
