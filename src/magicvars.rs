use ewwii_plugin_api::EwwiiAPI;
use std::sync::Arc;

pub fn register_magic_variables(host: Arc<dyn EwwiiAPI>) {
    let mut current_exe = None;
    match std::env::current_exe() {
        Ok(exe_path) => {
            current_exe = Some(exe_path.clone());
            host.register_signal("EWWII_EXECUTABLE", exe_path.to_string_lossy().to_string());
        },
        Err(_) => eprintln!("Failed to get current exe path, EWWII_EXECUTABLE and EWWII_CMD will be unavailable."),
    }

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
        if let Some(exe) = current_exe {
            host_clone.register_signal("EWWII_CMD", format!("{} -c {}", exe.to_string_lossy(), paths.config_dir));
        }
    });

    // Register time
    host.register_signal("EWWII_TIME", chrono::offset::Utc::now().timestamp().to_string());

    let host_clone = host.clone();
    std::thread::spawn(move || {
        loop {
            host_clone.update_signal("EWWII_TIME", chrono::offset::Utc::now().timestamp().to_string());
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}
