use serde::Serialize;
use sysinfo::System;
use tauri::{AppHandle, Emitter};

#[derive(Serialize, Debug)]
struct SystemInfo {
    cpu_usage: f32,
    memory_usage: f64,
}

pub fn emit_system_info(app_handle: AppHandle) {
    let mut sys = System::new_all();
    std::thread::spawn(move || {
        loop {
            let memory_usage = sys.used_memory() as f64;
            let total_memory = sys.total_memory() as f64;

            let percentage_memory_used = ((memory_usage / total_memory) * 100.0).round();

            sys.refresh_cpu_usage(); // Refreshing CPU usage.

            let cpu_usage_sum: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum();
            let average_cpu_usage = cpu_usage_sum / sys.cpus().len() as f32;

            let system_info = SystemInfo {
                cpu_usage: average_cpu_usage,
                memory_usage: percentage_memory_used,
            };

            if let Err(e) = app_handle.emit("system_info", &system_info) {
                eprintln!("Failed to emit system info: {}", e);
            }

            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });
}
