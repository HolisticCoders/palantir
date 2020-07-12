use crate::resources::Resources;
use std::error::Error;
use std::path::Path;

pub struct Application {
    pub resources: Resources,
    pub window: sdl2::video::Window,
    pub gl: gl::Gl,
    pub events: sdl2::EventPump,
    pub video: sdl2::VideoSubsystem,
    _sdl: sdl2::Sdl,
    _context: sdl2::video::GLContext,
}

impl Application {
    pub fn new(width: u32, height: u32) -> Result<Self, Box<dyn Error>> {
        let resources = Resources::from_relative_exe_path(Path::new("assets"))?;

        let _sdl = sdl2::init()?;

        let video = _sdl.video()?;

        let gl_attributes = video.gl_attr();

        gl_attributes.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attributes.set_context_version(3, 0);

        let window = video
            .window("Palantir", width, height)
            .opengl()
            .allow_highdpi()
            .resizable()
            .build()?;

        let _context = window.gl_create_context()?;
        let gl = gl::Gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

        let events = _sdl.event_pump()?;

        unsafe {
            gl.Viewport(0, 0, width as i32, height as i32);
            gl.Enable(gl::DEPTH_TEST);
        }

        Ok(Application {
            resources,
            window,
            gl,
            events,
            video,
            _sdl,
            _context,
        })
    }
}
