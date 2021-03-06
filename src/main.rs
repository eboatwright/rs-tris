use macroquad::rand::gen_range;
use macroquad::audio::stop_sound;
use macroquad::audio::Sound;
use crate::background::*;
use crate::game_state::*;
use crate::menu_state::*;
use crate::block::Block;
use macroquad::audio::play_sound;
use crate::util::load_sound_file;
use macroquad::audio::PlaySoundParams;
use crate::util::load_texture_file;
use macroquad::prelude::*;

mod background;
mod block;
mod menu_state;
mod game_state;
mod util;

pub const SCREEN_WIDTH: i32 = 320;
pub const SCREEN_HEIGHT: i32 = 256;
pub const COLORS: [Color; 10] = [
    Color {
        r: 0.156,
        g: 0.172,
        b: 0.235,
        a: 1.0,
    },
    Color {
        r: 0.450,
        g: 0.937,
        b: 0.909,
        a: 1.0,
    },
    Color {
        r: 0.258,
        g: 0.749,
        b: 0.909,
        a: 1.0,
    },
    Color {
        r: 0.152,
        g: 0.537,
        b: 0.803,
        a: 1.0,
    },
    Color {
        r: 0.168,
        g: 0.305,
        b: 0.584,
        a: 1.0,
    },
    Color {
        r: 0.270,
        g: 0.447,
        b: 0.890,
        a: 1.0,
    },
    Color {
        r: 0.286,
        g: 0.254,
        b: 0.509,
        a: 1.0,
    },
    Color {
        r: 0.470,
        g: 0.392,
        b: 0.776,
        a: 1.0,
    },
    Color {
        r: 0.105,
        g: 0.141,
        b: 0.278,
        a: 1.0,
    },
    Color {
        r: 0.043,
        g: 0.109,
        b: 0.152,
        a: 1.0,
    },
];

#[derive(PartialEq)]
pub enum GameState {
    Menu,
    Game,
}

pub struct Game {
    pub time: f32,
    pub state: GameState,
    pub placed_blocks: [[u8; 12]; 16],
    pub block: Block,
    pub game_over: bool,
    pub next_block: Block,
    pub held_block: Option<Block>,

    pub block_texture: Option<Texture2D>,
    pub background_texture: Option<Texture2D>,

    pub music: Option<Sound>,
    pub play_sfx: Option<Sound>,
    pub game_over_sfx: Option<Sound>,
    pub hit_sfx: Option<Sound>,

    pub particles: Vec<Particle>,

    pub play: bool,
    pub menu_delay: f32,
    pub played_game_over: bool,
    pub has_switched: bool,

    pub screen_shake: Vec2,
}

impl Game {
    async fn new() -> Game {
        Game {
            time: 0.0,
            state: GameState::Game,
            placed_blocks: [[8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8]; 16],
            block: Block::default(),
            game_over: false,
            next_block: Block::default(),
            held_block: None,

            block_texture: Some(load_texture_file("res/img/block.png".to_string()).await),
            background_texture: Some(load_texture_file("res/img/background.png".to_string()).await),

            music: Some(load_sound_file("res/sfx/music.ogg".to_string()).await),
            play_sfx: Some(load_sound_file("res/sfx/play.ogg".to_string()).await),
            game_over_sfx: Some(load_sound_file("res/sfx/game_over.ogg".to_string()).await),
            hit_sfx: Some(load_sound_file("res/sfx/hit.ogg".to_string()).await),

            particles: Vec::new(),

            play: false,
            menu_delay: 30.0,
            played_game_over: false,
            has_switched: false,

            screen_shake: vec2(0.0, 0.0),
        }
    }

    fn block_collides(&self) -> bool {
        let shape = self.block.get_shape();
        for y in 0..shape.len() {
            for x in 0..shape[y].len() {
                if shape[y][x] != 0 {
                    if self.block.position.y as usize + y >= self.placed_blocks.len()
                    || self.block.position.x as usize + x >= self.placed_blocks[0].len()
                    || self.block.position.x + (x as f32) <= 0.0
                    || self.placed_blocks[y + self.block.position.y as usize][x + self.block.position.x as usize] != 0 {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn shake(&mut self) {
        self.screen_shake = vec2(gen_range(-10.0, 10.0), gen_range(-10.0, 10.0));
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "RS-tris".to_string(),
        window_width: SCREEN_WIDTH * 3,
        window_height: SCREEN_HEIGHT * 3,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let game_render_target = render_target(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    let mut camera = Camera2D {
        zoom: vec2(1.0 / SCREEN_WIDTH as f32 * 2.0, 1.0 / SCREEN_HEIGHT as f32 * 2.0),
        target: vec2(SCREEN_WIDTH as f32 * 0.5 - 16.0, SCREEN_HEIGHT as f32 * 0.5),
        render_target: Some(game_render_target),
        ..Default::default()
    };
    let mut game = Game::new().await;
    for _ in 0..30 {
        game.particles.push(Particle {
            position: vec2(camera.target.x + gen_range(-SCREEN_WIDTH as f32 * 0.5, SCREEN_WIDTH as f32 * 0.5) - 16.0, camera.target.y + gen_range(-SCREEN_HEIGHT as f32 * 0.5, SCREEN_HEIGHT as f32 * 0.5)),
            radius: gen_range(20.0, 40.0),
        });
    }
    game.state = GameState::Menu;
    game.block.position = vec2(5.0, 0.0);
    let mut is_other_frame = true;
    loop {
        game.time += get_frame_time();
        if game.time < 0.0 {
            game.time = 0.0;
        }
        is_other_frame = !is_other_frame;
        update_background(&mut game);
        if game.state == GameState::Game {
            if !update_game(&mut game) {
                game.game_over = true;
                if !game.played_game_over {
                    game.played_game_over = true;
                    stop_sound(game.music.unwrap());
                    play_sound(
                        game.game_over_sfx.unwrap(),
                        PlaySoundParams {
                            looped: false,
                            volume: 0.8,
                        },
                    );
                }
            }
        } else {
            update_menu(&mut game);
        }
        if is_other_frame {
            game.screen_shake *= -0.9;
        } else {
            game.screen_shake *= 0.9;
        }
        
        camera.target += game.screen_shake.round();
        set_camera(&camera);
        clear_background(BLACK);

        render_background(&game);
        if game.state == GameState::Game {
            render_game(&game);
        } else {
            render_menu(&game);
        }

        set_default_camera();

        let game_diff_w = screen_width() / SCREEN_WIDTH as f32;
        let game_diff_h = screen_height() / SCREEN_HEIGHT as f32;
        let aspect_diff = game_diff_w.min(game_diff_h);

        let scaled_game_size_w = SCREEN_WIDTH as f32 * aspect_diff;
        let scaled_game_size_h = SCREEN_HEIGHT as f32 * aspect_diff;

        let width_padding = (screen_width() - scaled_game_size_w) * 0.5;
        let height_padding = (screen_height() - scaled_game_size_h) * 0.5;

        clear_background(BLACK);

        game_render_target.texture.set_filter(FilterMode::Nearest);
        draw_texture_ex(
            game_render_target.texture,
            width_padding,
            height_padding,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(scaled_game_size_w, scaled_game_size_h)),
                ..Default::default()
            },
        );

        camera.target -= game.screen_shake.round();

        next_frame().await
    }
}