use nixinfo::{
    cpu, device, distro,
    env, environment, gpu,
    hostname, kernel, memory,
    music, packages, terminal,
    uptime};

#[no_mangle]
pub extern fn cpu_ffi() {
    println!("{}", cpu().unwrap());
}

#[no_mangle]
pub extern fn device_ffi() {
    println!("{}", device().unwrap());
}

#[no_mangle]
pub extern fn distro_ffi() {
    println!("{}", distro().unwrap());
}

#[no_mangle]
pub extern fn editor_ffi() {
    println!("{}", env("EDITOR").unwrap());
}

#[no_mangle]
pub extern fn environment_ffi() {
    println!("{}", environment().unwrap());
}

#[no_mangle]
pub extern fn gpu_ffi() {
    println!("{}", gpu().unwrap());
}

#[no_mangle]
pub extern fn hostname_ffi() {
    println!("{}", hostname().unwrap_or("N/A (could not read /etc/hostname)".to_string()));
}

#[no_mangle]
pub extern fn kernel_ffi() {
    println!("{}", kernel().unwrap());
}

#[no_mangle]
pub extern fn memory_ffi() {
    println!("{}", memory().unwrap());
}

#[no_mangle]
pub extern fn packages_ffi(raw_manager: i64) {
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
    println!("{}", packages(manager).unwrap());
}

#[no_mangle]
pub extern fn shell_ffi() {
    println!("{}", env("SHELL").unwrap());
}

#[no_mangle]
pub extern fn terminal_ffi() {
    println!("{}", terminal().unwrap());
}

#[no_mangle]
pub extern fn uptime_ffi() {
    println!("{}", uptime().unwrap());
}

#[no_mangle]
pub extern fn user_ffi() {
    println!("{}", env("USER").unwrap());
}

#[no_mangle]
pub extern fn music_ffi() {
    #[cfg(feature = "music")]
    println!("{}", &music().unwrap());

    #[cfg(not(feature = "music"))]
    println!("{}", music());
}
