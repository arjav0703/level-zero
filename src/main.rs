use std::io;

use ratzilla::{
    backend::webgl2::{WebGl2Backend, WebGl2BackendOptions},
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

    let backend_options = WebGl2BackendOptions::new().size((width, height * 2)); // 200vh = 2 * viewport height
    let backend = WebGl2Backend::new_with_options(backend_options)?;
    let mut terminal = Terminal::new(backend)?;

    app.run(terminal);

    Ok(())
}
