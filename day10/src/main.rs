use day10::*;
use lazy_static::lazy_static;
use regex::Regex;
use termion;
use tui;

use std::io;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::canvas::{Canvas, Points};
use tui::widgets::{Block, Borders, Chart, Widget};
use tui::{Frame, Terminal};

use std::i32::{MAX, MIN};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let beacons = get_input();

    let mut minx = MAX;
    let mut miny = MAX;
    let mut maxx = MIN;
    let mut maxy = MIN;

    for b in beacons.iter() {
        minx = minx.min(b.init_pos.0);
        miny = miny.min(b.init_pos.1);

        maxx = maxx.max(b.init_pos.0);
        maxy = maxy.max(b.init_pos.1);
    }

    let sky = Sky {
        beacons: beacons,
        offset: (-minx, -miny),
    };

    let width = maxx - minx;
    let height = maxy - miny;

    let rect = Rect::new(0, 0, width as u16, height as u16);

    let _ = draw(&sky, &rect);
}

// utils below
fn draw(sky: &Sky, rect: &Rect) -> Result<(), io::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    let size = terminal.size()?;

    let x_bounds = [
        -sky.offset.0 as f64 / 16.0,
        (rect.width - sky.offset.0 as u16) as f64 / 16.0,
    ];
    let y_bounds = [
        -sky.offset.1 as f64 / 16.0,
        (rect.height - sky.offset.1 as u16) as f64 / 16.0,
    ];

    let sleepy = Duration::from_millis(500);

    for t in 190..210 {
        let ts = format!("{}", t);
        terminal.draw(|mut f| {
            Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("beacons"))
                .paint(|ctx| {
                    ctx.draw(&Points {
                        coords: &(sky.t(t)),
                        color: Color::White,
                    });
                })
                .x_bounds(x_bounds)
                .y_bounds(y_bounds)
                .render(&mut f, size);
        })?;
        sleep(sleepy);
    }

    Ok(())
}

fn get_input() -> Vec<Beacon> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"position=<(?P<px>[ -]?\d+), (?P<py>[ -]?\d+)> velocity=<(?P<vx>[ -]?\d+), (?P<vy>[ -]?\d+)"
        )
        .unwrap();
    }

    include_str!("../input")
        .lines()
        .map(|l| {
            let caps = RE.captures(l).unwrap();

            Beacon::new(
                (
                    caps.name("px")
                        .unwrap()
                        .as_str()
                        .trim()
                        .parse::<i32>()
                        .unwrap()
                        / SF,
                    caps.name("py")
                        .unwrap()
                        .as_str()
                        .trim()
                        .parse::<i32>()
                        .unwrap()
                        / SF,
                ),
                (
                    caps.name("vx")
                        .unwrap()
                        .as_str()
                        .trim()
                        .parse::<i32>()
                        .unwrap(),
                    caps.name("vy")
                        .unwrap()
                        .as_str()
                        .trim()
                        .parse::<i32>()
                        .unwrap(),
                ),
            )
        })
        .collect()
}
