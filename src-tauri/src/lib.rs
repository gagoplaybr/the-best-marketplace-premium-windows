#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.close();
            }
            WebviewWindowBuilder::new(
                app,
                "premium",
                WebviewUrl::External("https://thebestdigital.com.br/?app=desktop".parse().unwrap()),
            )
            .title("THE BEST MARKETPLACE PREMIUM")
            .inner_size(1366.0, 820.0)
            .min_inner_size(1100.0, 680.0)
            .center()
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("erro ao iniciar THE BEST MARKETPLACE PREMIUM");
}
