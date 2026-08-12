use nix::time::{clock_gettime, ClockId};

pub fn boot_time() -> String {
    let ts = clock_gettime(ClockId::CLOCK_MONOTONIC).unwrap();

    format!(
        "[{}.{:06}]",
        ts.tv_sec(),
        ts.tv_nsec() / 1_000
    )
}
