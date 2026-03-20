use std::io;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Tabs},
};

use contentforge_db::DbPool;

// ---------------------------------------------------------------------------
// Tabs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Dashboard,
    Drafts,
    Schedule,
    Publish,
    Analytics,
}

impl Tab {
    const ALL: [Tab; 5] = [
        Tab::Dashboard,
        Tab::Drafts,
        Tab::Schedule,
        Tab::Publish,
        Tab::Analytics,
    ];

    fn title(&self) -> &'static str {
        match self {
            Tab::Dashboard => "Dashboard",
            Tab::Drafts => "Drafts",
            Tab::Schedule => "Schedule",
            Tab::Publish => "Publish",
            Tab::Analytics => "Analytics",
        }
    }

    fn next(&self) -> Tab {
        let idx = Tab::ALL.iter().position(|t| t == self).unwrap_or(0);
        Tab::ALL[(idx + 1) % Tab::ALL.len()]
    }

    fn prev(&self) -> Tab {
        let idx = Tab::ALL.iter().position(|t| t == self).unwrap_or(0);
        if idx == 0 {
            Tab::ALL[Tab::ALL.len() - 1]
        } else {
            Tab::ALL[idx - 1]
        }
    }
}

// ---------------------------------------------------------------------------
// App state
// ---------------------------------------------------------------------------

pub struct App {
    pub db: DbPool,
    pub active_tab: Tab,
    pub should_quit: bool,
}

impl App {
    pub fn new(db: DbPool) -> Self {
        Self {
            db,
            active_tab: Tab::Dashboard,
            should_quit: false,
        }
    }

    /// Handle a key event, returning true if the event was consumed.
    fn handle_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Tab | KeyCode::Right => self.active_tab = self.active_tab.next(),
            KeyCode::BackTab | KeyCode::Left => self.active_tab = self.active_tab.prev(),
            KeyCode::Char('1') => self.active_tab = Tab::Dashboard,
            KeyCode::Char('2') => self.active_tab = Tab::Drafts,
            KeyCode::Char('3') => self.active_tab = Tab::Schedule,
            KeyCode::Char('4') => self.active_tab = Tab::Publish,
            KeyCode::Char('5') => self.active_tab = Tab::Analytics,
            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// Drawing
// ---------------------------------------------------------------------------

fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    // Tab bar
    let tab_titles: Vec<Line> = Tab::ALL.iter().map(|t| Line::from(t.title())).collect();
    let active_idx = Tab::ALL.iter().position(|t| *t == app.active_tab).unwrap_or(0);
    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title(" ContentForge "))
        .select(active_idx)
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    frame.render_widget(tabs, chunks[0]);

    // Tab content
    let content_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ", app.active_tab.title()));

    let body_text = match app.active_tab {
        Tab::Dashboard => "Welcome to ContentForge!\n\nUse Tab/Shift-Tab to switch tabs. Press q to quit.\n\n(Dashboard stats will appear here)",
        Tab::Drafts => "(Draft list will appear here)\n\nPress 'n' to create new draft, Enter to edit",
        Tab::Schedule => "(Schedule entries will appear here)\n\nPress 'n' to add schedule entry",
        Tab::Publish => "(Publish queue will appear here)\n\nPress Enter to publish selected",
        Tab::Analytics => "(Analytics overview will appear here)",
    };

    let paragraph = Paragraph::new(body_text)
        .block(content_block)
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(paragraph, chunks[1]);
}

// ---------------------------------------------------------------------------
// Main entry point
// ---------------------------------------------------------------------------

/// Run the TUI application. This blocks until the user quits.
pub fn run(db: DbPool) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(db);

    // Main loop
    loop {
        terminal.draw(|frame| draw(frame, &app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key.code);
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
