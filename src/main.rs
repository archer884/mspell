use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind}, layout::{Constraint, Layout, Rect}, style::{Style, Stylize}, widgets::{Block, List, ListState, Padding, Paragraph}, DefaultTerminal
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let items = [
        "word",
        "turd",
        "gird",
        "herd",
        "bird",
    ];

    let mut list_state = ListState::default();
    list_state.select_first();
    
    loop {
        terminal.draw(|frame| {
            let layout = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas::<2>(frame.area());
            let block = Block::new().title("misspelled wrod").padding(Padding::uniform(1));
            let list = List::new(items).highlight_style(Style::new().bold()).highlight_symbol("!");        

            frame.render_widget(block, layout[0]);
            frame.render_stateful_widget(list, layout[1], &mut list_state);
        })?;


        if let event::Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            
            match key.code {
                KeyCode::Enter => {
                    
                },
                KeyCode::Up => {
                    list_state.select_previous();
                },
                KeyCode::Down => {
                    list_state.select_next();
                },
                KeyCode::Char('q') => return Ok(()),

                _ => (),
            }        
        }
    }
}
