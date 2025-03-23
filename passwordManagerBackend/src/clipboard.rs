use arboard::Clipboard;
use std::io::Write;
use std::env;
use std::process::Command;

pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    if cfg!(target_os = "linux") {
        let is_wayland = env::var("WAYLAND_DISPLAY").is_ok();
        
        if is_wayland {
            // Use wl-copy for Wayland
            if Command::new("wl-copy").arg(text).status().is_ok() {
                return Ok(());
            } else {
                return Err("Failed to copy using wl-copy. Is wl-clipboard installed?".to_string());
            }
        } else {
            // Use xclip for X11
            if Command::new("xclip")
                .args(&["-selection", "clipboard"])
                .stdin(std::process::Stdio::piped())
                .spawn()
                .and_then(|mut child| child.stdin.as_mut().unwrap().write_all(text.as_bytes()))
                .is_ok()
            {
                return Ok(());
            } else {
                return Err("Failed to copy using xclip. Is xclip installed?".to_string());
            }
        }
    } else {
        // For Windows and macOS, use arboard
        let mut clipboard = Clipboard::new().map_err(|e| format!("Clipboard error: {}", e))?;
        clipboard.set_text(text).map_err(|e| format!("Clipboard error: {}", e))?;
        Ok(())
    }
}

