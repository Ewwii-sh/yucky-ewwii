use ewwii_plugin_api::EwwiiAPI;
use crate::system_stats;
use std::sync::Arc;

pub fn register_magic_variables(host: Arc<dyn EwwiiAPI>) {
    let current_exe = std::env::current_exe().map(|x| x.to_string_lossy().into_owned()).unwrap_or_else(|_| "ewwii".to_string());
    host.register_signal("EWWII_EXECUTABLE", current_exe.clone());

    let frt_paths = host.get_runtime_paths();
    let host_clone = host.clone();
    frt_paths.resolve_async(move |res| {
        let Ok(paths) = res else {
            eprintln!(
                "[yucky-ewwii] OH NO! Failed to resolve future paths. EWWII_CONFIG_DIR and EWWII_CMD will be unavailable."
            );
            return;
        };

        host_clone.register_signal("EWWII_CONFIG_DIR", paths.config_dir.clone());
        host_clone.register_signal("EWWII_CMD", format!("{} -c {}", current_exe, paths.config_dir));
    });

    // Register time
    host.register_signal("EWWII_TIME", system_stats::get_time());

    let host_clone = host.clone();
    std::thread::spawn(move || {
        loop {
            host_clone.update_signal("EWWII_TIME", system_stats::get_time());
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    // Register others
    host.register_signal("EWWII_TEMPS", system_stats::get_temperatures());
    host.register_signal("EWWII_RAM", system_stats::get_ram());
    host.register_signal("EWWII_DISK", system_stats::get_disks());
    if let Ok(s) = system_stats::get_battery_capacity() {
        host.register_signal("EWWII_BATTERY", s);
    } else {
        eprintln!("Cannot get battery capacity, EWWII_BATTERY will be unavailable.");
    }
    host.register_signal("EWWII_CPU", system_stats::get_cpus());
    host.register_signal("EWWII_NET", system_stats::net());

    // Update signals
    let host_clone = host.clone();
    std::thread::spawn(move || {
        loop {
            host_clone.update_signal("EWWII_TEMPS", system_stats::get_temperatures());
            host_clone.update_signal("EWWII_RAM", system_stats::get_ram());
            host_clone.update_signal("EWWII_DISK", system_stats::get_disks());
            if let Ok(s) = system_stats::get_battery_capacity() {
                host_clone.update_signal("EWWII_BATTERY", s);
            } else {
                eprintln!("Cannot get battery capacity, skipping updating EWWII_BATTERY.");
            }
            host_clone.update_signal("EWWII_CPU", system_stats::get_cpus());
            host_clone.update_signal("EWWII_NET", system_stats::net());
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}
