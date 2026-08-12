use sinit::*;

#[cfg(feature = "unsafe")]
use std::{thread, time::Duration};

fn spawn(
    command: process::Command,
    running: &mut process::RunningProcess,
) -> bool {
    match process::fork_exec(command.clone(), running) {
        Ok(pid) => {
            println!(
                "{} started process '{}' as PID {}",
                utils::boot_time(),
                command.path,
                pid
            );
            true
        }
        Err(e) => {
            eprintln!("fork failed for '{}': {e}", command.path);
            false
        }
    }
}

fn main() {
    if let Err(e) = signal::init() {
        eprintln!("failed to initialize signal handling: {e}");
    }

    if let Err(e) = mount::mount_vf() {
        eprintln!("failed to mount virtual filesystems: {e}");
    }

    if let Err(e) = process::setup_stdio() {
        eprintln!("failed to setup stdio redirection: {e}");
    }

    #[cfg(feature = "unsafe")]
    thread::spawn(|| {
        loop {
            panic::check_ipanic();
            thread::sleep(Duration::from_millis(500));
        }
    });

    let mut processes = Vec::new();

    if let Some(cmd) = process::Command::new(
        vec!["/bin/sh"],
        process::RestartPolicy::Always,
    ) {
        let mut running = process::RunningProcess {
            process: Vec::new(),
            command: cmd.clone(),
        };

        if spawn(cmd, &mut running) {
            processes.push(running);
        }
    }

    loop {
        let reaped_pids = process::reap_chd();

        for pid in reaped_pids {
            let Some(service) = processes
                .iter_mut()
                .find(|process| process.process.contains(&pid))
            else {
                continue;
            };

            service.process.retain(|p| *p != pid);

            match service.command.restart {
                process::RestartPolicy::Never => {
                    println!(
                        "{} PID {} exited permanently",
                        utils::boot_time(),
                        pid
                    );
                }

                process::RestartPolicy::Always => {
                    println!(
                        "{} PID {} exited, restarting {}...",
                        utils::boot_time(),
                        pid,
                        service.command.path
                    );

                    let command = service.command.clone();
                    spawn(command, service);
                }
            }
        }

        if let Err(e) = signal::wait() {
            eprintln!("failed to wait for signal: {e}");
        }
    }
}
