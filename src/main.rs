use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::Paragraph;

fn main() -> Result<()> {
    // enter the alternate screen, enable raw mode, set terminal and backend
    let mut terminal = ratatui::init();
    // main loop
    let mut counter = 0;
    loop {
        // draw
        terminal.draw(|frame| {
            let area = frame.area();
            let widget = Paragraph::new(format!("counter: {}", counter));
            frame.render_widget(widget, area);
        })?;
        
        // handle events
        match event::read()? {
            Event::Key(key_event) => {
                match key_event.kind {
                    KeyEventKind::Press => {
                        match key_event.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Up => counter += 1,
                            KeyCode::Down => counter -= 1,
                            _ => {}
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    ratatui::restore();
    Ok(())
}