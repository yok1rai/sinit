use sinit::*;

fn main() {
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
        nix::unistd::pause();
    }
}
