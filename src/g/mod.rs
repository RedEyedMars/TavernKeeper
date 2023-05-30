#[derive(Clone, Copy)]
pub struct Rotation {
    cos: f32,
    sin: f32,
    pub angle: f32,
}

impl Rotation {
    pub fn new(angle: f32) -> Rotation {
        Rotation {
            cos: angle.cos(),
            sin: angle.sin(),
            angle: angle,
        }
    }
    pub fn vec(self) -> Vec<f32> {
        vec![self.cos, self.sin, -self.sin, self.cos]
    }
    pub fn add(&mut self, angle: f32) {
        //cos(x + y) = cos(x)cos(y) - sin(x)sin(y)
        //sin(x + y) = sin(x)cos(y) + cos(x)sin(y)
        self.angle += angle;
        self.cos = self.angle.cos();
        self.sin = self.angle.sin();
    }

    pub fn add_and_vec(&self, other: &Rotation) -> Vec<f32> {
        //cos(x + y) = cos(x)cos(y) - sin(x)sin(y)
        //sin(x + y) = sin(x)cos(y) + cos(x)sin(y)

        let cos = (self.cos * other.cos) - (self.sin * other.sin);
        let sin = (self.sin * other.cos) + (self.cos * other.sin);

        vec![cos as f32, sin as f32, -sin as f32, cos as f32]
    }
}
use failure::err_msg;
use sdl2::video::GLContext;
use sdl2::video::Window;
use sdl2::Sdl;

pub mod animation;
pub mod render_gl;
pub mod resources;

pub fn setup() -> Result<(Sdl, Window, GLContext), failure::Error> {
    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().map_err(err_msg)?;
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::BLEND);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }
    Ok((sdl, window, _gl_context))
}

pub fn preprocess() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}
pub fn postprocess(window: &mut Window) {
    window.gl_swap_window()
}
