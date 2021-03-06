use eframe::{
	egui::{
		self, Button, Color32, Context, FontData, FontDefinitions, FontFamily, Hyperlink, Label, Layout, RichText, TextureHandle, TopBottomPanel, Ui, Window,
		TextStyle::{Body, Heading, Monospace}
	}
};
use std::{
	collections::HashMap
};

const MONOSPACE_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/monospace_font.ttf");
const REGULAR_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/regular_font.otf");
const PADDING: f32 = 5.0;

pub struct Values {
	general_messages: u64,
	counting_messages: u64,
	secret_area: bool,
	negacies_converted: u64,
	negacies_converted_toggle: bool,
	negacies_earned: u64,
	quacks: u64
}

impl Values {
	fn new() -> Self {
		Self {
			general_messages: 0,
			counting_messages: 0,
			secret_area: false,
			negacies_converted: 0,
			negacies_converted_toggle: true,
			negacies_earned: 0,
			quacks: 0
		}
	}
}

pub struct Calculator {
	pub textures: HashMap<String, TextureHandle>,
	pub textures_loaded: bool,
	pub info_active: bool,
	build_date: Vec<String>,
	version: String,
	general_legacies: Vec<u64>,
	counting_legacies: Vec<u64>,
	secret_area_cost: u64,
	quacker_roles: Vec<u64>,
	quacker_roles_names: Vec<String>,
	values: Values
}

impl Calculator {
	pub fn new(build_date: Vec<String>, version: String, general_legacies: Vec<u64>, counting_legacies: Vec<u64>, secret_area_cost: u64, quacker_roles: Vec<u64>, quacker_roles_names: Vec<String>) -> Self {
		Self {
			textures: HashMap::new(),
			textures_loaded: false,
			info_active: false,
			build_date,
			version,
			general_legacies,
			counting_legacies,
			secret_area_cost,
			quacker_roles,
			quacker_roles_names,
			values: Values::new()
		}
	}

	pub fn initialise_fonts(&self, ctx: &Context) {
		let mut fonts = FontDefinitions::default();
        fonts.font_data.insert("monospace".to_owned(), FontData::from_static(MONOSPACE_FONT_BYTES));
        fonts.font_data.insert("regular".to_owned(), FontData::from_static(REGULAR_FONT_BYTES));
		fonts.families.get_mut(&FontFamily::Monospace).unwrap()
    		.insert(0, "monospace".to_owned());
		fonts.families.get_mut(&FontFamily::Proportional).unwrap()
			.insert(0, "regular".to_owned());
		fonts.families.get_mut(&FontFamily::Proportional).unwrap()
			.push("monospace".to_owned());
		fonts.families.get_mut(&FontFamily::Monospace).unwrap()
			.push("regular".to_owned());
        ctx.set_fonts(fonts);

		let mut customized_text_styles = ctx.style().text_styles.clone();

		customized_text_styles.insert(
		Body, egui::FontId::new(16.0, FontFamily::Proportional)
		);
		customized_text_styles.insert(
		Heading, egui::FontId::new(24.0, FontFamily::Proportional)
		);
		ctx.set_style(egui::Style {
			text_styles: customized_text_styles,
			..Default::default()
		});
	}

	pub fn render_footer(&self, ctx: &Context) {
		TopBottomPanel::bottom("footer").show(ctx, |ui| {
			ui.vertical_centered(|ui| {
				ui.add_space(PADDING);
				ui.add(Hyperlink::from_label_and_url(RichText::new("App source code | Made by Pandicon").text_style(Monospace), "https://github.com/Pandicon/Alias-Heaven-Calculator"));
				ui.add_space(PADDING);
			});
		});
	}

	pub fn render_top_panel(&mut self, ctx: &Context) {
		TopBottomPanel::top("top_panel").frame(self.default_frame()).show(ctx, |ui| {
            ui.add_space(PADDING);
			egui::menu::bar(ui, |ui| {
				ui.with_layout(Layout::left_to_right(), |ui| {
					ui.add(Label::new(RichText::new(format!("{}", self.name())).text_style(Heading)));
				});
				ui.with_layout(Layout::right_to_left(), |ui| {
					let info_btn = ui.add(Button::new(RichText::new("???").text_style(Body)));
					if info_btn.clicked() {
						self.info_active = true;
					}
				});
			});
            ui.add_space(PADDING);
		});
	}

	pub fn name(&self) -> &str {
		"Alias' Heaven Calculator"
	}

	pub fn render_info(&mut self, ctx: &Context) {
		Window::new("Information").show(ctx, |ui| {
			ui.label(RichText::new("App information").text_style(Heading).size(20.0));
			ui.horizontal(|ui| {
				ui.label("App by:");
				ui.add(Hyperlink::from_label_and_url("Pandicon", "https://github.com/Pandicon"));
			});
			ui.horizontal(|ui| {
				ui.label("Game source code:");
				ui.add(Hyperlink::from_label_and_url("https://github.com/Pandicon/Alias-Heaven-Calculator", "https://github.com/Pandicon/Alias-Heaven-Calculator"));
			});
			ui.label("");
			ui.label(RichText::new("Build information").text_style(Heading).size(20.0));
			ui.label(format!("Version: {}", self.version));
			let bd = &self.build_date;
			ui.label(format!("Built on {}/{}/{} at {}:{}:{} UTC", bd[2], bd[1], bd[0], bd[3], bd[4], bd[5]));
			ui.horizontal(|ui| {
				let close_btn = ui.add(Button::new(RichText::new("Close").text_style(Body)));
				if close_btn.clicked() {
					self.info_active = false;
				}
			});
		});
	}
	
	fn render_leg_neg_conversion(&mut self, ui: &mut Ui) {
		ui.label(RichText::new("Legacy and negacy roles converting").text_style(Heading).size(20.0));
		ui.horizontal(|ui| {
			ui.checkbox(&mut self.values.negacies_converted_toggle, "");
			ui.label(RichText::new("Did you convert negacy roles to legacy roles?").text_style(Body));
		});
		ui.horizontal(|ui| {
			ui.add(egui::DragValue::new(&mut self.values.negacies_converted).speed(0.01));
			ui.label(RichText::new(if self.values.negacies_converted_toggle { 
				"Negacy roles you converted into legacy ones (you can only do that once you max out your negacy roles)"
			} else {
				"Legacy roles you converted into negacy ones (you can only do that once you max out your legacy roles)"
			}).text_style(Body));
		});
	}
	
	fn render_legacies(&mut self, ui: &mut Ui) {
		ui.label(RichText::new("Legacy roles").text_style(Heading).size(20.0));
		ui.horizontal(|ui| {
			ui.add(egui::DragValue::new(&mut self.values.general_messages).speed(10));
			ui.label(RichText::new("Messages you sent in the general chat").text_style(Body));
		});
		ui.horizontal(|ui| {
			ui.add(egui::DragValue::new(&mut self.values.counting_messages).speed(10));
			ui.label(RichText::new("Valid messages you sent in the counting channel").text_style(Body));
		});
		ui.horizontal(|ui| {
			ui.checkbox(&mut self.values.secret_area, "");
			ui.label(RichText::new("Do you have access to the secret area?").text_style(Body));
		});
		let mut legacies = (self.values.negacies_converted as i64) * (if self.values.negacies_converted_toggle {
			1
		} else {
			-1
		});
		for n in &self.general_legacies {
			if &self.values.general_messages < n {
				break;
			}
			legacies += 1;
		}
		for n in &self.counting_legacies {
			if &self.values.counting_messages < n {
				break;
			}
			legacies += 1;
		}
		if self.values.secret_area {
			legacies -= self.secret_area_cost as i64;
		}
		ui.label(RichText::new(format!("Final role: Legacy {}", legacies)).text_style(Body).size(18.0));
	}
	
	fn render_negacies(&mut self, ui: &mut Ui) {
		ui.label(RichText::new("Negacy roles").text_style(Heading).size(20.0));
		ui.horizontal(|ui| {
			ui.add(egui::DragValue::new(&mut self.values.negacies_earned).speed(0.01));
			ui.label(RichText::new("Negacy roles you earned").text_style(Body));
		});
		let negacies = (self.values.negacies_earned as i64) + (self.values.negacies_converted as i64) * if self.values.negacies_converted_toggle {
			-1
		} else {
			1
		};
		ui.label(RichText::new(format!("Final role: Negacy {}", negacies)).text_style(Body).size(18.0));
	}

	fn render_quacks(&mut self, ui: &mut Ui) {
		ui.label(RichText::new("Quacker roles").text_style(Heading).size(20.0));
		ui.horizontal(|ui| {
			ui.add(egui::DragValue::new(&mut self.values.quacks).speed(10));
			ui.label(RichText::new("Valid quacks you sent").text_style(Body));
		});
		let mut quack_role = 0;
		for n in &self.quacker_roles {
			if &self.values.quacks < n {
				break;
			}
			quack_role += 1;
		}
		let quack_role = &self.quacker_roles_names[quack_role];
		ui.label(RichText::new(format!("Final role: {}", quack_role)).text_style(Body).size(18.0));
	}

	pub fn render_window(&mut self, ui: &mut Ui) {
		self.render_leg_neg_conversion(ui);
		ui.label("");
		self.render_legacies(ui);
		ui.label("");
		self.render_negacies(ui);
		ui.label("");
		self.render_quacks(ui)
	}

	pub fn default_frame(&self) -> egui::Frame {
		egui::Frame::default().stroke(egui::Stroke::new(2.0, Color32::from_rgb(43, 43, 43))).margin(egui::style::Margin::symmetric(7.0, 2.0)).fill(Color32::from_rgb(27, 27, 27))
	}
}