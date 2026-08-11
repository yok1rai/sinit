use sinit::*;
use nix::unistd::Pid;

fn spawn_shell() -> Option<Pid> {
    match process::fork_exec("/bin/sh") {
        Ok((pid, arg)) => {
            println!("{} started as PID {}", arg, pid);
            Some(pid)
        }
        Err(e) => {
            eprintln!("fork failed: {e}");
            None
        }
    }
}

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

    let mut shell_pid = spawn_shell();
    loop {
        let reaped_pids = process::reap_chd();

        if let Some(active_pid) = shell_pid {
            if reaped_pids.contains(&active_pid) {
                println!("Shell (PID {}) exited. respawning...", active_pid);
                shell_pid = spawn_shell();
            }
         }

        if let Err(e) = signal::wait() {
            eprintln!("failed to wait for signal: {e}");
        }
    }
}
