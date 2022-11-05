use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr::null_mut;

use libc::{posix_spawnattr_t, posix_spawnattr_init, posix_spawnattr_destroy, posix_spawnattr_setflags};
use libc::{c_char, c_int, c_short};

extern "C" {
    // Signature comes from https://opensource.apple.com/source/xnu/xnu-3789.41.3/libsyscall/wrappers/spawn/posix_spawn.c.auto.html
    //fn posix_spawnattr_setcpumonitor(attr: *mut posix_spawnattr_t, percent: u64, interval: u64) -> c_int;
    fn posix_spawnattr_setcpumonitor_default(attr: *mut posix_spawnattr_t) -> c_int;
    /// Set jetsam attributes for the spawn attribute object referred to by
    /// 'attr'.
    ///
    /// Parameters:
    /// - flags: The flags value to set
    /// - priority: Relative jetsam priority
    /// - memlimit_active: Value in megabytes; memory footprint above this level
    ///   while process is active may result in termination.
    /// - memlimit_inactive: Value in megabytes; memory footprint above this
    ///   level while process is inactive may result in termination.
    fn posix_spawnattr_setjetsam_ext(attr: *mut posix_spawnattr_t, flags: c_short, priority: c_int, memlimit_active: c_int, memlimit_inactive: c_int) -> c_int;

    fn posix_spawnattr_disable_ptr_auth_a_keys_np(attr: *mut posix_spawnattr_t, flags: u32) -> c_int;

    fn posix_spawnattr_set_subsystem_root_path_np(attr: *mut posix_spawnattr_t, path: *const c_char) -> c_int;
    fn posix_spawnattr_set_platform_np(attr: *mut posix_spawnattr_t, platform: c_int, unused_flags: u32) -> c_int;

    fn responsibility_spawnattrs_setdisclaim(attr: *mut posix_spawnattr_t, disclaim: bool) -> c_int;
}

pub struct PosixSpawnAttr(posix_spawnattr_t);

impl Drop for PosixSpawnAttr {
    fn drop(&mut self) {
        let _res = unsafe { posix_spawnattr_destroy(&mut self.0) };
    }
}

impl PosixSpawnAttr {
    pub fn new() -> std::io::Result<Self> {
        let mut attr = null_mut();
        let res = unsafe { posix_spawnattr_init(&mut attr) };
        if res == 0 {
            Ok(PosixSpawnAttr(attr))
        } else {
            Err(std::io::Error::from_raw_os_error(res))
        }
    }

    pub fn as_raw(&self) -> &posix_spawnattr_t {
        &self.0
    }

    pub fn setflags(&mut self, flags: i16) -> std::io::Result<()> {
        let res = unsafe { posix_spawnattr_setflags(&mut self.0, flags) };
        if res == 0 {
            Ok(())
        } else {
            Err(std::io::Error::from_raw_os_error(res))
        }
    }

    pub fn setcpumonitor_default(&mut self) -> std::io::Result<()> {
        let res = unsafe { posix_spawnattr_setcpumonitor_default(&mut self.0) };
        if res == 0 {
            Ok(())
        } else {
            Err(std::io::Error::from_raw_os_error(res))
        }
    }

    pub fn setjetsam_ext(&mut self, flags: c_short, priority: c_int, memlimit_active: c_int, memlimit_inactive: c_int) -> std::io::Result<()> {
        let res = unsafe { posix_spawnattr_setjetsam_ext(&mut self.0, flags, priority, memlimit_active, memlimit_inactive) };
        if res == 0 {
            Ok(())
        } else {
            Err(std::io::Error::from_raw_os_error(res))
        }
    }

    pub fn disable_ptr_auth_a_keys_np(&mut self, flags: u32) -> std::io::Result<()> {
        let res = unsafe { posix_spawnattr_disable_ptr_auth_a_keys_np(&mut self.0, flags) };
        if res == 0 {
            Ok(())
        } else {
            Err(std::io::Error::from_raw_os_error(res))
        }
    }

    pub fn responsibility_spawnattrs_setdisclaim(&mut self, disclaim: bool) -> std::io::Result<()> {
        let res = unsafe { responsibility_spawnattrs_setdisclaim(&mut self.0, disclaim) };
        if res == 0 {
            Ok(())
        } else {
            Err(std::io::Error::from_raw_os_error(res))
        }
    }

    pub fn set_subsystem_root_path_np<T: AsRef<Path>>(&mut self, path: T) -> std::io::Result<()> {
        let mut path_cstr = path.as_ref().as_os_str().as_bytes().to_vec();
        path_cstr.push(0);
        let res = unsafe { posix_spawnattr_set_subsystem_root_path_np(&mut self.0, path_cstr.as_ptr() as *mut c_char) };
        if res == 0 {
            Ok(())
        } else {
            Err(std::io::Error::from_raw_os_error(res))
        }
    }

    pub fn set_platform_np(&mut self, platform: c_int, flags: u32) -> std::io::Result<()> {
        let res = unsafe { posix_spawnattr_set_platform_np(&mut self.0, platform, flags) };
        if res == 0 {
            Ok(())
        } else {
            Err(std::io::Error::from_raw_os_error(res))
        }
    }
}
