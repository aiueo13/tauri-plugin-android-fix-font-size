# Overview

On Android, the system font size settings can cause the text size set in the frontend to become unintended.
Even if you specify 14px, the actual size may become 20px or 10px, and this cannot be avoided on the frontend.

This plugin prevents that and ensures that the correct font size is always used.

# Setup

All you need to do is register this plugin with your Tauri project: 

`src-tauri/src/lib.rs`

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_android_fix_font_size::init()) // This
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

# License
MIT OR Apache-2.0