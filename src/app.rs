use ratzilla::{event::KeyCode, ratatui::Terminal, WebGl2Backend, WebRenderer};

#[derive(Debug, Clone, Default)]
pub struct App {}

impl App {
    pub fn handle_key_event(&mut self, key_code: KeyCode) {}

    pub fn run(mut self, mut terminal: Terminal<WebGl2Backend>) {
        let mut cloned = self.clone();
        terminal.on_key_event(move |key_event| {
            cloned.handle_key_event(key_event.code);
        });

        terminal.draw_web(move |f| {
            self.render(f);
        });
    }
}
