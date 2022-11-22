use std::ffi::CStr;

use anyhow::{bail, Result};
use libc::geteuid;

use crate::{
    default::RDNF_INSTANCE_LOCK_FILE,
    lock::{flock_acquire, flock_release},
};

use super::lock::{flock_new, RdnfFlockMode};

pub fn check_root() -> Result<()> {
    if unsafe { geteuid() } != 0 {
        bail!("root permission is required for this operation");
    }
    Ok(())
}

pub fn is_already_running() -> Result<()> {
    if unsafe { geteuid() } == 0 {
        let mut lock = flock_new(RDNF_INSTANCE_LOCK_FILE, "rdnf_instance")?;
        if !flock_acquire(&mut lock, RdnfFlockMode::WriteRead)? {
            match lock.openmode {
                RdnfFlockMode::WriteRead => {
                    println!("waiting for {} lock on {}", lock.descr, lock.path);
                    if !flock_acquire(&mut lock, RdnfFlockMode::Wait)? {
                        flock_release(&mut lock);
                        bail!("can't create {} lock on {}", lock.descr, lock.path);
                    }
                }
                _ => {
                    flock_release(&mut lock);
                    bail!("Failed to acquire rdnf_instance lock")
                }
            }
        }
    }
    Ok(())
}
pub fn check_dir(path: &str) -> Result<bool> {
    match std::fs::read_dir(path) {
        Ok(c) => {
            return Ok(c.count() > 0);
        }
        Err(_) => {
            bail!("Dir {} don't exist", path)
        }
    }
}
pub fn format_size(size: u64) -> String {
    let mut dsize = size as f32;
    for i in ["b", "k", "M", "G"] {
        if dsize >= 1024.0 {
            dsize /= 1024.0;
        } else {
            return format!("{:.2}{}", dsize, i);
        }
    }
    format!("{:.2}T", dsize)
}
#[inline]
pub fn c_str_ptr_to_rust_string(ptr:*const i8)->Option<String>{
    if ptr.is_null(){
        None
    }else{
        unsafe{
            Some(CStr::from_ptr(ptr).to_str().unwrap().to_string())
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    pub fn test() {}
}
