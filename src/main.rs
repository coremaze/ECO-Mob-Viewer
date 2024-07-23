

use eframe::{
    egui::{
        self, CentralPanel, Context, Grid, RichText, SidePanel, TopBottomPanel, Ui,
        Vec2, ViewportBuilder, Visuals,
    },
    NativeOptions,
};


mod eco;

fn stat_row(ui: &mut Ui, name: impl AsRef<str>, body: impl AsRef<str>) {
    ui.label(RichText::new(name.as_ref().to_string()).size(24.));
    ui.centered_and_justified(|ui| {
        ui.label(RichText::new(body.as_ref().to_string()).size(24.));
    });
    ui.end_row();
}

fn stat_grid(ui: &mut Ui) -> Grid {
    let available_width = ui.available_width();
    let num_columns = 2;
    let column_width = (available_width - (num_columns - 1) as f32) / num_columns as f32;

    Grid::new("grid")
        .min_col_width(column_width)
        .max_col_width(column_width)
        .spacing((10., 10.))
}

struct EcoApp {
    mob_id: Option<u32>,
    name: String,
    level: String,
    hp: String,
    def: String,
    mdef: String,
    exp: String,
    job_exp: String,
    fire: String,
    water: String,
    wind: String,
    earth: String,
    light: String,
    dark: String,
    weakness: String,
}

impl EcoApp {
    pub fn new() -> Self {
        Self {
            mob_id: None,
            name: "-".to_owned(),
            level: "-".to_owned(),
            hp: "-".to_owned(),
            def: "-".to_owned(),
            mdef: "-".to_owned(),
            exp: "-".to_owned(),
            job_exp: "-".to_owned(),
            fire: "-".to_owned(),
            water: "-".to_owned(),
            wind: "-".to_owned(),
            earth: "-".to_owned(),
            light: "-".to_owned(),
            dark: "-".to_owned(),
            weakness: "-".to_owned(),
        }
    }

    fn show(&self, ctx: &Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::light());
        // ctx.set_debug_on_hover(true);
        CentralPanel::default().show(ctx, |_ui| {
            TopBottomPanel::top("top panel").show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(RichText::new(&self.name).size(32.));
                });
            });
            TopBottomPanel::bottom("bottom panel").show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(RichText::new(&self.weakness).size(32.));
                });
            });
            CentralPanel::default().show(ctx, |_ui| {
                self.show_body(ctx, frame);
            });
        });
    }

    fn show_body(&self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |_ui| {
                CentralPanel::default().show(ctx, |ui| {
                    SidePanel::left("left panel")
                        .show_separator_line(false)
                        .exact_width(ui.available_width() * 0.5)
                        .resizable(false)
                        .show(ctx, |ui| {
                            self.stats_grid(ui);
                        });
                    SidePanel::right("right panel")
                        .show_separator_line(false)
                        .exact_width(ui.available_width() * 0.5)
                        .resizable(false)
                        .show(ctx, |ui| {
                            self.element_grid(ui);
                        });
                });
            });
    }

    fn stats_grid(&self, ui: &mut Ui) {
        stat_grid(ui).show(ui, |ui| {
            stat_row(ui, "Level", &self.level);
            stat_row(ui, "HP", &self.hp);
            stat_row(ui, "DEF", &self.def);
            stat_row(ui, "MDEF", &self.mdef);
            stat_row(ui, "EXP", &self.exp);
            stat_row(ui, "Job EXP", &self.job_exp);
        });
    }

    fn element_grid(&self, ui: &mut Ui) {
        stat_grid(ui).show(ui, |ui| {
            stat_row(ui, "Fire", &self.fire);
            stat_row(ui, "Water", &self.water);
            stat_row(ui, "Wind", &self.wind);
            stat_row(ui, "Earth", &self.earth);
            stat_row(ui, "Light", &self.light);
            stat_row(ui, "Dark", &self.dark);
        });
    }

    fn update_monster(&mut self) {
        let Some(mob_id) = eco::get_eco().and_then(|handle| eco::get_hovered_monster_id(&handle))
        else {
            return;
        };

        if Some(mob_id) == self.mob_id {
            return;
        }

        let Some(monster) = eco::monster_by_id(mob_id) else {
            return;
        };

        self.name = monster.name.to_string();
        self.level = monster.level.to_string();
        self.hp = monster.hp.to_string();
        self.def = format!("{} - {}", monster.def[0], monster.def[1]);
        self.mdef = format!("{} - {}", monster.mdef[0], monster.mdef[1]);
        self.exp = monster.exp.to_string();
        self.job_exp = monster.job.to_string();
        self.fire = monster.properties.fire.to_string();
        self.water = monster.properties.water.to_string();
        self.wind = monster.properties.wind.to_string();
        self.earth = monster.properties.earth.to_string();
        self.light = monster.properties.light.to_string();
        self.dark = monster.properties.dark.to_string();
        self.weakness = weakness(&monster.properties);
    }
}

impl eframe::App for EcoApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        self.update_monster();
        self.show(ctx, frame);
        ctx.request_repaint();
    }
}

fn weakness(properties: &eco::Properties) -> String {
    let Some(&most) = [
        &properties.fire,
        &properties.water,
        &properties.wind,
        &properties.earth,
        &properties.light,
        &properties.dark,
    ]
    .iter()
    .max() else {
        return "-".to_string();
    };

    if *most == 0 {
        return "No weaknesses".to_string();
    }

    let weak_to = {
        if most == &properties.fire {
            "wind"
        } else if most == &properties.water {
            "fire"
        } else if most == &properties.wind {
            "earth"
        } else if most == &properties.earth {
            "water"
        } else if most == &properties.light {
            "all"
        } else if most == &properties.dark {
            "light"
        } else {
            "?"
        }
    };

    format!("Weak against {weak_to} ({most}%)")
}

fn main() -> eframe::Result {
    eframe::run_native(
        "ECO Mob Viewer",
        NativeOptions {
            viewport: ViewportBuilder {
                inner_size: Some(Vec2 { x: 400.0, y: 400.0 }),
                resizable: Some(false),
                ..ViewportBuilder::default()
            },
            follow_system_theme: false,

            ..NativeOptions::default()
        },
        Box::new(|_cc| Ok(Box::new(EcoApp::new()))),
    )
}
