use crate::logging::log_activity;
use crate::paths::mount_path;
use std::fs;
use std::process::Command;
use std::time::Duration;

pub fn is_mounted() -> bool {
    let Ok(target) = mount_path() else {
        return false; // can't determine home dir, so we can't be mounted
    };

    let Ok(mounts) = fs::read_to_string("/proc/mounts") else {
        return Command::new("mountpoint")
            .arg("-q")
            .arg(&target)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
    };

    let target_str = target.to_string_lossy();
    mounts.lines().any(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        parts.len() >= 2 && parts[1] == target_str
    })
}

pub fn start() -> Result<(), String> {
    let target = mount_path()?;

    if is_mounted() {
        println!("[WARN] Already mounted at {:?}", target);
        return Ok(());
    }

    fs::create_dir_all(&target)
        .map_err(|e| format!("Could not create mount directory {:?}: {}", target, e))?;

    let _ = log_activity("START", "Initiating mount");
    println!("Starting mount at {:?}...", target);

    let target_str = target
        .to_str()
        .ok_or_else(|| "Mount path is not valid UTF-8".to_string())?;

    let spawn_result = Command::new("rclone")
        .args([
            "mount",
            ":http:",
            target_str,
            "--http-url",
            "https://172.16.16.6",
            "--no-check-certificate",
            "--read-only",
            "--vfs-cache-mode",
            "minimal",
            "--daemon",
        ])
        .status();

    if let Err(e) = spawn_result {
        let err_msg = format!("Failed to launch rclone: {}", e);
        let _ = log_activity("START_FAILED", &err_msg);
        return Err(err_msg);
    }

    // Poll for up to ~3 seconds instead of one blind sleep
    for _ in 0..10 {
        if is_mounted() {
            let _ = log_activity("SUCCESS", "Mounted successfully");
            println!("[SUCCESS] Mounted successfully.");
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(300));
    }

    let err_msg = "rclone launched but mount never appeared".to_string();
    let _ = log_activity("START_FAILED", &err_msg);
    Err(err_msg)
}

pub fn stop() -> Result<(), String> {
    let target = mount_path()?;

    if !is_mounted() {
        println!("[WARN] Not currently mounted.");
        return Ok(());
    }

    let _ = log_activity("STOP", "Initiating unmount");
    println!("Stopping mount...");

    let target_str = target
        .to_str()
        .ok_or_else(|| "Mount path is not valid UTF-8".to_string())?;

    let status = Command::new("fusermount3")
        .args(["-u", target_str])
        .status()
        .map_err(|e| format!("Failed to run fusermount3: {}", e))?;

    if status.success() {
        let _ = log_activity("SUCCESS", "Unmounted successfully");
        println!("[SUCCESS] Unmounted successfully.");
        Ok(())
    } else {
        let err_msg = "fusermount3 exited with an error (device might be busy)".to_string();
        let _ = log_activity("STOP_FAILED", &err_msg);
        Err(err_msg)
    }
}
