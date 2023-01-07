use egui::*;
use egui_extras::RetainedImage;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    #[serde(skip)]
    image: RetainedImage,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            image: RetainedImage::from_image_bytes("logo", include_bytes!("../assets/logo-s.png"))
                .unwrap(),
        }
    }
}

fn setup_custom_fonts(ctx: &Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "chn".to_owned(),
        FontData::from_static(include_bytes!("../assets/SourceHanSansCN-Regular.otf")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "chn".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("chn".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        setup_custom_fonts(&cc.egui_ctx);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let Self {
            label,
            value,
            image,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            menu::bar(ui, |ui| {
                ui.menu_button("菜单", |ui| {
                    if ui.button("退出").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("ばんざい！");

            ui.horizontal(|ui| {
                ui.label("输入测试：");
                ui.text_edit_singleline(label);
            });

            ui.add(Slider::new(value, 0.0..=10.0));

            if ui.button(RichText::new("+1").size(32.0)).clicked() {
                *value += 1.0;
            }

            ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("测试");
            ui.hyperlink("https://github.com/RA3CoronaDevelopers");
            warn_if_debug_build(ui);

            ui.add(Image::new(image.texture_id(ctx), image.size_vec2()));
        });

        if true {
            Window::new("Window").show(ctx, |ui| {
                ui.label(format!(
                    "x: {}, y: {}",
                    image.size_vec2().x,
                    image.size_vec2().y
                ));
            });
        }
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
