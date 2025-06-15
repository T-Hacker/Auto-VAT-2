#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use arboard::Clipboard;
use eframe::egui::{
    self, Align, Color32, FontFamily, FontId, RichText, TextFormat, TextStyle, text::LayoutJob,
};

const VAT_RATE: f32 = 0.21; // Assuming a VAT rate of 21%
const VAT_PERCENT: u32 = (VAT_RATE * 100.0) as u32;

const PURPLE_HEART: &str = "\u{1F49C}";

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(600.0, 100.0))
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Auto-VAT-2",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    clipboard: Clipboard,
}

impl Default for MyApp {
    fn default() -> Self {
        let clipboard = Clipboard::new().unwrap();

        Self { clipboard }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown), // <- ♥
                |ui| match self.clipboard.get_text() {
                    Ok(text) => display_price_conversion(&text, ui),
                    Err(e) => match e {
                        arboard::Error::ContentNotAvailable => no_text_in_clipboard(ui),
                        _ => panic!("Failed to get text from clipboard: {}", e),
                    },
                },
            );

            ui.with_layout(egui::Layout::bottom_up(Align::RIGHT), |ui| {
                ui.heading(RichText::new(PURPLE_HEART).color(Color32::PURPLE));
            });
        });

        ctx.request_repaint_after_secs(1.0); // Repaint every second to check clipboard updates.
    }
}

fn no_text_in_clipboard(ui: &mut egui::Ui) {
    ui.label(
        RichText::new("Clipboard is empty or does not contain text.")
            .color(Color32::RED)
            .size(ui.text_style_height(&TextStyle::Heading)),
    );
}

fn display_price_conversion(price: &str, ui: &mut egui::Ui) {
    if let Some(price) = parse_text(price) {
        let total_price = price * (1.0 + VAT_RATE);

        let number_text_format = TextFormat {
            font_id: FontId::new(32.0, FontFamily::Monospace),
            ..Default::default()
        };

        let arrow_text_format = TextFormat {
            font_id: FontId::new(32.0, FontFamily::Monospace),
            color: Color32::YELLOW,
            ..Default::default()
        };

        let mut job = LayoutJob::default();
        job.append(&format!("{:.2}", price), 0.0, number_text_format.clone());
        job.append(
            &format!(" == +{}% ==> ", VAT_PERCENT),
            0.0,
            arrow_text_format,
        );
        job.append(&format!("{:.2}", total_price), 0.0, number_text_format);

        ui.label(job);
    } else {
        ui.heading(RichText::new("Invalid price format in clipboard.").color(Color32::RED));
    }
}

fn parse_text(text: &str) -> Option<f32> {
    if let Ok(number) = text.trim().parse::<f32>() {
        return Some(number);
    }

    let filtered_text = text.trim().replace('.', "").replace(',', ".");
    filtered_text.parse::<f32>().ok()
}
