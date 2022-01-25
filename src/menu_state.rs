use macroquad::prelude::*;
use crate::Game;
use crate::GameState;

pub fn update_menu(game: &mut Game) {
    if is_key_pressed(KeyCode::X) {
        game.state = GameState::Game;
    }
}

pub fn render_menu(_game: &Game) {
    draw_text("RS-tris", 95.0, 60.0 + (f32::sin(get_time() as f32 * 2.0) * 10.0).round(), 32.0, WHITE);
    draw_text("X to play", 110.0, 76.0 + (f32::sin(get_time() as f32 * 2.0) * 10.0).round(), 16.0, WHITE);
}