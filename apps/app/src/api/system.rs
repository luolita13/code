use crate::api::Result;
use serde::Serialize;
use std::sync::Mutex;
use std::time::Instant;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, Networks, RefreshKind, System};
use tauri::plugin::TauriPlugin;
use tauri::{Manager, State};

pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new("system")
        .invoke_handler(tauri::generate_handler![get_system_info])
        .setup(|app, _api| {
            app.manage(SystemMonitor::default());
            Ok(())
        })
        .build()
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CpuInfo {
    pub usage_percent: f32,
    pub brand: String,
    pub core_count: usize,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub usage_percent: f32,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiskInfo {
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f32,
    pub mount_point: String,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInfo {
    pub received_bytes_per_second: u64,
    pub transmitted_bytes_per_second: u64,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub disk: DiskInfo,
    pub network: NetworkInfo,
}

#[derive(Clone, Copy, Debug)]
struct NetworkSample {
    timestamp: Instant,
    received: u64,
    transmitted: u64,
}

pub struct SystemMonitor {
    system: Mutex<System>,
    last_network_sample: Mutex<Option<NetworkSample>>,
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self {
            system: Mutex::new(System::new_with_specifics(
                RefreshKind::nothing()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything()),
            )),
            last_network_sample: Mutex::new(None),
        }
    }
}

#[tauri::command]
pub fn get_system_info(monitor: State<SystemMonitor>) -> Result<SystemInfo> {
    let mut system = monitor.system.lock().map_err(|e| {
        theseus::Error::from(theseus::ErrorKind::OtherError(format!(
            "Failed to lock system monitor: {e}"
        )))
    })?;

    system.refresh_all();

    let cpu = system.cpus().first().map(|cpu| CpuInfo {
        usage_percent: cpu.cpu_usage(),
        brand: cpu.brand().to_string(),
        core_count: system.cpus().len(),
    });

    let memory = MemoryInfo {
        total_bytes: system.total_memory(),
        used_bytes: system.used_memory(),
        usage_percent: if system.total_memory() > 0 {
            (system.used_memory() as f32 / system.total_memory() as f32) * 100.0
        } else {
            0.0
        },
    };

    let mut disk = DiskInfo {
        total_bytes: 0,
        available_bytes: 0,
        usage_percent: 0.0,
        mount_point: String::new(),
    };

    let disks = Disks::new_with_refreshed_list();
    #[cfg(target_os = "windows")]
    let target = std::path::Path::new("C:\\");
    #[cfg(not(target_os = "windows"))]
    let target = std::path::Path::new("/");

    if let Some(main_disk) = disks.iter().find(|d| d.mount_point() == target) {
        disk.total_bytes = main_disk.total_space();
        disk.available_bytes = main_disk.available_space();
        disk.usage_percent = if main_disk.total_space() > 0 {
            ((main_disk.total_space() - main_disk.available_space()) as f64
                / main_disk.total_space() as f64
                * 100.0) as f32
        } else {
            0.0
        };
        disk.mount_point = main_disk.mount_point().to_string_lossy().to_string();
    }

    let networks = Networks::new_with_refreshed_list();
    let received_total: u64 = networks.iter().map(|(_, n)| n.total_received()).sum();
    let transmitted_total: u64 = networks.iter().map(|(_, n)| n.total_transmitted()).sum();

    let mut last_sample = monitor.last_network_sample.lock().map_err(|e| {
        theseus::Error::from(theseus::ErrorKind::OtherError(format!(
            "Failed to lock network sample: {e}"
        )))
    })?;

    let now = Instant::now();
    let network = if let Some(sample) = *last_sample {
        let elapsed = now.duration_since(sample.timestamp).as_secs_f64().max(1e-6);
        let received = received_total.saturating_sub(sample.received) as f64 / elapsed;
        let transmitted = transmitted_total.saturating_sub(sample.transmitted) as f64 / elapsed;
        NetworkInfo {
            received_bytes_per_second: received as u64,
            transmitted_bytes_per_second: transmitted as u64,
        }
    } else {
        NetworkInfo {
            received_bytes_per_second: 0,
            transmitted_bytes_per_second: 0,
        }
    };

    *last_sample = Some(NetworkSample {
        timestamp: now,
        received: received_total,
        transmitted: transmitted_total,
    });

    Ok(SystemInfo {
        cpu: cpu.unwrap_or(CpuInfo {
            usage_percent: 0.0,
            brand: String::new(),
            core_count: 0,
        }),
        memory,
        disk,
        network,
    })
}
