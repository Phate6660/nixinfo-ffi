use nixinfo::{
    cpu, device, distro,
    env, environment, gpu,
    hostname, kernel, memory,
    music, packages, terminal,
    uptime};

use std::ffi::CString;

#[no_mangle]
pub extern fn cpu_ffi() -> CString {
    CString::new(cpu().unwrap()).unwrap()
}

#[no_mangle]
pub extern fn device_ffi() -> CString {
    CString::new(device().unwrap()).unwrap()
}

#[no_mangle]
pub extern fn distro_ffi() -> CString {
    CString::new(distro().unwrap()).unwrap()
}

#[no_mangle]
pub extern fn editor_ffi() -> CString{
    CString::new(env("EDITOR").unwrap()).unwrap()
}

#[no_mangle]
pub extern fn environment_ffi() -> CString {
    CString::new(environment().unwrap()).unwrap()
}

#[no_mangle]
pub extern fn gpu_ffi() -> CString {
    CString::new(gpu().unwrap()).unwrap()
}

#[no_mangle]
pub extern fn hostname_ffi() -> CString {
    CString::new(hostname().unwrap_or("N/A (could not read /etc/hostname)".to_string())).unwrap()
}

#[no_mangle]
pub extern fn kernel_ffi() -> CString {
    CString::new(kernel().unwrap()).unwrap()
}

#[no_mangle]
pub extern fn memory_ffi() -> CString {
    CString::new(memory().unwrap()).unwrap()
}

#[no_mangle]
pub extern fn packages_ffi(raw_manager: i64) -> CString {
    let manager = match raw_manager {
        0 => "apk",
        1 => "apt",
        2 => "dnf",
        3 => "dpkg",
        4 => "eopkg",
        5 => "pacman",
        6 => "pip",
        7 => "portage",
        8 => "rpm",
        9 => "xbps",
        _ => "unknown"
    };
    CString::new(packages(manager).unwrap()).unwrap()
}

#[no_mangle]
pub extern fn shell_ffi() -> CString {
    CString::new(env("SHELL").unwrap()).unwrap()
}

#[no_mangle]
pub extern fn terminal_ffi() -> CString {
    CString::new(terminal().unwrap()).unwrap()
}

#[no_mangle]
pub extern fn uptime_ffi() -> CString {
    CString::new(uptime().unwrap()).unwrap()
}

#[no_mangle]
pub extern fn user_ffi() -> CString {
    CString::new(env("USER").unwrap()).unwrap()
}

#[no_mangle]
pub extern fn music_ffi() -> CString {
    #[cfg(feature = "music")]
    return CString::new(music().unwrap()).unwrap();

    #[cfg(not(feature = "music"))]
    return CString::new(music()).unwrap();
}
