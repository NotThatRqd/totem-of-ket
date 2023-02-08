/*
 * MIT License
 *
 * Copyright (c) 2023 rad
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::{sync::mpsc, time::{Duration, Instant}, thread, io::stdout, fs};
use file_loader::load_save_file;
use tui::{backend::CrosstermBackend, Terminal, layout::{Layout, Direction, Constraint, Alignment}, widgets::{Paragraph, Block, Borders, BorderType, Tabs}, style::{Style, Color, Modifier}, text::{Spans, Span}};
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, event::{self, KeyCode}, execute};
use crossterm::event::KeyEvent;
use utils::COPYRIGHT_TEXT;
use crate::utils::get_bool;

mod file_loader;
mod utils;

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum MenuItem {
    Home,
    Pray,
    Save,
}

impl From<MenuItem> for usize {
    fn from(value: MenuItem) -> usize {
        match value {
            MenuItem::Home => 0,
            MenuItem::Pray => 1,
            MenuItem::Save => 2,
        }
    }
}

fn main() {
    println!("load save file? (y/n)");

    let should_load_save_file = get_bool().expect("get bool");

    // load save file or create a new one depending on user input
    let mut player_data = if should_load_save_file {
        load_save_file("player_data.json").expect("load save file")
    } else {
        file_loader::PlayerData::default()
    };

    enable_raw_mode().expect("run in raw mode");


    // setup an input loop
    let rx = input_loop();

    enable_raw_mode().expect("enter raw mode");
    execute!(stdout(), EnterAlternateScreen).expect("enter alt screen");

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).expect("create terminal");

    let menu_titles = vec!["Home", "Pray", "Save", "Quit"];
    let mut active_menu_item = MenuItem::Home;

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(2),
                    Constraint::Length(3),
                ].as_ref()
            )
            .split(size);

            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED)
                        ),
                        Span::styled(
                            rest,
                            Style::default().fg(Color::White)
                        )
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item.into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);

            let copyright = Paragraph::new(COPYRIGHT_TEXT)
                .style(Style::default().fg(Color::Green))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Copyright")
                        .border_type(BorderType::Plain)
                );

            rect.render_widget(copyright, chunks[2]);


            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(&player_data.name), chunks[1]),
                MenuItem::Pray => rect.render_widget(render_pray(player_data.prays), chunks[1]),
                MenuItem::Save => rect.render_widget(render_save(), chunks[1]),
            }
            
        }).expect("can draw on terminal");

        match rx.recv().expect("able to receive from channel") {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode().expect("can leave raw mode");
                    execute!(stdout(), LeaveAlternateScreen).expect("can leave alt screen");
                    break;
                },
                KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                KeyCode::Char('p') => active_menu_item = MenuItem::Pray,
                KeyCode::Char('s') => active_menu_item = MenuItem::Save,
                KeyCode::Char(' ') => {
                    if active_menu_item == MenuItem::Pray {
                        player_data.prays += 1;
                    }

                    else if active_menu_item == MenuItem::Save {
                        // save player_data to file
                        let as_string = serde_json::to_string(&player_data).unwrap();
                        fs::write("player_data.json", as_string).expect("should be able to write to file");
                    }
                }
                _ => (),
            },
            Event::Tick => (),
        }
    }

    fn render_home(name: &str) -> Paragraph {
        let home = Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Welcome,")]),
            Spans::from(vec![Span::raw(name)]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("to")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::styled(
                "Totem of Ket",
                Style::default().fg(Color::LightBlue),
            )]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Press 'p' to access pray, 'h' to go home, 'q' to quit, 's' to save")]),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Home")
                .border_type(BorderType::Plain),
        );
        home
    }

    fn render_pray<'a>(pray_amount: u32) -> Paragraph<'a> {
        Paragraph::new(vec![
            Spans::from(vec![Span::styled(
                "Press [space bar] to pray to the totem",
                Style::default().fg(Color::LightMagenta),
            )]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw(format!("You have prayed {} times", pray_amount))]),
        ])
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Pray")
                    .border_type(BorderType::Plain)
            )
    }

    fn render_save<'a>() -> Paragraph<'a> {
        Paragraph::new(vec![
            Spans::from(vec![Span::styled(
                "Press [space bar] to save to file",
                Style::default().fg(Color::Yellow),
            )]),
        ])
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Save")
                    .border_type(BorderType::Plain)
            )
    }

}

fn input_loop() -> std::sync::mpsc::Receiver<Event<KeyEvent>> {
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("use poll") {
                if let event::Event::Key(key) = event::read().expect("read events") {
                    tx.send(Event::Input(key)).expect("send key event");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    rx
}