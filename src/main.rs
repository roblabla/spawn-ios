use libc::{c_int, c_short};
use libc::{POSIX_SPAWN_SETPGROUP, POSIX_SPAWN_CLOEXEC_DEFAULT, POSIX_SPAWN_SETEXEC, POSIX_SPAWN_START_SUSPENDED};
//use libc::{S_IROTH, S_IWOTH, S_IRGRP, S_IWGRP, S_IRUSR, S_IWUSR, O_NOCTTY, O_RDWR};

mod posix_spawn;

pub use posix_spawn::*;

// From https://github.com/apple-opensource/xnu/blob/eb45a4f3d6bc33c958fcbf4ea7388da13ae63a2e/bsd/sys/spawn.h
const _POSIX_SPAWN_NANO_ALLOCATOR: i32 = 0x200;
const POSIX_SPAWN_JETSAM_MEMLIMIT_ACTIVE_FATAL: c_short = 0x04;
const POSIX_SPAWN_JETSAM_MEMLIMIT_INACTIVE_FATAL: c_short = 0x08;
const JETSAM_PRIORITY_BACKGROUND: c_int = 3;

const PLATFORM_IOS: c_int = 2;

fn main() {
    let path = std::env::args().nth(1).unwrap();

    //let mut file_actions = PosixSpawnFileActions::new().expect("Failed to create fileactions");
    //file_actions.addopen(0, "/dev/null", O_NOCTTY, S_IROTH | S_IWOTH | S_IRGRP | S_IWGRP | S_IRUSR | S_IWUSR).expect("Failed to open stdin");
    //file_actions.addopen(1, "/dev/null", O_NOCTTY | O_RDWR, S_IROTH | S_IWOTH | S_IRGRP | S_IWGRP | S_IRUSR | S_IWUSR).expect("Failed to open stdout");
    //file_actions.addopen(2, "/dev/null", O_NOCTTY | O_RDWR, S_IROTH | S_IWOTH | S_IRGRP | S_IWGRP | S_IRUSR | S_IWUSR).expect("Failed to open stderr");

    let mut spawnattr = PosixSpawnAttr::new().expect("Failed to create spawn attrs");

    spawnattr.setflags((POSIX_SPAWN_SETPGROUP | _POSIX_SPAWN_NANO_ALLOCATOR | POSIX_SPAWN_SETEXEC | POSIX_SPAWN_START_SUSPENDED | POSIX_SPAWN_CLOEXEC_DEFAULT) as _).unwrap();
    spawnattr.setcpumonitor_default().unwrap();
    spawnattr.setjetsam_ext(POSIX_SPAWN_JETSAM_MEMLIMIT_ACTIVE_FATAL | POSIX_SPAWN_JETSAM_MEMLIMIT_INACTIVE_FATAL, JETSAM_PRIORITY_BACKGROUND, 0x4000, 0x4000).unwrap();
    spawnattr.disable_ptr_auth_a_keys_np(1).unwrap();
    spawnattr.responsibility_spawnattrs_setdisclaim(true).unwrap();
    spawnattr.set_subsystem_root_path_np("/System/iOSSupport/").unwrap();
    spawnattr.set_platform_np(PLATFORM_IOS, 0).unwrap();

    

    // TODO: Get bundle id, either in embedded binary, or by walking the parents
    // until we find an Info.plist.
    let bundle_id = "jp.co.capcom.gyakusai4en.62GD992E8J".to_string();
    // TODO: Get path to container from bundleid.
    let container_path = format!("{}/Library/Containers/01CE337F-0CC7-4CEA-911C-E17AF0C14100/Data", std::env::var("HOME").unwrap());
    let _pid = PosixSpawn::new(path)
        //.file_actions(file_actions)
        .attr(spawnattr)
        .env("MallocSpaceEfficient", "1")
        .env("USER", std::env::var("USER").unwrap())
        .env("COMMAND_MODE", "unix2003")
        .env("__CFBundleIdentifier", bundle_id.clone())
        .env("_DYLD_CLOSURE_HOME", container_path.clone())
        .env("PATH", "/usr/bin:/bin:/usr/sbin:/sbin")
        .env("LOGNAME", std::env::var("LOGNAME").unwrap())
        .env("CFFIXED_USER_HOME", container_path.clone())
        .env("SSH_AUTH_SOCK", std::env::var("SSH_AUTH_SOCK").unwrap())
        .env("HOME", std::env::var("HOME").unwrap())
        .env("SHELL", "/bin/zsh")
        .env("TMPDIR", std::env::var("TMPDIR").unwrap())
        .env("__CF_USER_TEXT_ENCODING", "0x1F5:0x0:0x0")
        .env("XPC_SERVICE_NAME", format!("application.{}", bundle_id))
        .env("XPC_FLAGS", "1")
        .spawn()
        .unwrap();

    // Will never reach here due to SETEXEC
}
