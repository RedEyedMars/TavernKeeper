use crate::g::render_gl::{self, buffer};
use crate::g::resources::Resources;
use failure;
use gl;
use packed_simd_2::f32x2;

use crate::a::GameState;
use crate::g::animation::animaton::{Animation, SizeMode};
use crate::g::animation::texture::Texture;
use crate::g::animation::{animaton, texture};

pub struct Img {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer, // _ to disable warning about not used vbo
    vao: buffer::VertexArray,
    animation: animaton::Animation,

    xy: f32x2,
    tex: Texture,
    y_shift: f32,
}

impl Img {
    pub fn new(
        image_name: String,
        y_shift: f32,
        size: SizeMode,
        animation: Animation,
        res: &mut Resources,
    ) -> Result<Img, failure::Error> {
        let program = render_gl::Program::from_res(res, "shaders/dynamic_img")?;
        let tex = match res.get_texture(image_name.clone()) {
            Some(texture) => texture.clone(),
            None => {
                let t = texture::Texture::create_texture(image_name.clone(), &res)?;
                res.insert_texture(image_name, t.clone());
                t
            }
        };
        let vertices = animation.vertices(size);

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

        Ok(Img {
            program: program,
            _vbo: vbo,
            vao,
            animation: animation,
            tex: tex,
            xy: f32x2::new(0f32, 0f32),
            y_shift: y_shift,
        })
    }
    pub fn shift(&mut self, x: f32, y: f32) {
        self.xy += f32x2::new(x, y);
    }
    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.xy = f32x2::new(x, y);
    }
    pub fn render(&self, game: &GameState) {
        self.program.set_used();
        self.vao.bind();
        self.animation.animate(
            self.y_shift,
            &self.tex,
            &self.program.get_tex_coord_shift_loc(),
            game.animation_state,
        );
        //self.x += 0.01;
        unsafe {
            gl::Uniform2fv(
                self.program.get_pos_coord_shift_loc(),
                1,
                (&vec![self.xy]).as_ptr() as *const f32,
            );
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                6,             // number of indices to be rendered
            );
        }
        self.vao.unbind();
    }
}

pub struct ImgFactory {
    image_name: String,
    size: SizeMode,
    animation: Animation,
    y_shift: f32,
}

impl ImgFactory {
    pub fn new(image_name: &str, y_shift: f32, size: SizeMode, animation: Animation) -> ImgFactory {
        ImgFactory {
            image_name: String::from(image_name),
            size: size,
            animation: animation,
            y_shift: y_shift,
        }
    }
    pub fn create(&self, res: &mut Resources) -> Result<Img, failure::Error> {
        Img::new(
            self.image_name.clone(),
            self.y_shift,
            self.size,
            self.animation,
            res,
        )
    }
}
