use glutin_window::GlutinWindow as Window;
use piston::window::{ Size, WindowSettings };
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct GraphicsConfig {
    // OpenGL drawing backend
    pub gl: GlGraphics,
    // Window
    pub settings: Window,
    // Window size
    pub size: Size,
}

impl GraphicsConfig {
    pub fn new(title: &'static str, width: f64, height: f64) -> GraphicsConfig {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_3;
        
        let test_window: Window = WindowSettings::new(title, [width, height])
            // Sets the OpenGL version
            .graphics_api(opengl)
            .exit_on_esc(true)
            .fullscreen(true)
            .build()
            .unwrap();

        let test_w = test_window.ctx.window().current_monitor().unwrap();
        let test_w_size = test_w.size();
        let test_w_scale_factor = test_w.scale_factor();
        let size = Size { width: test_w_size.width as f64 / test_w_scale_factor, height: test_w_size.height as f64 / test_w_scale_factor };

        let settings: Window = WindowSettings::new(title, [size.width as f64, size.height as f64])
            // Sets the OpenGL version
            .graphics_api(opengl)
            .exit_on_esc(true)
            .fullscreen(true)
            .build()  
            .unwrap() ;

        GraphicsConfig {
            gl: GlGraphics::new(opengl),
            settings,
            size,
        }
    }
}