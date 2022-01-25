use macroquad::prelude::*;
use crate::COLORS;
use crate::Game;
use ::rand::Rng;
use ::rand::thread_rng;

#[derive(Copy, Clone)]
pub enum BlockShape {
    I, O, T, J, L, S, Z,
}

#[derive(Copy, Clone)]
pub struct Block {
    pub position: Vec2,
    pub render_position: Vec2,
    pub rotation: u8,
    pub block_shape: BlockShape,
    pub gravity_timer: f32,
    pub movement_timer: f32,
}

impl Default for Block {
    fn default() -> Block {
        Block {
            position: vec2(13.0, 1.0),
            render_position: vec2(13.0 * 16.0, 16.0),
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
    pub fn get_shape(&self) -> [[u8; 4]; 4] {
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

    pub fn lerp_position(&mut self) {
        self.render_position = self.render_position.lerp(self.position * 16.0, 0.5);
    }

    pub fn render(&self, game: &Game) {
        let shape = self.get_shape();
        for y in 0..shape.len() {
            for x in 0..shape[y].len() {
                if shape[y][x] != 0 {
                    draw_texture(
                        game.block_texture.unwrap(),
                        self.render_position.x + x as f32 * 16.0,
                        self.render_position.y + y as f32 * 16.0,
                        COLORS[shape[y][x] as usize],
                    );
                }
            }
        }
    }
}