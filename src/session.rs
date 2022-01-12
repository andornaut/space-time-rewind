use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Result, Stdout};
use tui::{backend::CrosstermBackend, Terminal};

type CrosstermTerminal = Terminal<CrosstermBackend<Stdout>>;

pub struct Session {
    pub terminal: CrosstermTerminal,
}

impl Session {
    pub fn begin() -> Result<Self> {
        enable_raw_mode()?;

        let mut terminal = create_terminal()?;
        terminal.hide_cursor()?;
        Ok(Self { terminal })
    }

    pub fn end(&mut self) -> Result<()> {
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        self.terminal.show_cursor()?;
        disable_raw_mode()?;
        Ok(())
    }
}

fn create_terminal() -> Result<CrosstermTerminal> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout))
}
