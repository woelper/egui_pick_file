use egui::Context;
use std::future::Future;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct BrowseApp {
    text_channel: (Sender<String>, Receiver<String>),
    sample_text: String,
}

impl Default for BrowseApp {
    fn default() -> Self {
        Self {
            text_channel: channel(),
            sample_text: "This is some sample text".into(),
        }
    }
}

impl BrowseApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        BrowseApp::default()
    }
}

impl eframe::App for BrowseApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // assign sample text once it comes in
        if let Ok(text) = self.text_channel.1.try_recv() {
            self.sample_text = text;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_multiline(&mut self.sample_text);
            // a simple button opening the dialog
            if ui.button("📂 Open text file").clicked() {
                let sender = self.text_channel.0.clone();
                let task = rfd::AsyncFileDialog::new().pick_file();
                // Context is wrapped in an Arc so it's cheap to clone as per:
                // > Context is cheap to clone, and any clones refers to the same mutable data (Context uses refcounting internally).
                // Taken from https://docs.rs/egui/0.24.1/egui/struct.Context.html
                let ctx = ui.ctx().clone();
                execute(async move {
                    let file = task.await;
                    if let Some(file) = file {
                        let text = file.read().await;
                        let _ = sender.send(String::from_utf8_lossy(&text).to_string());
                        ctx.request_repaint();
                    }
                });
            }

            if ui.button("💾 Save text to file").clicked() {
                let task = rfd::AsyncFileDialog::new().save_file();
                let contents = self.sample_text.clone();
                execute(async move {
                    let file = task.await;
                    if let Some(file) = file {
                        _ = file.write(contents.as_bytes()).await;
                    }
                });
            }
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    // this is stupid... use any executor of your choice instead
    std::thread::spawn(move || futures::executor::block_on(f));
}

#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
