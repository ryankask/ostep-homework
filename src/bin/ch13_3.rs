use nix::libc;
use std::io::Write;

fn main() -> Result<(), String> {
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize };
    let size_in_mb = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse::<usize>().ok())
        .ok_or_else(|| "argument size in mb required or invalid number".to_owned())?;
    let size_in_bytes = size_in_mb * 1024 * 1024;
    let mut data = vec![0u8; size_in_bytes];

    // Touch every page using the dynamic page size
    for i in (0..size_in_bytes).step_by(page_size) {
        data[i] = 1;
    }

    println!(
        "Process with ID {} allocated and physically committed {} MB\nPress enter to exit...",
        std::process::id(),
        size_in_mb
    );

    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    Ok(())
}
