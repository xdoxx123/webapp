use std::io;

use ratzilla::ratatui::{

    Terminal,
};
mod app;


use ratzilla::{CanvasBackend, DomBackend, WebGl2Backend, WebRenderer};

fn main() -> io::Result<()> {
    
    let backend = WebGl2Backend::new()?;
    let terminal = Terminal::new(backend)?;
    let mut app = app::app::new();
    terminal.draw_web(move |f|{
        app.render(f);
    } );

    Ok(())
}