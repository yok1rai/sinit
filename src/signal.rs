use nix::sys::signal::{
    self, SaFlags, SigAction, SigHandler, SigSet, SigmaskHow, sigaction, Signal
};

extern "C" fn signal_handler(_: i32) {}

pub fn init() -> nix::Result<()> {
    let mut blocked = SigSet::empty();
    blocked.add(Signal::SIGCHLD);

    signal::sigprocmask(
    SigmaskHow::SIG_BLOCK,
    Some(&blocked),
 None)?;

    let action = SigAction::new(
            SigHandler::Handler(signal_handler),
        SaFlags::empty(),
        SigSet::empty());

    unsafe {
        sigaction(Signal::SIGCHLD, &action)?;
    }

    Ok(())
}

pub fn wait() -> nix::Result<()> {
    SigSet::empty().suspend()
}
