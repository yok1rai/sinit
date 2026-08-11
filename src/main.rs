use sinit::*;

fn main() {
    signal::init().expect("failed to initialize signal handling");

    match mount::mount_vf() {
        Ok(()) => {},
        Err(e) => {
            eprintln!("failed to mount virtual filesystem: {e}");
            return;
        }
    }
    match process::fork_exec("/bin/sh") {
        Ok((pid, arg)) => println!("{} started as PID {}", arg, pid),
        Err(e) => println!("fork failed: {e}")
    };
    loop {
        process::reap_chd();

        if let Err(e) = signal::wait() {
            eprintln!("failed to wait for signal: {e}"); 
        }
    }
}
