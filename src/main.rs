use sinit::*; 

fn main() {
    match fork_exec("/bin/sh") {
        Ok((pid, arg)) => println!("{} started as PID {}", arg, pid),
        Err(e) => println!("fork failed: {e}")
    };
    loop {}
}
