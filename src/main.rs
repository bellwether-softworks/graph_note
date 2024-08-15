mod app;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Graph Notes",
        native_options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
