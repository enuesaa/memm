use std::io::stdout;

use anyhow::{Result, anyhow};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

struct App {
    input: String,
    items: Vec<(i32, String)>,
    filtered: Vec<usize>,
    selected: usize,
}

impl App {
    fn new(items: Vec<(i32, String)>) -> Self {
        let filtered = (0..items.len()).collect();

        Self {
            input: String::new(),
            items,
            filtered,
            selected: 0,
        }
    }

    fn filter(&mut self) {
        let q = self.input.to_lowercase();

        self.filtered = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, (_, title))| title.to_lowercase().contains(&q))
            .map(|(i, _)| i)
            .collect();

        if self.selected >= self.filtered.len() {
            self.selected = 0;
        }
    }

    fn next(&mut self) {
        if self.filtered.is_empty() {
            return;
        }

        self.selected = (self.selected + 1) % self.filtered.len();
    }

    fn prev(&mut self) {
        if self.filtered.is_empty() {
            return;
        }

        if self.selected == 0 {
            self.selected = self.filtered.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    fn selected_item(&self) -> Option<&(i32, String)> {
        self.filtered
            .get(self.selected)
            .map(|i| &self.items[*i])
    }
}

fn ui(frame: &mut Frame, app: &App) {
    let areas = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
    ])
    .split(frame.area());

    let input = Paragraph::new(format!("> {}", app.input))
        .block(Block::bordered().title("Search"));

    frame.render_widget(input, areas[0]);

    let items: Vec<ListItem> = app
        .filtered
        .iter()
        .map(|i| ListItem::new(app.items[*i].1.clone()))
        .collect();

    let list = List::new(items)
        .block(Block::bordered().title("Memos"))
        .highlight_symbol("❯ ")
        .highlight_style(Style::default().reversed());

    let mut state = ListState::default();
    state.select(Some(app.selected));

    frame.render_stateful_widget(list, areas[1], &mut state);
}

pub fn run_search(items: Vec<(i32, String)>) -> Result<(i32, String)> {
    enable_raw_mode()?;

    let mut out = stdout();
    execute!(out, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(out);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(items);

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    app.input.push(c);
                    app.filter();
                }
                KeyCode::Backspace => {
                    app.input.pop();
                    app.filter();
                }
                KeyCode::Up => {
                    app.prev();
                }
                KeyCode::Down => {
                    app.next();
                }
                KeyCode::Enter => {
                    break;
                }
                KeyCode::Esc => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    return Err(anyhow!("notselected"));
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    if let Some(item) = app.selected_item() {
        return Ok(item.clone());
    }
    return Err(anyhow!("notselected"));
}
