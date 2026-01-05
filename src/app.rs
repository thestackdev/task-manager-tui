use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal,
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

#[derive(PartialEq, Default)]
enum Mode {
    #[default]
    Normal,
    Input,
}

struct TodoItem {
    description: String,
    is_done: bool,
}

impl TodoItem {
    fn new(description: &str) -> Self {
        Self {
            description: description.to_string(),
            is_done: false,
        }
    }
}

#[derive(Default)]
pub struct App {
    should_exit: bool,
    items: Vec<TodoItem>,
    state: ListState,
    mode: Mode,
    input_buffer: String,
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

        match self.mode {
            Mode::Normal => match key.code {
                KeyCode::Char('q') => self.should_exit = true,
                KeyCode::Char('a') => {
                    self.mode = Mode::Input;
                    self.input_buffer.clear();
                }
                KeyCode::Char('j') | KeyCode::Down => self.select_next(),
                KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
                KeyCode::Char('g') => self.select_first(),
                KeyCode::Char('G') => self.select_last(),
                KeyCode::Char(' ') | KeyCode::Enter => self.toggle_selected(),
                KeyCode::Char('d') => self.delete_selected(),
                _ => {}
            },
            Mode::Input => match key.code {
                KeyCode::Enter => {
                    if !self.input_buffer.is_empty() {
                        self.items.push(TodoItem::new(&self.input_buffer));
                        self.input_buffer.clear();
                        self.state.select_last();
                    }
                    self.mode = Mode::Normal;
                }
                KeyCode::Esc => {
                    self.input_buffer.clear();
                    self.mode = Mode::Normal;
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                }
                KeyCode::Char(c) => {
                    self.input_buffer.push(c);
                }
                _ => {}
            },
        }
    }

    fn toggle_selected(&mut self) {
        if let Some(index) = self.state.selected()
            && let Some(item) = self.items.get_mut(index)
        {
            item.is_done = !item.is_done;
        }
    }

    fn delete_selected(&mut self) {
        if let Some(index) = self.state.selected()
            && index < self.items.len()
        {
            self.items.remove(index);
            if self.items.is_empty() {
                self.state.select(None);
            } else if index >= self.items.len() {
                self.state.select_last();
            }
        }
    }

    fn select_next(&mut self) {
        self.state.select_next();
    }

    fn select_previous(&mut self) {
        self.state.select_previous();
    }

    fn select_first(&mut self) {
        self.state.select_first();
    }

    fn select_last(&mut self) {
        self.state.select_last();
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [main_area, footer_area] =
            Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).areas(area);

        self.render_list(main_area, buf);
        self.render_footer(footer_area, buf);
    }
}

impl App {
    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Task Manager ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|item| {
                let checkbox = if item.is_done { "[x]" } else { "[ ]" };
                let style = if item.is_done {
                    Style::default().fg(Color::DarkGray).crossed_out()
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(format!("{} {}", checkbox, item.description)).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_symbol("▶ ")
            .highlight_style(Style::default().fg(Color::Yellow).bold());

        StatefulWidget::render(list, area, buf, &mut self.state);
    }

    fn render_footer(&mut self, area: Rect, buf: &mut Buffer) {
        let text = match self.mode {
            Mode::Normal => {
                " q: Quit | a: Add | j/k: Navigate | Enter/Space: Toggle | d: Delete | g/G: First/Last "
            }
            Mode::Input => " Type task description, Enter to save, Esc to cancel ",
        };

        let footer = if self.mode == Mode::Input {
            let input_text = format!(" New task: {}▏", self.input_buffer);
            Paragraph::new(input_text)
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Yellow))
                        .title(" Input Mode "),
                )
        } else {
            Paragraph::new(text)
                .style(Style::default().fg(Color::DarkGray))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::DarkGray)),
                )
        };

        footer.render(area, buf);
    }
}
