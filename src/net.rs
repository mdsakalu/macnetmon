use std::collections::HashMap;
use std::ffi::CStr;
use std::io;
use std::process::Command;

#[derive(Clone, Debug)]
pub struct InterfaceSample {
    pub name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub flags: u32,
    pub is_loopback: bool,
}

pub fn get_hostname() -> Option<String> {
    let mut buf = [0u8; 256];
    let rc = unsafe { libc::gethostname(buf.as_mut_ptr() as *mut i8, buf.len()) };
    if rc != 0 {
        return None;
    }
    let len = buf.iter().position(|b| *b == 0).unwrap_or(buf.len());
    String::from_utf8(buf[..len].to_vec()).ok()
}

pub fn is_up(flags: u32) -> bool {
    (flags & libc::IFF_UP as u32) != 0
}

pub fn is_physical_interface(name: &str) -> bool {
    if !name.starts_with("en") || name.len() <= 2 {
        return false;
    }
    name[2..].chars().all(|c| c.is_ascii_digit())
}

pub fn sample_interfaces() -> io::Result<Vec<InterfaceSample>> {
    let mut addrs: *mut libc::ifaddrs = std::ptr::null_mut();
    let result = unsafe { libc::getifaddrs(&mut addrs) };
    if result != 0 {
        return Err(io::Error::last_os_error());
    }

    let mut samples = Vec::new();
    let mut current = addrs;

    while !current.is_null() {
        let ifa = unsafe { &*current };

        if ifa.ifa_addr.is_null() || ifa.ifa_name.is_null() {
            current = ifa.ifa_next;
            continue;
        }

        let family = unsafe { (*ifa.ifa_addr).sa_family as i32 };
        if family == libc::AF_LINK && !ifa.ifa_data.is_null() {
            let data = unsafe { &*(ifa.ifa_data as *const libc::if_data) };
            let name = unsafe { CStr::from_ptr(ifa.ifa_name) }
                .to_string_lossy()
                .to_string();
            let flags = ifa.ifa_flags;
            let is_loopback = (flags & libc::IFF_LOOPBACK as u32) != 0;

            samples.push(InterfaceSample {
                name,
                rx_bytes: data.ifi_ibytes as u64,
                tx_bytes: data.ifi_obytes as u64,
                flags,
                is_loopback,
            });
        }

        current = ifa.ifa_next;
    }

    unsafe { libc::freeifaddrs(addrs) };

    Ok(samples)
}

pub fn load_interface_aliases() -> io::Result<HashMap<String, String>> {
    let output = Command::new("networksetup")
        .arg("-listallhardwareports")
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "networksetup failed"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut map = HashMap::new();
    let mut current_port: Option<String> = None;

    for line in stdout.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("Hardware Port:") {
            current_port = Some(rest.trim().to_string());
        } else if let Some(rest) = line.strip_prefix("Device:") {
            let dev = rest.trim();
            if !dev.is_empty() {
                if let Some(port) = current_port.take() {
                    map.insert(dev.to_string(), port);
                }
            }
        }
    }

    Ok(map)
}
