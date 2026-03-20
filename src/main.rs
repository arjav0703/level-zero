use std::io;

use ratzilla::{
    backend::webgl2::{FontAtlasData, WebGl2Backend, WebGl2BackendOptions},
    ratatui::Terminal,
};

mod app;
use app::App;
mod render;

fn main() -> io::Result<()> {
    let mut app = App::default();

    let window = ratzilla::web_sys::window().expect("no global `window` exists");
    let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as u32;

    let backend_options = WebGl2BackendOptions::new()
        .font_atlas_config(ratzilla::FontAtlasConfig::dynamic(
            &["Hack Nerd Font Mono", "Hack", "Fira Code"],
            16.0,
        ))
        // .font_atlas(FontAtlasData {
        //     font_name: "Arial".into(),
        //     font_size: 60.0,
        //     ..Default::default()
        // })
        .size((width, (height * 2) + 600)); // 200vh = 2 * viewport height
    let backend = WebGl2Backend::new_with_options(backend_options)?;
    let terminal = Terminal::new(backend)?;

    app.run(terminal);

    Ok(())
}
