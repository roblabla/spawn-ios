mod attr;
mod file_action;

pub use attr::*;
pub use file_action::*;

use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::{OsStrExt, OsStringExt};

use libc::posix_spawn;

use libc::{c_char, pid_t};


pub struct PosixSpawn {
    actions: Option<file_action::PosixSpawnFileActions>,
    attr: Option<attr::PosixSpawnAttr>,

    path: OsString,
    args: Vec<OsString>,
    env: Vec<OsString>,
}

impl PosixSpawn {
    pub fn new<S: AsRef<OsStr>>(program: S) -> PosixSpawn {
        let arg0 = program.as_ref().to_owned();
        PosixSpawn {
            actions: None,
            attr: None,

            path: arg0.clone(),
            args: vec![arg0],
            env: vec![],
        }
    }

    pub fn arg<S: AsRef<OsStr>>(mut self, arg: S) -> PosixSpawn {
        self.args.push(arg.as_ref().to_owned());
        self
    }

    pub fn env<K, V>(mut self, key: K, val: V) -> PosixSpawn
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        let mut env = key.as_ref().to_owned();
        env.push("=");
        env.push(val.as_ref());
        self.env.push(env);

        self
    }

    pub fn attr(mut self, attr: PosixSpawnAttr) -> PosixSpawn {
        self.attr = Some(attr);
        self
    }

    pub fn file_actions(mut self, actions: PosixSpawnFileActions) -> PosixSpawn {
        self.actions = Some(actions);
        self
    }

    pub fn spawn(self) -> std::io::Result<pid_t> {
        let mut path_cstr = self.path.into_vec();
        path_cstr.push(0);

        let action = self.actions.as_ref().map(|v| v.as_raw() as *const _).unwrap_or(std::ptr::null());
        let attr = self.attr.as_ref().map(|v| v.as_raw() as *const _).unwrap_or(std::ptr::null());

        let mut args = self.args;
        let mut args_ffi = args.iter_mut().map(|v| v.as_bytes().as_ptr() as *mut c_char).collect::<Vec<_>>();
        args_ffi.push(std::ptr::null_mut());

        let mut env = self.env;
        let mut env_ffi = env.iter_mut().map(|v| v.as_bytes().as_ptr() as *mut c_char).collect::<Vec<_>>();
        env_ffi.push(std::ptr::null_mut());

        let mut child_pid = 0;
        let res = unsafe {
            posix_spawn(&mut child_pid, path_cstr.as_ptr() as *const c_char, action, attr, args_ffi.as_ptr(), env_ffi.as_ptr())
        };

        if res == 0 {
            Ok(child_pid)
        } else {
            Err(std::io::Error::from_raw_os_error(res))
        }
    }
}
