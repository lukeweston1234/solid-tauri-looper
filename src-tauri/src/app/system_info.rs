// use sysinfo::System;
// // use tauri::AppHandle;

// pub fn emit_system_info() {
//     let mut sys = System::new_all();
//     std::thread::spawn(move || {
//         loop {
//             let memory_usage = sys.used_memory() as f64;
//             let total_memory = sys.total_memory() as f64;
//             let percentage_used = (memory_usage / total_memory).round() * 100.0;

//             sys.refresh_cpu_usage(); // Refreshing CPU usage.
//             for cpu in sys.cpus() {
//                 print!("{}% ", cpu.cpu_usage());
//             }
//             println!("{:?}", percentage_used);
//             std::thread::sleep(std::time::Duration::from_secs(3));
//         }
//     });
// }
