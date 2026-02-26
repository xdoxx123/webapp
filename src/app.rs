use std::default;

use ratzilla::ratatui::prelude::*;
use ratzilla::ratatui::widgets::{Axis, Chart, Dataset};
use ratzilla::ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Style, Stylize},
    symbols,
    text::Span,
    widgets::{Block, BorderType, Clear, Paragraph},
};
use tachyonfx::Duration;
use tachyonfx::{Effect, EffectRenderer, EffectTimer, fx};
pub struct app {
    page: i8,
    offset:f64,
    fe: Effect,
}

impl app {
    pub fn new() -> Self {
        Self {
            page: 0,
            offset:0.0,
            fe: fx::coalesce(EffectTimer::from_ms(
                4000,
                tachyonfx::Interpolation::CubicIn,
            )),
        }
    }
    pub fn render(&mut self, frame: &mut Frame) {
        match self.page {
            0 => {
                Clear.render(frame.area(), frame.buffer_mut());
                let border = Block::bordered()
                    .border_style(Style::new().light_magenta())
                    .border_type(BorderType::Rounded)
                    .title_alignment(Alignment::Center)
                    .title_style(Style::new().fg(Color::LightCyan))
                    .title("About me");
                let yapping = Paragraph::new(
                    r#"hi im xdoxx123 and im your another soon-to-be-replaced dev!

i like coding in rust and lua

also i like old tech and messing around with minecraft

if you even care this website was made in rust 

i wish you a good day/night"#,
                )
                .centered();
                let sin: Vec<(f64, f64)> = (0..200)
                    .map(|i| {
                        let x = i as f64 * 0.1;
                        let y = (x * 3.14159 / 5.0 + self.offset).sin();
                    
                        (x, y)
                    })
                    .collect();
                let ds = Dataset::default()
                    
                    .marker(symbols::marker::Marker::Braille)
                    .graph_type(ratzilla::ratatui::widgets::GraphType::Line)
                    .style(Style::new().light_cyan())
                    .data(&sin);

                let sinechart = Chart::new(vec![ds]).block(
                    Block::bordered()
                        .title("oo cool wave its 100% not here just to fill space")
                        .border_style(Style::new().light_magenta())
                        .border_type(BorderType::Rounded),
                ).x_axis(Axis::default().bounds([0.0,20.0])).y_axis(Axis::default().bounds([-1.0,1.0]));
                let chunks = Layout::default()
                    .constraints([
                        Constraint::Length(9),
                        Constraint::Length(5),
                        Constraint::Min(0),
                    ])
                    .direction(Direction::Vertical)
                    .split(border.inner(frame.area()));
                frame.render_widget(&border, frame.area());
                frame.render_widget(yapping, chunks[0]);
                frame.render_widget(sinechart, chunks[2]);

                if self.fe.running() {
                    frame.render_effect(&mut self.fe, chunks[0], Duration::from_millis(100));
                    frame.render_effect(&mut self.fe, chunks[2], Duration::from_millis(100));
                }
                self.tick();
            }

            _ => {
                frame.render_widget(Block::bordered().title("NUL"), frame.area());
            }
        }
    }
    fn tick(&mut self) {self.offset += 0.1}
}
