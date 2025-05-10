use nix::{
    libc,
    sys::wait::waitpid,
    unistd::{ForkResult, fork},
};

fn main() {
    let mut x = 0;

    println!("before fork x={}", x);

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            x = 5;
            println!(
                "Continuing execution in parent process, new child has pid: {}; x={}",
                child, x
            );
            waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            x = 10;
            // This is unsafe
            println!("Continuing execution in child process. x={}", x);
            unsafe { libc::_exit(0) };
        }
        Err(_) => println!("Fork failed"),
    }
}
