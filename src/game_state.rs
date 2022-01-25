use macroquad::audio::play_sound;
use macroquad::audio::PlaySoundParams;
use crate::COLORS;
use crate::Block;
use crate::util::clamp_range;
use macroquad::prelude::*;
use crate::Game;
use crate::util::delta_time;
use crate::GameState;

pub fn update_game(game: &mut Game) -> bool {
    game.block.lerp_position();
    game.next_block.lerp_position();
    if game.game_over {
        if is_key_pressed(KeyCode::X) {
            *game = Game {
                state: GameState::Game,
                placed_blocks: [[8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8]; 16],
                block: Block {
                    position: vec2(5.0, 0.0),
                    ..Default::default()
                },
                game_over: false,
                next_block: Block::default(),

                particles: game.particles.clone(),

                played_game_over: false,
                ..*game
            };
            play_sound(
                game.music.unwrap(),
                PlaySoundParams {
                    looped: true,
                    volume: 0.4,
                },
            );
            return true;
        }
        return false;
    }
    if is_key_pressed(KeyCode::Z) {
        game.block.rotation += 1;
        if game.block.rotation > 3 {
            game.block.rotation = 0;
        }
        if game.block_collides() {
            if game.block.rotation == 0 {
                game.block.rotation = 3;
            } else {
                game.block.rotation -= 1;
            }
        }
    }
    if is_key_pressed(KeyCode::X) {
        for _ in game.block.position.y as usize..16 {
            game.block.position.y += 1.0;
            if game.block_collides() {
                game.block.position.y -= 1.0;
                game.block.gravity_timer = 0.0;
                game.block.movement_timer = 6.0;
                break;
            }
        }
    }
    if is_key_down(KeyCode::Left)
    || is_key_down(KeyCode::Right) {
        game.block.movement_timer -= delta_time();
        if game.block.movement_timer <= 0.0 {
            game.block.movement_timer = 7.0;
            game.block.position.x += if is_key_down(KeyCode::Left) { -1.0 } else { 1.0 };
            if game.block_collides() {
                game.block.position.x -= if is_key_down(KeyCode::Left) { -1.0 } else { 1.0 };
            }
        }
    } else {
        game.block.movement_timer = 0.0;
    }

    game.block.gravity_timer -= delta_time();
    if game.block.gravity_timer <= 0.0
    || is_key_pressed(KeyCode::Down) {
        if is_key_down(KeyCode::Down) {
            game.block.gravity_timer = 5.0;
        } else {
            game.block.gravity_timer = 45.0;
        }
        game.block.position.y += 1.0;

        let shape = game.block.get_shape();
        if game.block_collides() {
            play_sound(
                game.hit_sfx.unwrap(),
                PlaySoundParams {
                    looped: false,
                    volume: 0.85,
                },
            );
            game.block.position.y -= 1.0;
            for y in 0..shape.len() {
                for x in 0..shape[y].len() {
                    if shape[y][x] != 0 {
                        game.placed_blocks[y + clamp_range(0.0, game.block.position.y, 20.0) as usize][x + clamp_range(0.0, game.block.position.x, 20.0) as usize] = shape[y][x];
                    }
                }   
            }
            game.block = Block {
                position: vec2(5.0, 0.0),
                block_shape: game.next_block.block_shape.clone(),
                ..Default::default()
            };
            game.next_block = Block::default();
        }
    }

    for y in 0..game.placed_blocks.len() {
        let mut is_full_line = true;
        for x in 1..game.placed_blocks[y].len() - 1 {
            if game.placed_blocks[y][x] == 0 {
                is_full_line = false;
            }
        }
        if is_full_line {
            play_sound(
                game.clear_line_sfx.unwrap(),
                PlaySoundParams {
                    looped: false,
                    volume: 1.1,
                },
            );
             for i in (1..=y).rev() {
                game.placed_blocks[i] = game.placed_blocks[i - 1];
            }
        }
    }

    for x in 1..game.placed_blocks[0].len() - 1 {
        if game.placed_blocks[0][x] != 0 {
            return false;
        }
    }
    true
}

pub fn render_game(game: &Game) {
    for y in 0..game.placed_blocks.len() {
        for x in 0..game.placed_blocks[y].len() {
            draw_texture(
                game.block_texture.unwrap(),
                x as f32 * 16.0,
                y as f32 * 16.0,
                COLORS[game.placed_blocks[y][x] as usize]
            );
        }
    }
    game.block.render(game);
    game.next_block.render(game);
    draw_text("Next:", 215.0, 12.0, 16.0, WHITE);
    if game.game_over {
        draw_text("GAME OVER!", 28.0, 25.0, 32.0, WHITE);
        draw_text("X to play again?", 40.0, 41.0, 16.0, WHITE);
    }
}