use nix::{
    libc::_exit,
    sys::wait::waitpid,
    unistd::{ForkResult, close, dup2_stdin, dup2_stdout, fork, pipe, read, write},
};

fn main() {
    let (read_fd, write_fd) = pipe().expect("pipe failed");
    let mut child_pids = vec![];

    // Writer
    match unsafe { fork().unwrap() } {
        ForkResult::Parent { child } => {
            child_pids.push(child);
        }
        ForkResult::Child => {
            close(read_fd).unwrap();
            dup2_stdout(&write_fd).unwrap();
            close(write_fd).unwrap();
            write(std::io::stdout(), b"This is the secret message\n").unwrap();
            unsafe { _exit(0) }
        }
    }

    // Reader
    match unsafe { fork().unwrap() } {
        ForkResult::Parent { child } => {
            child_pids.push(child);
        }
        ForkResult::Child => {
            close(write_fd).unwrap();
            dup2_stdin(&read_fd).unwrap();
            close(read_fd).unwrap();
            let mut buf = [0u8; 128];
            let n = read(std::io::stdin(), &mut buf).unwrap();
            write(std::io::stdout(), b"Received: ").unwrap();
            write(std::io::stdout(), &buf[..n]).unwrap();
            unsafe { _exit(0) }
        }
    }

    close(read_fd).unwrap();
    close(write_fd).unwrap();

    for pid in child_pids {
        waitpid(pid, None).unwrap();
    }
}
