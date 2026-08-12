import { invoke, isTauri } from "@tauri-apps/api/core";

let cached: Promise<boolean> | null = null;

/**
 * True when the shell should draw its own window-control buttons
 * (minimize / maximize / close) in the top chrome.
 *
 * Only Linux desktops whose windows conventionally carry titlebar buttons
 * (GNOME, KDE, …) return true — the app runs undecorated there. Tiling
 * compositors (Hyprland, sway, …), macOS (native traffic lights), and
 * Windows (native decorations) return false. The answer cannot change
 * within a session, so it is cached after the first call.
 */
export function needsClientWindowControls(): Promise<boolean> {
  if (!isTauri()) {
    return Promise.resolve(false);
  }

  cached ??= invoke<boolean>("needs_client_window_controls").catch(() => false);
  return cached;
}
