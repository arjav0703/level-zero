use std::{cell::RefCell, io, rc::Rc};

use ratzilla::{WebGl2Backend, ratatui::Terminal};

mod app;
use app::App;
mod render;

fn main() -> io::Result<()> {
    let mut app = App::default();

    // let counter = Rc::new(RefCell::new(0));
    let backend = WebGl2Backend::new()?;
    let mut terminal = Terminal::new(backend)?;

    app.run(terminal);

    Ok(())
}
