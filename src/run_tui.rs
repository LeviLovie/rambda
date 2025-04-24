use super::State;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};

pub fn run_tui() -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut input = String::new();
    let mut state = State::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(5), Constraint::Length(3)].as_ref())
                .split(size);

            let log_content = Paragraph::new(state.displayed_history.join("\n"))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title("History"),
                )
                .alignment(Alignment::Left);
            f.render_widget(log_content, chunks[0]);

            let input_text = Paragraph::new(input.as_str())
                .style(Style::default())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title("Execute"),
                )
                .alignment(Alignment::Left);
            f.render_widget(input_text, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(25))? {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Enter => {
                        state.exec(input.clone());
                        input.clear();
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    _ => {}
                }
            }
        }

        if state.exit {
            break;
        }

        for _ in 1..3 {
            for (i, line) in state.history.iter().enumerate() {
                let displayed_line = state.displayed_history.get(i);
                if displayed_line.is_none() {
                    state.displayed_history.push(String::new());
                } else {
                    let displayed_line = displayed_line.unwrap();
                    if displayed_line.len() < line.len() {
                        let char_to_push = line.chars().nth(displayed_line.len());
                        if let Some(char_to_push) = char_to_push {
                            state.displayed_history[i].push(char_to_push);
                            break;
                        }
                    }
                }
            }
        }

        thread::sleep(Duration::from_millis(10));
    }

    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
