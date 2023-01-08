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
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT // Make sure we don't paint anything behind the rounded corners
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        CentralPanel::default()
            .frame(Frame::none())
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let painter = ui.painter();
                const TITLE_HEIGHT: f32 = 32.0;

                // Paint the frame background
                #[cfg(not(target_arch = "wasm32"))]
                painter.rect(
                    rect.shrink(1.0),
                    8.0,
                    ctx.style().visuals.window_fill(),
                    Stroke::new(1.0, ctx.style().visuals.text_color()),
                );
                painter.text(
                    rect.center_top() + vec2(0.0, TITLE_HEIGHT / 2.0),
                    Align2::CENTER_CENTER,
                    "测试应用",
                    FontId::proportional(TITLE_HEIGHT * 0.6),
                    ctx.style().visuals.text_color(),
                );

                let title_bar_rect = {
                    let mut rect = rect;
                    rect.max.y = rect.min.y + TITLE_HEIGHT;
                    rect
                };

                #[cfg(not(target_arch = "wasm32"))]
                {
                    let title_bar_response =
                        ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());
                    if title_bar_response.is_pointer_button_down_on() {
                        frame.drag_window();
                    }
                }

                let content_rect = {
                    let mut rect = rect;
                    rect.min.y = title_bar_rect.max.y;
                    rect
                }
                .shrink(4.0);
                let mut content_ui = ui.child_ui(content_rect, *ui.layout());

                #[cfg(not(target_arch = "wasm32"))]
                TopBottomPanel::top("top_panel").show_inside(&mut content_ui, |ui| {
                    ui.horizontal(|ui| {
                        // The top panel is often a good place for a menu bar:
                        menu::bar(ui, |ui| {
                            ui.menu_button("菜单", |ui| {
                                if ui.button("退出").clicked() {
                                    frame.close();
                                }
                            });
                        });
                    });
                });

                SidePanel::left("side_panel").show_inside(&mut content_ui, |ui| {
                    ui.heading("ばんざい！");

                    ui.horizontal(|ui| {
                        ui.label("输入测试：");
                        ui.text_edit_singleline(&mut self.label);
                    });

                    ui.add(Slider::new(&mut self.value, 0.0..=10.0));

                    if ui.button(RichText::new("+1").size(32.0)).clicked() {
                        self.value += 1.0;
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

                CentralPanel::default().show_inside(&mut content_ui, |ui| {
                    // The central panel the region left after adding TopPanel's and SidePanel's

                    ui.heading("测试");
                    ui.hyperlink("https://github.com/RA3CoronaDevelopers");
                    warn_if_debug_build(ui);

                    ui.add(Image::new(
                        self.image.texture_id(ctx),
                        self.image.size_vec2(),
                    ));
                });

                Window::new("Window").show(content_ui.ctx(), |ui| {
                    ui.label(format!(
                        "x: {}, y: {}",
                        self.image.size_vec2().x,
                        self.image.size_vec2().y
                    ));
                });
            });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
