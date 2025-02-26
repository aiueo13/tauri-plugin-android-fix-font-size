use tauri::{plugin::{Builder, TauriPlugin}, Runtime};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    #[allow(unused_variables)]
    Builder::new("android-fix-font-size")
        .setup(|_, api| {
            #[cfg(mobile)] 
            api.register_android_plugin("com.plugin.android_fix_font_size", "FixFontSize")?;
            Ok(())
        })
        .build()
}