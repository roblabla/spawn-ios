use std::path::Path;
use std::os::unix::ffi::OsStrExt;

use libc::{c_char, posix_spawn_file_actions_t, posix_spawn_file_actions_addopen, posix_spawn_file_actions_destroy, posix_spawn_file_actions_init};

pub struct PosixSpawnFileActions(posix_spawn_file_actions_t);

impl Drop for PosixSpawnFileActions {
    fn drop(&mut self) {
        let _res = unsafe { posix_spawn_file_actions_destroy(&mut self.0) };
    }
}

impl PosixSpawnFileActions {
    pub fn new() -> std::io::Result<Self> {
        let mut actions = std::ptr::null_mut();
        let res = unsafe { posix_spawn_file_actions_init(&mut actions) };
        if res == 0 {
            Ok(PosixSpawnFileActions(actions))
        } else {
            Err(std::io::Error::from_raw_os_error(res))
        }
    }

    pub fn addopen<T: AsRef<Path>>(&mut self, fd: i32, path: T, oflags: i32, perms: u16) -> std::io::Result<()> {
        let mut path_cstr = path.as_ref().as_os_str().as_bytes().to_vec();
        path_cstr.push(0);
        let res = unsafe { posix_spawn_file_actions_addopen(&mut self.0, fd, path_cstr.as_ptr() as *const c_char, oflags, perms) };

        if res == 0 {
            Ok(())
        } else {
            Err(std::io::Error::from_raw_os_error(res))
        }
    }

    pub fn as_raw(&self) -> &posix_spawn_file_actions_t {
        &self.0
    }
}
