use nix::unistd::{execv, fork, ForkResult};
use std::ffi::CString;

fn main() {
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            let bash = CString::new("/bin/bash").unwrap();

            execv(&bash, &[bash.clone()]).unwrap();

            unreachable!();
        }
        Ok(ForkResult::Parent { child }) => {
            println!("bash started as PID {}", child);
        }
        Err(e) => {
            println!("fork failed: {e}");
        }
    }
    loop {}
}
