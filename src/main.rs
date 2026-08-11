use sinit::*;

fn main() {
    if let Err(e) = signal::init() {
        eprintln!("failed to initialize signal handling: {e}");
    }
    if let Err(e) =  mount::mount_vf() {
        eprintln!("failed to mount virtual filesystems: {e}");
    }

    if let Err(e) = process::setup_stdio() {
        eprintln!("failed to setup stdio redirection: {e}"); 
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
