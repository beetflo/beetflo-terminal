use conrod::glium::glutin::{WindowBuilder};
use conrod::glium::{DisplayBuild, Display};

pub struct Window {
    
}


impl Window {
    pub fn build(w: u32, h: u32) -> Display {
        WindowBuilder::new()
            .with_vsync()
            .with_dimensions(w, h)
            .with_title("Beetflo")
            .with_multisampling(4)
            .build_glium()
            .unwrap()
    }
}
