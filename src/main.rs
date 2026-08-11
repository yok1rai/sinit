use sinit::*;
use nix::unistd::{Pid, getuid};
#[cfg(feature = "unsafe")]
use std::{thread, time::Duration};

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

#[cfg(feature = "unsafe")]
unsafe fn ipanic() {
    if !getuid().is_root() {
        eprintln!("you need to run as root");
        return;
    }
    std::process::exit(0);
}

#[cfg(feature =  "unsafe")]
fn check_ipanic() {
    if let Ok(content) = std::fs::read_to_string("/etc/ipanic") {
        if content.trim() == "panic" {
            println!("panic triggered!");
            unsafe {
                ipanic();
            }
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

    #[cfg(feature = "unsafe")]
    thread::spawn(|| {
        loop {
            use std::time::Duration;

            check_ipanic();
            thread::sleep(Duration::from_millis(500));
        }
    });

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
