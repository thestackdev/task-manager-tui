use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal,
    prelude::*,
    widgets::{Block, Borders},
};

#[derive(Default)]
pub struct App {
    should_exit: bool,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;

            if let Event::Key(key) = event::read()? {
                self.handle_event(key);
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char('q') => self.should_exit = true,
            KeyCode::Char('a') => self.should_exit = true,
            _ => {}
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [main_area, footer_area] =
            Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)]).areas(area);

        App::render_main(main_area, buf);
        App::render_footer(footer_area, buf);
    }
}

impl App {
    fn render_main(area: Rect, buf: &mut Buffer) {
        Block::default()
            .title("Main Area")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow))
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Block::default()
            .title("Footer Area")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow))
            .render(area, buf);
    }
}
