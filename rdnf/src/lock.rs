use std::{
    fs::{File, OpenOptions},
    str::FromStr,
};

use anyhow::{bail, Result};
use rustix::fd::AsRawFd;

pub enum RdnfFlockMode {
    READ,
    WriteRead,
    Wait,
}
pub struct Rdnflock {
    fd: File,
    pub openmode: RdnfFlockMode,
    pub path: String,
    pub descr: String,
    fdrefs: usize,
}
pub fn flock_new(lock_path: &str, descr: &str) -> Result<Rdnflock> {
    let flock = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(lock_path)
    {
        Ok(file) => Rdnflock {
            fd: file,
            openmode: RdnfFlockMode::WriteRead,
            path: String::from_str(lock_path).unwrap(),
            descr: String::from_str(descr).unwrap(),
            fdrefs: 1,
        },
        Err(_) => match OpenOptions::new().read(true).open(lock_path) {
            Ok(file) => Rdnflock {
                fd: file,
                openmode: RdnfFlockMode::READ,
                path: String::from_str(lock_path).unwrap(),
                descr: String::from_str(descr).unwrap(),
                fdrefs: 1,
            },
            Err(_) => {
                bail!("can't create {} lock on {}", lock_path, descr)
            }
        },
    };
    Ok(flock)
}
pub fn flock_acquire(lock: &mut Rdnflock, mode: RdnfFlockMode) -> Result<bool> {
    let mut res: bool = false;
    if lock.fdrefs > 1 {
        res = true
    } else {
        let cmd: i32 = match mode {
            RdnfFlockMode::Wait => libc::F_SETLKW,
            _ => libc::F_SETLK,
        };
        let l_type = match mode {
            RdnfFlockMode::READ => libc::F_RDLCK,
            RdnfFlockMode::WriteRead => libc::F_RDLCK,
            _ => libc::F_WRLCK,
        };
        let info = libc::flock {
            l_type: l_type as i16,
            l_whence: libc::SEEK_SET as i16,
            l_start: 0,
            l_len: 0,
            l_pid: 0,
        };
        unsafe {
            if libc::fcntl(lock.fd.as_raw_fd(), cmd, &info) != -1 {
                res = true;
            };
        }
    }
    lock.fdrefs += res as usize;
    Ok(res)
}
pub fn flock_release(lock: &mut Rdnflock) {
    if lock.fdrefs == 2 {
        let info = libc::flock {
            l_type: libc::F_UNLCK as i16,
            l_whence: libc::SEEK_SET as i16,
            l_start: 0,
            l_len: 0,
            l_pid: 0,
        };
        unsafe {
            libc::fcntl(lock.fd.as_raw_fd(), libc::F_SETLK, &info);
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_flock() {
        // let c=umask;
        // let t=c.to_string();
        // println!("c {}",t);
    }
}
