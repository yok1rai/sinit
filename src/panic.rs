use nix::unistd::getuid;

 fn ipanic() {
    if !getuid().is_root() {
        eprintln!("you need to run as root");
        return;
    }
    if let Err(e) = std::fs::write("/etc/ipanic", b"") {
        eprintln!("cannot clear the /etc/ipanic, canceling...");
        return;
    }
    nix::unistd::sync();
    std::process::exit(0);
}

pub fn check_ipanic() {
    if let Ok(content) = std::fs::read_to_string("/etc/ipanic") {
        if content.trim() == "panic" {
            println!("panic triggered!");
            ipanic();
        }
    }
}
