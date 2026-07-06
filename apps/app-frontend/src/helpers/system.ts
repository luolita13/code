import { invoke } from '@tauri-apps/api/core'

export interface CpuInfo {
	usagePercent: number
	brand: string
	coreCount: number
}

export interface MemoryInfo {
	totalBytes: number
	usedBytes: number
	usagePercent: number
}

export interface DiskInfo {
	totalBytes: number
	availableBytes: number
	usagePercent: number
	mountPoint: string
}

export interface NetworkInfo {
	receivedBytesPerSecond: number
	transmittedBytesPerSecond: number
}

export interface SystemInfo {
	cpu: CpuInfo
	memory: MemoryInfo
	disk: DiskInfo
	network: NetworkInfo
}

export async function get_system_info(): Promise<SystemInfo> {
	return await invoke('plugin:system|get_system_info')
}
