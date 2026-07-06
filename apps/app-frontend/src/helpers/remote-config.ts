import { openUrl } from '@tauri-apps/plugin-opener'

export const REMOTE_CONFIG_BASE =
  'https://raw.githubusercontent.com/luolita13/code/main/remote'

export interface RemoteVersionInfo {
  latest: string
  downloadUrl: string
  releaseNotes?: string
}

export interface RemoteNotice {
  id: string
  title?: string
  body: string
  severity?: 'info' | 'warning' | 'critical'
  minAppVersion?: string
  maxAppVersion?: string
  dismissible?: boolean
  showOnce?: boolean
}

export interface RemoteNotices {
  notices: RemoteNotice[]
}

export async function fetchRemoteVersion(): Promise<RemoteVersionInfo | null> {
  try {
    const res = await fetch(`${REMOTE_CONFIG_BASE}/version.json`, {
      cache: 'no-cache',
    })
    if (!res.ok) return null
    return (await res.json()) as RemoteVersionInfo
  } catch (e) {
    console.error('Failed to fetch remote version:', e)
    return null
  }
}

export async function fetchRemoteNotices(): Promise<RemoteNotices | null> {
  try {
    const res = await fetch(`${REMOTE_CONFIG_BASE}/notices.json`, {
      cache: 'no-cache',
    })
    if (!res.ok) return null
    return (await res.json()) as RemoteNotices
  } catch (e) {
    console.error('Failed to fetch remote notices:', e)
    return null
  }
}

export function compareVersions(a: string, b: string): number {
  const parse = (v: string) =>
    v
      .replace(/^v/, '')
      .split('.')
      .map((x) => parseInt(x, 10) || 0)
  const av = parse(a)
  const bv = parse(b)
  for (let i = 0; i < Math.max(av.length, bv.length); i++) {
    const ai = av[i] ?? 0
    const bi = bv[i] ?? 0
    if (ai !== bi) return ai - bi
  }
  return 0
}

export function openExternalUpdateLink(url: string) {
  openUrl(url)
}
