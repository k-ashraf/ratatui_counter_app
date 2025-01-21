use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{widgets::Paragraph, Frame};

struct App {
    counter: i32,
    should_exit: bool
}

fn render_ui(app: &mut App, frame: &mut Frame<'_>) {
    let widget = Paragraph::new(format!("counter: {}", app.counter));
    let area = frame.area();
    frame.render_widget(widget, area);
}

fn update_app(app: &mut App) -> Result<()> {
    match event::read()? {
        Event::Key(key_event) => {
            match key_event.kind {
                KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char('q') => app.should_exit = true,
                        KeyCode::Up => app.counter += 1,
                        KeyCode::Down => app.counter -= 1,
                        _ => {}
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
    Ok(())
}

fn run() -> Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App{
        counter: 0,
        should_exit: false,
    };

    loop {
        terminal.draw(|frame| render_ui(&mut app, frame))?;
        update_app(&mut app)?;
        if app.should_exit == true { break; }
    }

    Ok(())
}

fn main() -> Result<()> {
    run()?;
    ratatui::restore();
    Ok(())
}