/// Performs the platform's default sidebar alignment haptic when available.
#[tauri::command]
pub fn perform_sidebar_default_haptic() {
    #[cfg(target_os = "macos")]
    {
        use objc2_app_kit::{
            NSHapticFeedbackManager, NSHapticFeedbackPattern, NSHapticFeedbackPerformanceTime,
            NSHapticFeedbackPerformer,
        };

        NSHapticFeedbackManager::defaultPerformer().performFeedbackPattern_performanceTime(
            NSHapticFeedbackPattern::Alignment,
            NSHapticFeedbackPerformanceTime::Now,
        );
    }
}

/// True when the frontend should draw its own window-control buttons
/// (minimize / maximize / close) in the top chrome.
///
/// Linux runs without native decorations (`set_decorations(false)` in setup),
/// so desktops whose windows conventionally carry titlebar buttons (GNOME,
/// KDE, XFCE, …) need client-side replacements. Tiling compositors (Hyprland,
/// sway, river, niri, i3, …) manage windows from the keyboard and draw no
/// buttons, so none are shown there. macOS keeps its native traffic lights
/// and Windows keeps native decorations — both return false.
#[tauri::command]
pub fn needs_client_window_controls() -> bool {
    #[cfg(target_os = "linux")]
    {
        if std::env::var_os("HYPRLAND_INSTANCE_SIGNATURE").is_some()
            || std::env::var_os("SWAYSOCK").is_some()
        {
            return false;
        }

        const TILING_DESKTOPS: &[&str] = &[
            "hyprland", "sway", "river", "niri", "i3", "bspwm", "dwm", "awesome", "xmonad", "qtile",
        ];
        // XDG_CURRENT_DESKTOP is a colon-separated list (e.g. "ubuntu:GNOME").
        let desktop = std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("XDG_SESSION_DESKTOP"))
            .unwrap_or_default()
            .to_lowercase();
        !desktop
            .split(':')
            .any(|segment| TILING_DESKTOPS.contains(&segment.trim()))
    }

    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

/// Performs the window action matching the macOS "double-click a window's
/// title bar to" preference (`AppleActionOnDoubleClick`).
///
/// macOS values are `Minimize`, `Maximize` (default when unset), `Fill`, or
/// `None`.
/// The desktop app uses a web-based title-bar drag region, so the frontend
/// forwards double-clicks here and suppresses Tauri's injected drag-region
/// handler, whose default macOS path hardcodes maximize.
///
/// For `Fill`, resize to the current monitor work area instead of using
/// Tauri's maximize path, which maps to macOS zoom for titled, resizable
/// windows.
///
/// On non-macOS platforms this always toggles maximize (the historical
/// behavior).
#[tauri::command]
pub fn title_bar_double_click(window: tauri::Window) {
    #[cfg(target_os = "macos")]
    {
        let action = {
            let output = std::process::Command::new("defaults")
                .args(["read", "-g", "AppleActionOnDoubleClick"])
                .output();
            match output {
                Ok(output) if output.status.success() => {
                    String::from_utf8_lossy(&output.stdout).trim().to_string()
                }
                _ => "Maximize".to_string(),
            }
        };

        match action.as_str() {
            "None" => {}
            "Minimize" => {
                let _ = window.minimize();
            }
            "Fill" => {
                fill_window(&window);
            }
            // "Maximize" or any unexpected value.
            _ => {
                toggle_maximize(&window);
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        toggle_maximize(&window);
    }
}

/// Fills the current display work area, excluding system UI like the menu bar
/// and Dock.
#[cfg(target_os = "macos")]
fn fill_window(window: &tauri::Window) {
    match window.current_monitor() {
        Ok(Some(monitor)) => {
            if window.is_maximized().unwrap_or(false) {
                let _ = window.unmaximize();
            }

            let work_area = monitor.work_area();
            let _ = window.set_position(work_area.position);
            let _ = window.set_size(work_area.size);
        }
        _ => {
            let _ = window.maximize();
        }
    }
}

/// Toggles the window between maximized and its previous size, matching the
/// historical double-click behavior.
fn toggle_maximize(window: &tauri::Window) {
    match window.is_maximized() {
        Ok(true) => {
            let _ = window.unmaximize();
        }
        _ => {
            let _ = window.maximize();
        }
    }
}
