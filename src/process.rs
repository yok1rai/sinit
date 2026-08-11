use nix::{
    unistd::{execv, fork, ForkResult, Pid},
    sys::wait::{waitpid, WaitPidFlag, WaitStatus},
};
use std::ffi::CString;

use crate::utils;


pub fn fork_exec(args: &str) -> nix::Result<(Pid, &str)> {
    match unsafe { fork()? } {
        ForkResult::Child => {
            let bash = CString::new(args).unwrap();
            execv(&bash, &[bash.clone()])?;

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
