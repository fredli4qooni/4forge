export async function invokeTauri<T>(cmd: string, args: Record<string, any> = {}): Promise<T | null> {
  if (typeof window === "undefined") return null;
  const hasTauri = Boolean((window as any).__TAURI_INTERNALS__) || Boolean((window as any).__TAURI__);
  if (!hasTauri) return null;
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    return await invoke<T>(cmd, args);
  } catch (err) {
    console.error(`[4Forge IPC Error] ${cmd}:`, err);
    return null;
  }
}
