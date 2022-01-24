use crate::util::delta_time;
use crate::util::load_texture_file;
use macroquad::prelude::*;
use ::rand::prelude::*;

mod util;

pub const SCREEN_WIDTH: i32 = 320;
pub const SCREEN_HEIGHT: i32 = 256;
pub const COLORS: [Color; 9] = [
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
];

struct Game {
    placed_blocks: [[u8; 12]; 16],
    block: Block,
    game_over: bool,
    next_block: Block,

    block_texture: Option<Texture2D>,
}

impl Game {
    async fn new() -> Game {
        Game {
            placed_blocks: [
                [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8]; 16
            ],
            block: Block::default(),
            game_over: false,
            next_block: Block::default(),

            block_texture: Some(load_texture_file("res/img/block.png".to_string()).await),
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
}

#[derive(Clone)]
enum BlockShape {
    I, O, T, J, L, S, Z,
}

#[derive(Clone)]
struct Block {
    position: Vec2,
    rotation: u8,
    block_shape: BlockShape,
    gravity_timer: f32,
    movement_timer: f32,
}

impl Default for Block {
    fn default() -> Block {
        Block {
            position: vec2(12.0, 1.0),
            rotation: 0,
            block_shape: match thread_rng().gen_range(0..7) {
                0 => BlockShape::I,
                1 => BlockShape::O,
                2 => BlockShape::T,
                3 => BlockShape::J,
                4 => BlockShape::L,
                5 => BlockShape::S,
                6 => BlockShape::Z,
                _ => BlockShape::I,
            },
            gravity_timer: 45.0,
            movement_timer: 0.0,
        }
    }
}

impl Block {
    fn get_shape(&self) -> [[u8; 4]; 4] {
        return match self.block_shape {
            BlockShape::I => {
                match self.rotation {
                    0 => {
                        [
                            [0, 1, 0, 0],
                            [0, 1, 0, 0],
                            [0, 1, 0, 0],
                            [0, 1, 0, 0],
                        ]
                    },
                    1 => {
                        [
                            [0, 0, 0, 0],
                            [1, 1, 1, 1],
                            [0, 0, 0, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    2 => {
                        [
                            [0, 0, 1, 0],
                            [0, 0, 1, 0],
                            [0, 0, 1, 0],
                            [0, 0, 1, 0],
                        ]
                    },
                    3 => {
                        [
                            [0, 0, 0, 0],
                            [0, 0, 0, 0],
                            [1, 1, 1, 1],
                            [0, 0, 0, 0],
                        ]
                    },
                    _ => panic!("Block rotation overflow!"),
                }
            },
            BlockShape::O => {
                match self.rotation {
                    0 | 1 | 2 | 3 => {
                        [
                            [0, 0, 0, 0],
                            [0, 2, 2, 0],
                            [0, 2, 2, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    _ => panic!("Block rotation overflow!"),
                }
            },
            BlockShape::T => {
                match self.rotation {
                    0 => {
                        [
                            [0, 3, 0, 0],
                            [3, 3, 3, 0],
                            [0, 0, 0, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    1 => {
                        [
                            [0, 3, 0, 0],
                            [0, 3, 3, 0],
                            [0, 3, 0, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    2 => {
                        [
                            [0, 0, 0, 0],
                            [3, 3, 3, 0],
                            [0, 3, 0, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    3 => {
                        [
                            [0, 3, 0, 0],
                            [3, 3, 0, 0],
                            [0, 3, 0, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    _ => panic!("Block rotation overflow!"),
                }
            },
            BlockShape::J => {
                match self.rotation {
                    0 => {
                        [
                            [0, 4, 0, 0],
                            [0, 4, 0, 0],
                            [4, 4, 0, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    1 => {
                        [
                            [4, 0, 0, 0],
                            [4, 4, 4, 0],
                            [0, 0, 0, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    2 => {
                        [
                            [0, 4, 4, 0],
                            [0, 4, 0, 0],
                            [0, 4, 0, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    3 => {
                        [
                            [0, 0, 0, 0],
                            [4, 4, 4, 0],
                            [0, 0, 4, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    _ => panic!("Block rotation overflow!"),
                }
            },
            BlockShape::L => {
                match self.rotation {
                    0 => {
                        [
                            [0, 5, 0, 0],
                            [0, 5, 0, 0],
                            [0, 5, 5, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    1 => {
                        [
                            [0, 0, 0, 0],
                            [5, 5, 5, 0],
                            [5, 0, 0, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    2 => {
                        [
                            [5, 5, 0, 0],
                            [0, 5, 0, 0],
                            [0, 5, 0, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    3 => {
                        [
                            [0, 0, 5, 0],
                            [5, 5, 5, 0],
                            [0, 0, 0, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    _ => panic!("Block rotation overflow!"),
                }
            },
            BlockShape::S => {
                match self.rotation {
                    0 => {
                        [
                            [0, 6, 6, 0],
                            [6, 6, 0, 0],
                            [0, 0, 0, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    1 => {
                        [
                            [0, 6, 0, 0],
                            [0, 6, 6, 0],
                            [0, 0, 6, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    2 => {
                        [
                            [0, 0, 0, 0],
                            [0, 6, 6, 0],
                            [6, 6, 0, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    3 => {
                        [
                            [6, 0, 0, 0],
                            [6, 6, 0, 0],
                            [0, 6, 0, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    _ => panic!("Block rotation overflow!"),
                }
            },
            BlockShape::Z => {
                match self.rotation {
                    0 => {
                        [
                            [7, 7, 0, 0],
                            [0, 7, 7, 0],
                            [0, 0, 0, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    1 => {
                        [
                            [0, 0, 7, 0],
                            [0, 7, 7, 0],
                            [0, 7, 0, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    2 => {
                        [
                            [0, 0, 0, 0],
                            [7, 7, 0, 0],
                            [0, 7, 7, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    3 => {
                        [
                            [0, 7, 0, 0],
                            [7, 7, 0, 0],
                            [7, 0, 0, 0],
                            [0, 0, 0, 0],
                        ]
                    },
                    _ => panic!("Block rotation overflow!"),
                }
            },
        }
    }

    fn render(&self, game: &Game) {
        let shape = self.get_shape();
        for y in 0..shape.len() {
            for x in 0..shape[y].len() {
                if shape[y][x] != 0 {
                    draw_texture(
                        game.block_texture.unwrap(),
                        (self.position.x + x as f32) * 16.0,
                        (self.position.y + y as f32) * 16.0,
                        COLORS[shape[y][x] as usize],
                    );
                }
            }
        }
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
    let mut game = Game::new().await;
    game.block = Block {
        position: vec2(5.0, 0.0),
        ..Default::default()
    };
    let game_render_target = render_target(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    let camera = Camera2D {
        zoom: vec2(1.0 / SCREEN_WIDTH as f32 * 2.0, 1.0 / SCREEN_HEIGHT as f32 * 2.0),
        target: vec2(SCREEN_WIDTH as f32 * 0.5 - 16.0, SCREEN_HEIGHT as f32 * 0.5),
        render_target: Some(game_render_target),
        ..Default::default()
    };
    loop {
        if !update(&mut game).await {
            game.game_over = true;
        }
        
        set_camera(&camera);
        clear_background(BLACK);

        render(&game);

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

        next_frame().await
    }
}

async fn update(game: &mut Game) -> bool {
    if game.game_over {
        if is_key_pressed(KeyCode::X) {
            *game = Game::new().await;
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
            game.block.position.y -= 1.0;
            for y in 0..shape.len() {
                for x in 0..shape[y].len() {
                    if shape[y][x] != 0 {
                        game.placed_blocks[y + game.block.position.y as usize][x + game.block.position.x as usize] = shape[y][x];
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
            for i in (1..=y).rev() {
                game.placed_blocks[i] = game.placed_blocks[i - 1];
            }
        }
    }

    let mut lost = false;
    for x in 1..game.placed_blocks[0].len() - 1 {
        if game.placed_blocks[0][x] != 0 {
            lost = true;
            break;
        }
    }
    if lost {
        return false;
    }
    true
}

fn render(game: &Game) {
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
    if game.game_over {
        draw_text("GAME OVER!", 28.0, 20.0, 32.0, WHITE);
        draw_text("X to play again?", 40.0, 36.0, 16.0, WHITE);
    }
}