use gl;

use crate::g::animation::texture::Texture;
use packed_simd_2::f32x4;

#[derive(Clone, Copy)]
pub enum SizeMode {
    Bot,
    Screen,
}

#[derive(Clone, Copy)]
pub enum Animation {
    MainXShift2x16,
    MainXShift2x1,
}

impl Animation {
    pub fn vertices(self, size: SizeMode) -> Vec<f32x4> {
        let (x_coord, y_coord) = match size {
            SizeMode::Bot => (0.1f32, 0.1f32),
            SizeMode::Screen => (1f32, 1f32),
        };
        let (x_shift, y_shift) = match self {
            Animation::MainXShift2x16 => (0.5f32, 1.0f32 / 16.0f32),
            Animation::MainXShift2x1 => (0.5f32, 1.0f32),
        };

        vec![
            f32x4::new(-x_coord, -y_coord, 0.0, 0.0),
            f32x4::new(-x_coord, y_coord, 0.0, y_shift),
            f32x4::new(x_coord, -y_coord, x_shift, 0.0),
            f32x4::new(-x_coord, y_coord, 0.0, y_shift),
            f32x4::new(x_coord, y_coord, x_shift, y_shift),
            f32x4::new(x_coord, -y_coord, x_shift, 0.0),
        ]
    }
    pub fn animate(
        &self,
        y_shift: f32,
        tex: &Texture,
        tex_coord_loc: &gl::types::GLint,
        animation_state: u8,
    ) {
        unsafe {
            Animation::static_animate(tex);
            match self {
                Animation::MainXShift2x1 => {
                    Animation::shift_animate(0f32, tex_coord_loc, animation_state);
                }
                Animation::MainXShift2x16 => {
                    Animation::shift_animate(
                        (15f32 - y_shift) / 16f32,
                        tex_coord_loc,
                        animation_state,
                    );
                }
            }
        }
    }
    unsafe fn static_animate(tex: &Texture) {
        gl::ActiveTexture(gl::TEXTURE0);
        tex.bind();
    }
    unsafe fn shift_animate(y_shift: f32, tex_coord_loc: &gl::types::GLint, animation_state: u8) {
        gl::Uniform2f(
            *tex_coord_loc,
            match animation_state {
                0u8 => 0.0f32,
                _ => 0.5f32,
            },
            y_shift,
        );
    }
}
