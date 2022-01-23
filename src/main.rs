use crate::util::delta_time;
use crate::util::load_texture_file;
use macroquad::prelude::*;
use ::rand::prelude::*;

mod util;

pub const SCREEN_WIDTH: i32 = 320;
pub const SCREEN_HEIGHT: i32 = 256;
pub const COLORS: [Color; 9] = [
    Color {
        r: 40.0 / 255.0,
        g: 44.0 / 255.0,
        b: 60.0 / 255.0,
        a: 1.0,
    },
    Color {
        r: 115.0 / 255.0,
        g: 239.0 / 255.0,
        b: 232.0 / 255.0,
        a: 1.0,
    },
    Color {
        r: 66.0 / 255.0,
        g: 191.0 / 255.0,
        b: 232.0 / 255.0,
        a: 1.0,
    },
    Color {
        r: 39.0 / 255.0,
        g: 137.0 / 255.0,
        b: 205.0 / 255.0,
        a: 1.0,
    },
    Color {
        r: 43.0 / 255.0,
        g: 78.0 / 255.0,
        b: 149.0 / 255.0,
        a: 1.0,
    },
    Color {
        r: 69.0 / 255.0,
        g: 114.0 / 255.0,
        b: 227.0 / 255.0,
        a: 1.0,
    },
    Color {
        r: 73.0 / 255.0,
        g: 65.0 / 255.0,
        b: 130.0 / 255.0,
        a: 1.0,
    },
    Color {
        r: 120.0 / 255.0,
        g: 100.0 / 255.0,
        b: 198.0 / 255.0,
        a: 1.0,
    },
    Color {
        r: 27.0 / 255.0,
        g: 36.0 / 255.0,
        b: 71.0 / 255.0,
        a: 1.0,
    },
];

struct Game {
    pub placed_blocks: [[u8; 12]; 16],
    pub block: Block,
    pub game_over: bool,

    pub block_texture: Option<Texture2D>,
}

impl Game {
    async fn new() -> Game {
        Game {
            placed_blocks: [
                [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8]; 16
            ],
            block: Block::new(),
            game_over: false,

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

enum BlockShape {
    I, O, T, J, L, S, Z,
}

struct Block {
    position: Vec2,
    rotation: u8,
    block_shape: BlockShape,
    gravity_timer: f32,
    movement_timer: f32,
}

impl Block {
    fn new() -> Block {
        Block {
            position: vec2(5.0, 0.0),
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
            gravity_timer: 30.0,
            movement_timer: 0.0,
        }
    }

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
    game.block = Block::new();
    loop {
        if !update(&mut game).await {
            game.game_over = true;
        }

        let game_render_target = render_target(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
        set_camera(&Camera2D {
            zoom: vec2(1.0 / SCREEN_WIDTH as f32 * 2.0, 1.0 / SCREEN_HEIGHT as f32 * 2.0),
            target: vec2(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0),
            render_target: Some(game_render_target),
            ..Default::default()
        });
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
        for y in 0..16 {
            game.block.position.y = y as f32;
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
            game.block.movement_timer = 6.0;
            game.block.position.x += if is_key_down(KeyCode::Left) { -1.0 } else { 1.0 };
            if game.block_collides() {
                game.block.position.x -= if is_key_down(KeyCode::Left) { -1.0 } else { 1.0 };
            }
        }
    } else {
        game.block.movement_timer = 0.0;
    }

    game.block.gravity_timer -= delta_time();
    if game.block.gravity_timer <= 0.0 {
        if is_key_down(KeyCode::Down) {
            game.block.gravity_timer = 5.0;
        } else {
            game.block.gravity_timer = 30.0;
        }
        game.block.position.y += 1.0;

        let shape = game.block.get_shape();
        if game.block_collides() {
            for y in 0..shape.len() {
                for x in 0..shape[y].len() {
                    if shape[y][x] != 0 {
                        game.placed_blocks[y + game.block.position.y as usize - 1][x + game.block.position.x as usize] = shape[y][x];
                    }
                }   
            }
            game.block = Block::new();
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
    let shape = game.block.get_shape();
    for y in 0..shape.len() {
        for x in 0..shape[y].len() {
            if shape[y][x] != 0 {
                draw_texture(
                    game.block_texture.unwrap(),
                    (game.block.position.x + x as f32) * 16.0,
                    (game.block.position.y + y as f32) * 16.0,
                    COLORS[shape[y][x] as usize],
                );
            }
        }
    }
    if game.game_over {
        draw_text("GAME OVER!", 10.0, 10.0, 16.0, WHITE);
    }
}