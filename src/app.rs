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
            sample_text: "yo".into(),
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
        if let Ok(f) = self.text_channel.1.try_recv() {
            self.sample_text = f;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&self.sample_text);
            // a simple button opening the dialog
            if ui.button("Open text file").clicked() {
                let sender = self.text_channel.0.clone();
                let task = rfd::AsyncFileDialog::new().pick_file();
                execute(async move {
                    let file = task.await;
                    if let Some(file) = file {
                        let text = file.read().await;
                        let _ = sender.send(String::from_utf8_lossy(&text).to_string());
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
