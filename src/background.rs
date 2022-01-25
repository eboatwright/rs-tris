use crate::SCREEN_HEIGHT;
use crate::COLORS;
use macroquad::prelude::*;
use crate::Game;

pub fn update_background(game: &mut Game) {
	for particle in game.particles.iter_mut() {
		particle.position.y -= particle.radius * 0.01;
		if particle.position.y <= -particle.radius * 2.0 {
			particle.position = vec2(
				particle.position.x,
				SCREEN_HEIGHT as f32 + particle.radius,
			);
		}
	}
}

pub fn render_background(game: &Game) {
	draw_texture(
		game.background_texture.unwrap(),
		-16.0,
		0.0,
		WHITE,
	);
	for particle in game.particles.iter() {
		draw_circle(
			particle.position.x.round(),
			particle.position.y.round(),
			particle.radius,
			COLORS[9],
		);
	}
}

#[derive(Clone)]
pub struct Particle {
	pub position: Vec2,
	pub radius: f32,
}