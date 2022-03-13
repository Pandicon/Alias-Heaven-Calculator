use chrono::{Datelike, Timelike, Utc};
use const_gen::*;
use dotenv::dotenv;
use image;
use std::{env, fs::{self, File}, io::Read, path::Path};
use toml::Value;
use winres;

fn zero_nothing(num: i64) -> String {
    String::from(if num < 10 {
        "0"
    } else {
        ""
    })
}

fn to_u64_vec(arr: &Value) -> Vec<u64> {
    arr.as_array().unwrap().iter().map(|val| val.as_integer().unwrap() as u64).collect::<Vec<u64>>()
}

fn to_str_vec(arr: &Value) -> Vec<&str> {
    arr.as_array().unwrap().iter().map(|val| val.as_str().unwrap()).collect::<Vec<&str>>()
}

fn main() {
	dotenv().ok();
	let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("const_gen.rs");
	
    let icon = image::open("./assets//icon.png").unwrap();
    let icon_rgba: Vec<u8> = icon.to_rgba8().into_raw();

    let mut cargo_toml_file = File::open("./Cargo.toml").unwrap();
    let mut cargo_toml_contents = String::new();
    cargo_toml_file.read_to_string(&mut cargo_toml_contents).unwrap();
    let cargo_toml = cargo_toml_contents.parse::<Value>().unwrap();

    let mut config_toml_file = File::open("./config.toml").unwrap();
    let mut config_toml_contents = String::new();
    config_toml_file.read_to_string(&mut config_toml_contents).unwrap();
    let config_toml = config_toml_contents.parse::<Value>().unwrap();

    let curr_time = Utc::now();
    let date: Vec<String> = vec![
        format!("{}", curr_time.year()),
        format!("{}{}", zero_nothing(curr_time.month() as i64), curr_time.month()),
        format!("{}{}", zero_nothing(curr_time.day() as i64), curr_time.day()),
        format!("{}{}", zero_nothing(curr_time.hour() as i64), curr_time.hour()),
        format!("{}{}", zero_nothing(curr_time.minute() as i64), curr_time.minute()),
        format!("{}{}", zero_nothing(curr_time.second() as i64), curr_time.second()),
        format!("{}", curr_time.timestamp_millis())
    ];
	
	let const_declarations = vec!{
        const_declaration!(ICON_RGBA = icon_rgba),
        const_declaration!(ICON_WIDTH = icon.width()),
        const_declaration!(ICON_HEIGHT = icon.height()),
        const_declaration!(VERSION = cargo_toml["package"]["version"].as_str().unwrap()),
        const_declaration!(BUILD_DATE = date),
        const_declaration!(GENERAL_LEGACIES = to_u64_vec(&config_toml["general_legacies"])),
        const_declaration!(COUNTING_LEGACIES = to_u64_vec(&config_toml["counting_legacies"])),
        const_declaration!(SECRET_AREA_COST = (config_toml["secret_area_cost"].as_integer().unwrap() as u64)),
        const_declaration!(QUACKER_ROLES = to_u64_vec(&config_toml["quacker_roles"])),
        const_declaration!(QUACKER_ROLES_NAMES = to_str_vec(&config_toml["quacker_roles_names"]))
    }.join("\n");
    fs::write(&dest_path, const_declarations).unwrap();
	if cfg!(target_os = "windows") {
		let mut res = winres::WindowsResource::new();
		res.set_icon("./assets/icon.ico");
		res.compile().unwrap();
	}
}