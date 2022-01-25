use std::env;
use macroquad::prelude::*;
use macroquad::audio;
use macroquad::audio::Sound;

pub fn clamp_range<T: std::cmp::PartialOrd>(min: T, val: T, max: T) -> T {
	if val > max { max }
	else if val < min { min }
	else { val }
}

pub fn delta_time() -> f32 { get_frame_time() * 60.0 }

pub fn get_file_path(path: String) -> String {
	return if cfg!(wasm32_unknown_unknown) {
		path
	} else {
		let mut full_path = env::current_exe().unwrap();
		full_path.pop();
		full_path.push(&path);
		full_path.as_os_str().to_str().unwrap().to_string()
	}
}

pub async fn load_texture_file(file_path: String) -> Texture2D {
	load_texture(&get_file_path(file_path)).await.unwrap()
}

pub async fn load_sound_file(file_path: String) -> Sound {
	audio::load_sound(&get_file_path(file_path)).await.unwrap()
}