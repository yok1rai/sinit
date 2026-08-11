use libc::{ioctl, TIOCSCTTY};
use nix::{
    unistd::{execv, fork, ForkResult, Pid, setsid},
    sys::wait::{waitpid, WaitPidFlag, WaitStatus},
    fcntl::{open, OFlag},
    sys::stat::Mode
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

pub fn reap_chd() {
    loop {
        match waitpid(
            Pid::from_raw(-1),
            Some(WaitPidFlag::WNOHANG)
        ) {
            Ok(WaitStatus::Exited(pid, status)) => {
                println!("{} reaped PID {pid}, exit status: {status}", utils::boot_time());
            }
            Ok(WaitStatus::Signaled(pid, signal, _)) => {
                println!("{} reaped PID {pid}, killed by signal {signal}", utils::boot_time());
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
}
