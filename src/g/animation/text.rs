use crate::g::render_gl::{self, buffer};
use crate::g::resources::Resources;
use failure;
use gl;
use packed_simd_2::{f32x2, f32x4};

use crate::g::animation::texture;
use crate::g::animation::texture::Texture;

const CHAR_WIDTH: f32 = 0.06f32;

#[derive(StructOfArray)]
pub struct TextChar {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer, // _ to disable warning about not used vbo
    vao: buffer::VertexArray,
    x_shift: f32,
    xy: f32x2,
}

pub struct Text {
    chars: TextCharVec,

    tex: Texture,
}

impl Text {
    pub fn new(size: usize, res: &mut Resources) -> Result<Text, failure::Error> {
        let tex = match res.get_texture("abc_1.png".to_string()) {
            Some(texture) => texture.clone(),
            None => {
                let t = texture::Texture::create_texture("abc_1.png".to_string(), &res)?;
                res.insert_texture("abc_1.png".to_string(), t.clone());
                t
            }
        };
        let mut chars = TextCharVec::with_capacity(size);

        for i in 0..size {
            let program = render_gl::Program::from_res(res, "shaders/dynamic_img")?;
            let vertices = Text::vertices();

            let vbo = buffer::ArrayBuffer::new();
            vbo.bind();
            vbo.static_draw_data(&vertices);
            vbo.unbind();

            // set up vertex array object

            let vao = buffer::VertexArray::new();

            vao.bind();
            vbo.bind();
            unsafe {
                //Vertex::vertex_attrib_pointers(gl);
                gl::EnableVertexAttribArray(0 as gl::types::GLuint);
                gl::VertexAttribPointer(
                    0 as gl::types::GLuint, // location
                    2,                      // the number of components per generic vertex attribute
                    gl::FLOAT,              // data type
                    gl::FALSE,              // normalized (int-to-float conversion)
                    4 * ::std::mem::size_of::<f32>() as gl::types::GLint,
                    0 as *const gl::types::GLvoid,
                );
                gl::EnableVertexAttribArray(1 as gl::types::GLuint);
                gl::VertexAttribPointer(
                    1 as gl::types::GLuint, // location
                    2,                      // the number of components per generic vertex attribute
                    gl::FLOAT,              // data type
                    gl::FALSE,              // normalized (int-to-float conversion)
                    4 * ::std::mem::size_of::<f32>() as gl::types::GLint,
                    (2 * ::std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
                );
            }
            vbo.unbind();
            vao.unbind();

            chars.push(TextChar {
                program,
                _vbo: vbo,
                vao: vao,
                x_shift: 62f32,
                xy: f32x2::new(CHAR_WIDTH * (i as f32), 0f32),
            })
        }

        Ok(Text {
            chars: chars,
            tex: tex,
        })
    }
    pub fn vertices() -> Vec<f32x4> {
        let (x_coord, y_coord) = (0.05f32, 0.05f32);
        let (x_shift, y_shift) = (1f32 / 64f32, 1.0f32);

        vec![
            f32x4::new(-x_coord, -y_coord, 0.0, 0.0),
            f32x4::new(-x_coord, y_coord, 0.0, y_shift),
            f32x4::new(x_coord, -y_coord, x_shift, 0.0),
            f32x4::new(-x_coord, y_coord, 0.0, y_shift),
            f32x4::new(x_coord, y_coord, x_shift, y_shift),
            f32x4::new(x_coord, -y_coord, x_shift, 0.0),
        ]
    }

    pub fn shift(&mut self, x: f32, y: f32) {
        let delta = f32x2::new(x, y);
        for c in self.chars.iter_mut() {
            *c.xy += delta;
        }
    }
    pub fn set_pos(&mut self, x: f32, y: f32) {
        let delta = f32x2::new(x, y);
        for (i, c) in self.chars.iter_mut().enumerate() {
            *c.xy += delta + f32x2::new(CHAR_WIDTH * (i as f32), 0f32);
        }
    }
    fn find_x_shift(c: u8) -> f32 {
        if c >= '$' as u8 && c <= '&' as u8 {
            0f32 + (c - '$' as u8) as f32
        } else if c >= '0' as u8 && c <= '9' as u8 {
            62f32 - (c - '0' as u8) as f32
        } else {
            63f32
        }
    }
    pub fn text(&mut self, s: &[u8]) {
        let mut i = 0usize;
        for x_shift in &mut self.chars.x_shift {
            *x_shift = Text::find_x_shift(s[i]);
            i += 1;
        }
    }
    pub fn render(&self) {
        unsafe {
            for c in self.chars.iter() {
                c.program.set_used();
                c.vao.bind();
                gl::ActiveTexture(gl::TEXTURE0);
                self.tex.bind();
                gl::Uniform2f(
                    c.program.get_tex_coord_shift_loc(),
                    (63f32 - c.x_shift) / 64f32,
                    0.0f32,
                );

                gl::Uniform2fv(
                    c.program.get_pos_coord_shift_loc(),
                    1,
                    (&vec![c.xy.extract(0), c.xy.extract(1)]).as_ptr() as *const f32,
                );
                gl::DrawArrays(
                    gl::TRIANGLES, // mode
                    0,             // starting index in the enabled arrays
                    6,             // number of indices to be rendered
                );

                c.vao.unbind();
            }
        }
    }
}
