use nix::{
    errno::Errno, mount::{MsFlags, mount},
};
use std::{
    fs
};

use crate::utils;

pub fn mount_fs(source: Option<&str>, target: &str, fstype: Option<&str>) -> nix::Result<()> {
    mount(
        source,
        target,
        fstype,
        MsFlags::empty(),
        None::<&str>,
    )
}

pub fn mount_vf() -> nix::Result<()> {
    let vfs = [
        (None,"/proc", "proc"),
        (None,"/sys", "sysfs"),
        (None,"/dev", "devtmpfs"),
        (None,"/run", "tmpfs"),
        (None,"/dev/pts", "devpts"),
        (None,"/dev/shm", "tmpfs"),
        (None ,"/sys/fs/cgroup", "cgroup2"),
    ];
    for (idx, (source ,target, fstype)) in vfs.iter().enumerate() {
        match mount_fs(*source, target, Some(fstype)) {
            Ok(()) => {
                println!(
                    "* [{}/{}] {} successfully mounted {}",
                    idx + 1,
                    vfs.len(),
                    target,
                    utils::boot_time()
                );
            }
            Err(Errno::ENOENT) => {
                fs::create_dir_all(target)
                    .map_err(|_| Errno::EIO)?;

                mount_fs(*source, target, Some(fstype))?;

                println!(
                    "* [{}/{}] {} successfully mounted {}",
                    idx + 1,
                    vfs.len(),
                    target,
                    utils::boot_time()
                );
            }
            Err(e) => {
                return Err(e);
            }
        };
    }
    println!("{} every virtual filesystems are mounted!", utils::boot_time());
    Ok(())
}
