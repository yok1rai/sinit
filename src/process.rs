use nix::unistd::{execv, fork, ForkResult, Pid};
use std::ffi::CString;

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
