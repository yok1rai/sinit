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



pub fn fork_exec(args: &str) -> nix::Result<(Pid, &str)> {
    match unsafe { fork()? } {
        ForkResult::Child => {
            signal::sigprocmask(
                signal::SigmaskHow::SIG_SETMASK,
                Some(&SigSet::empty()),
                None)?;
            setsid()?;

            unsafe {
                ioctl(0, TIOCSCTTY as _, 0);
            }

            let path = CString::new(args).unwrap();
            execv(&path, &[path.clone()])?;

            unreachable!();
        }
        ForkResult::Parent { child } => Ok((child, args))
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
