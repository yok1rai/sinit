use libc::{ioctl, TIOCSCTTY};
use nix::{
    fcntl::{OFlag, open}, sys::{signal::{self, SigSet}, stat::Mode, wait::{WaitPidFlag, WaitStatus, waitpid}}, unistd::{ForkResult, Pid, execv, fork, setsid}
};
use std::{ffi::CString, os::fd::AsRawFd};
use crate::utils;

pub fn setup_stdio() -> nix::Result<()> {
    let fd = open("/dev/console", OFlag::O_RDWR, Mode::empty())?;

    for target_fd in 0..=2 {
        unsafe {
            if libc::dup2(fd.as_raw_fd(), target_fd) < 0 {
                return Err(nix::Error::last());
            }
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug)]
pub enum RestartPolicy {
    Never,
    Always
}

#[derive(Clone)]
pub struct RunningProcess {
    pub process: Vec<Pid>,
    pub command: Command,
}


#[derive(Clone, Debug)]
pub struct Command {
    pub path: String,
    pub args: Vec<String>,
    pub restart: RestartPolicy
}

impl Command {
    pub fn new(args: Vec<&str>, restart: RestartPolicy) -> Option<Self> {
        let path = *args.first()?;
        Some(Self {
            path: path.to_string(),
            args: args.iter().map(|i| i.to_string()).collect(),
            restart,
        })
    }
}

pub fn fork_exec(command: Command, process_list: &mut RunningProcess) -> nix::Result<Pid> {
    match unsafe { fork()? } {
        ForkResult::Child => {
            signal::sigprocmask(
                signal::SigmaskHow::SIG_SETMASK,
                Some(&SigSet::empty()),
                None,
            )?;

            setsid()?;

            unsafe {
                if ioctl(0, TIOCSCTTY as _, 0) < 0 {
                    return Err(nix::Error::last());
                }
            }

            let args: Vec<CString> = command
                .args
                .iter()
                .map(|arg| CString::new(arg.as_str()))
                .collect::<Result<_, _>>()
                .map_err(|_| nix::errno::Errno::EINVAL)?;

            let path = &args[0];

            execv(path, &args)?;

            unreachable!();
        }

        ForkResult::Parent { child } => {
            process_list.process.push(child);
            Ok(child)
        }
    }
}

pub fn reap_chd() -> Vec<Pid> {
    let mut reaped = Vec::new();
    loop {
        match waitpid(
            Pid::from_raw(-1),
            Some(WaitPidFlag::WNOHANG)
        ) {
            Ok(WaitStatus::Exited(pid, status)) => {
                println!("{} reaped PID {pid}, exit status: {status}", utils::boot_time());
                reaped.push(pid);
            }
            Ok(WaitStatus::Signaled(pid, signal, _)) => {
                println!("{} reaped PID {pid}, killed by signal {signal}", utils::boot_time());
                reaped.push(pid);
            }
            Ok(WaitStatus::StillAlive) => {
                break;
            }
            Ok(_) => {}
            Err(nix::errno::Errno::ECHILD) => {
                break;
            }
            Err(err) => {
                eprintln!("{} waitpid failed: {err}", utils::boot_time());
                break;
            }
        }
    }
    reaped
}
