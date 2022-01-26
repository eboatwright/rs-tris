use macroquad::audio::PlaySoundParams;
use macroquad::audio::play_sound;
use crate::util::delta_time;
use macroquad::prelude::*;
use crate::Game;
use crate::GameState;

pub fn update_menu(game: &mut Game) {
    if game.play {
        game.menu_delay -= delta_time();
    }
    if is_key_pressed(KeyCode::X)
    && !game.play {
        game.play = true;
        game.shake();
        play_sound(
            game.play_sfx.unwrap(),
            PlaySoundParams {
                looped: false,
                volume: 0.9
            },
        );
    }
    if game.menu_delay <= 0.0 {
        game.state = GameState::Game;
        play_sound(
            game.music.unwrap(),
            PlaySoundParams {
                looped: true,
                volume: 0.4,
            },
        );
    }
}

pub fn render_menu(game: &Game) {
    draw_text("RS-tris", 95.0, 60.0 + (f32::sin(game.time as f32 * 2.0) * 10.0).round(), 32.0, WHITE);
    draw_text("X to play", 110.0, 76.0 + (f32::sin(game.time as f32 * 2.0) * 10.0).round(), 16.0, WHITE);
}