//! This crate is a simple implementation of minesweeper. It is carefully documented to encourage
//! newbies to add new games to the repository.

extern crate termion;

use termion::{color, clear, cursor};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

// use std::env;
use std::io::{self,Stdout,Write};
use std::process;

mod kubectl;

use anyhow::Result;

use std::sync::{RwLock,Arc,Mutex};
use std::sync::mpsc::channel;

// use extra::rand::Randomizer;

// /// A cell in the grid.
// #[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
// struct Cell {
//     /// Does it contain a mine?
//     mine: bool,
//     /// Is it revealed?
//     ///
//     /// That is, is it showed or chosen previously by the player?
//     revealed: bool,
//     /// Is this cell observed?
//     ///
//     /// That is, is the state of this cell determined, or is it pending for randomization.
//     observed: bool,
//     /// Does this flag contain a flag?
//     flagged: bool,
// }
//
// /// The string printed for flagged cells.
// const FLAGGED: &str = "*";
// /// The string printed for mines in the game over revealing.
// const MINE: &str = "*";
// /// The string printed for concealed cells.
// const CONCEALED: &str = "▒";
//
// /// The game over screen.
// const GAME_OVER: &str = "
// ╔═════════════════╗\n\r\
// ║───┬Game over────║\n\r\
// ║ r ┆ replay      ║\n\r\
// ║ q ┆ quit        ║\n\r\
// ╚═══╧═════════════╝";
//
// /// The upper and lower boundary char.
// const HORZ_BOUNDARY: &str = "─";
// /// The left and right boundary char.
// const VERT_BOUNDARY: &str = "│";
//
// /// The top-left corner
// const TOP_LEFT_CORNER: &str = "┌";
// /// The top-right corner
// const TOP_RIGHT_CORNER: &str = "┐";
// /// The bottom-left corner
// const BOTTOM_LEFT_CORNER: &str = "└";
// /// The bottom-right corner
// const BOTTOM_RIGHT_CORNER: &str = "┘";
//
// /// The help page.
// const HELP: &str = r#"
// minesweeper ~ a simple minesweeper implementation.
//
// rules:
//     Select a cell to reveal, printing the number of adjacent cells holding a mine.
//     If no adjacent cells hold a mine, the cell is called free. Free cell will recursively
//     reveal their neighboring cells. If a mine is revealed, you loose. The grid wraps.
//
// flags:
//     -r | --height N ~ set the height of the grid.
//     -c | --width N  ~ set the width of the grid.
//     -h | --help     ~ this help page.
//     -b              ~ beginner mode.
//     -i              ~ intermediate mode.
//     -a              ~ advanced mode.
//     -g              ~ god mode.
//
// controls:
//     ---selection--------------------
//     space ~ reveal the current cell.
//     ---movement---------------------
//     h | a ~ move left.
//     j | s ~ move down.
//     k | w ~ move up.
//     l | d ~ move right.
//     ---flags------------------------
//     f     ~ set flag.
//     F     ~ remove flag.
//     ---control----------------------
//     q     ~ quit game.
//     r     ~ restart game.
//
// author:
//     ticki.
// "#;
//
// /// The game state.
// struct Game<R, W: Write> {
//     /// Width of the grid.
//     width: u16,
//     /// The grid.
//     ///
//     /// The cells are enumerated like you would read a book. Left to right, until you reach the
//     /// line ending.
//     grid: Box<[Cell]>,
//     /// The difficulty of the game.
//     ///
//     /// The lower, the easier.
//     difficulty: u8,
//     /// The x coordinate.
//     x: u16,
//     /// The y coordinate.
//     y: u16,
//     /// Points.
//     ///
//     /// That is, revealed fields.
//     points: u16,
//     /// Standard output.
//     stdout: W,
//     /// Standard input.
//     stdin: R,
// }
//
// /// Initialize the game.
// fn init<W: Write, R: Read>(mut stdout: W, stdin: R, difficulty: u8, w: u16, h: u16) {
//     write!(stdout, "{}", clear::All).unwrap();
//
//     // Set the initial game state.
//     let mut game = Game {
//         x: 0,
//         y: 0,
//         width: w,
//         grid: vec![Cell {
//             mine: false,
//             revealed: false,
//             observed: false,
//             flagged: false,
//         }; w as usize * h as usize].into_boxed_slice(),
//         points: 0,
//         stdin: stdin.keys(),
//         stdout,
//         difficulty
//     };
//
//     // Reset that game.
//     game.reset();
//
//     // Start the event loop.
//     game.start();
// }
//
// impl<R, W: Write> Drop for Game<R, W> {
//     fn drop(&mut self) {
//         // When done, restore the defaults to avoid messing with the terminal.
//         write!(self.stdout, "{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1)).unwrap();
//     }
// }
//
// impl<R: Iterator<Item=Result<Key, std::io::Error>>, W: Write> Game<R, W> {
//     /// Get the grid position of a given coordinate.
//     fn pos(&self, x: u16, y: u16) -> usize {
//         y as usize * self.width as usize + x as usize
//     }
//
//     /// Read cell, randomizing it if it is unobserved.
//     fn read_cell(&mut self, c: usize) {
//         if !self.grid[c].observed {
//             self.grid[c].mine = false;
//             self.grid[c].observed = true;
//         }
//     }
//
//     /// Get the cell at (x, y).
//     fn get(&mut self, x: u16, y: u16) -> Cell {
//         let pos = self.pos(x, y);
//
//         self.read_cell(pos);
//         self.grid[pos]
//     }
//
//     /// Get a mutable reference to the cell at (x, y).
//     fn get_mut(&mut self, x: u16, y: u16) -> &mut Cell {
//         let pos = self.pos(x, y);
//
//         self.read_cell(pos);
//         &mut self.grid[pos]
//     }
//
//     /// Start the game loop.
//     ///
//     /// This will listen to events and do the appropriate actions.
//     fn start(&mut self) {
//         let mut first_click = true;
//         loop {
//             // Read a single byte from stdin.
//             let b = self.stdin.next().unwrap().unwrap();
//             use termion::event::Key::*;
//             match b {
//                 Char('h') | Char('a') | Left  => self.x = self.left(self.x),
//                 Char('j') | Char('s') | Down  => self.y = self.down(self.y),
//                 Char('k') | Char('w') | Up    => self.y = self.up(self.y),
//                 Char('l') | Char('d') | Right => self.x = self.right(self.x),
//                 Char(' ') => {
//                     // Check if it was a mine.
//                     let (x, y) = (self.x, self.y);
//
//                     if first_click {
//                         // This is the player's first turn; clear all cells of
//                         // mines around the cursor.
//                         for &(x, y) in self.adjacent(x, y).iter() {
//                             self.get_mut(x, y).mine = false;
//                         }
//                         self.get_mut(x, y).mine = false;
//                         first_click = false;
//                     }
//
//                     if self.get(x, y).mine {
//                         self.reveal_all();
//                         // Make the background colour of the mine we just
//                         // landed on red, and the foreground black.
//                         write!(self.stdout, "{}{}{}{}{}",
//                                cursor::Goto(x + 2, y + 2),
//                                color::Bg(color::Red), color::Fg(color::Black),
//                                MINE,
//                                style::Reset).unwrap();
//                         self.game_over();
//                         return;
//                     }
//
//                     if !self.get(x, y).revealed {
//                         self.points += 1;
//                     }
//
//                     // Reveal the cell.
//                     self.reveal(x, y);
//
//                     self.print_points();
//                 },
//                 Char('f') => {
//                     let (x, y) = (self.x, self.y);
//                     self.toggle_flag(x, y);
//                 }
//                 Char('r') => {
//                     self.restart();
//                     return;
//                 }
//                 Char('q') => return,
//                 _ => {},
//             }
//
//             // Make sure the cursor is placed on the current position.
//             write!(self.stdout, "{}", cursor::Goto(self.x + 2, self.y + 2)).unwrap();
//             self.stdout.flush().unwrap();
//         }
//     }
//
//     /// Set a flag on cell.
//     fn set_flag(&mut self, x: u16, y: u16) {
//         if !self.get(x, y).revealed {
//             self.stdout.write_all(FLAGGED.as_bytes()).unwrap();
//             self.get_mut(x, y).flagged = true;
//         }
//     }
//     /// Remove a flag on cell.
//     fn remove_flag(&mut self, x: u16, y: u16) {
//         self.stdout.write_all(CONCEALED.as_bytes()).unwrap();
//         self.get_mut(x, y).flagged = false;
//     }
//     /// Place a flag on cell if unflagged, or remove it if present.
//     fn toggle_flag(&mut self, x: u16, y: u16) {
//         if !self.get(x, y).flagged {
//             self.set_flag(x, y);
//         } else {
//             self.remove_flag(x, y);
//         }
//     }
//
//     /// Reset the game.
//     ///
//     /// This will display the starting grid, and fill the old grid with random mines.
//     fn reset(&mut self) {
//         // Reset the cursor.
//         write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();
//
//         // Write the upper part of the frame.
//         self.stdout.write_all(TOP_LEFT_CORNER.as_bytes()).unwrap();
//         for _ in 0..self.width {
//             self.stdout.write_all(HORZ_BOUNDARY.as_bytes()).unwrap();
//         }
//         self.stdout.write_all(TOP_RIGHT_CORNER.as_bytes()).unwrap();
//         self.stdout.write_all(b"\n\r").unwrap();
//
//         // Conceal all the cells.
//         for _ in 0..self.height() {
//             // The left part of the frame
//             self.stdout.write_all(VERT_BOUNDARY.as_bytes()).unwrap();
//
//             for _ in 0..self.width {
//                 self.stdout.write_all(CONCEALED.as_bytes()).unwrap();
//             }
//
//             // The right part of the frame.
//             self.stdout.write_all(VERT_BOUNDARY.as_bytes()).unwrap();
//             self.stdout.write_all(b"\n\r").unwrap();
//         }
//
//         // Write the lower part of the frame.
//         self.stdout.write_all(BOTTOM_LEFT_CORNER.as_bytes()).unwrap();
//         for _ in 0..self.width {
//             self.stdout.write_all(HORZ_BOUNDARY.as_bytes()).unwrap();
//         }
//         self.stdout.write_all(BOTTOM_RIGHT_CORNER.as_bytes()).unwrap();
//
//         write!(self.stdout, "{}", cursor::Goto(self.x + 2, self.y + 2)).unwrap();
//         self.stdout.flush().unwrap();
//
//         // Reset the grid.
//         for i in 0..self.grid.len() {
//             // Fill it with random, concealed fields.
//             self.grid[i] = Cell {
//                 mine: false,
//                 revealed: false,
//                 observed: false,
//                 flagged: false,
//             };
//
//             self.points = 0;
//         }
//     }
//
//     /// Get the value of a cell.
//     ///
//     /// The value represent the sum of adjacent cells containing mines. A cell of value, 0, is
//     /// called "free".
//     fn val(&mut self, x: u16, y: u16) -> u8 {
//         // To avoid nightly version, we manually sum the adjacent mines.
//         let mut res = 0;
//         for &(x, y) in self.adjacent(x, y).iter() {
//             res += self.get(x, y).mine as u8;
//         }
//         res
//     }
//
//     /// Reveal the cell, _c_.
//     ///
//     /// This will recursively reveal free cells, until non-free cell is reached, terminating the
//     /// current recursion descendant.
//     fn reveal(&mut self, x: u16, y: u16) {
//         let v = self.val(x, y);
//
//         self.get_mut(x, y).revealed = true;
//
//         write!(self.stdout, "{}", cursor::Goto(x + 2, y + 2)).unwrap();
//
//         if v == 0 {
//             // If the cell is free, simply put a space on the position.
//             self.stdout.write_all(b" ").unwrap();
//
//             // Recursively reveal adjacent cells until a non-free cel is reached.
//             for &(x, y) in self.adjacent(x, y).iter() {
//                 if !self.get(x, y).revealed && !self.get(x, y).mine {
//                     self.reveal(x, y);
//                 }
//             }
//         } else {
//             // Aww. The cell was not free. Print the value instead.
//             self.stdout.write_all(&[b'0' + v]).unwrap();
//         }
//     }
//
//     /// Print the point count.
//     fn print_points(&mut self) {
//         let height = self.height();
//         write!(self.stdout, "{}", cursor::Goto(3, height + 2)).unwrap();
//         self.stdout.write_all(self.points.to_string().as_bytes()).unwrap();
//     }
//
//     /// Reveal all the fields, printing where the mines were.
//     fn reveal_all(&mut self) {
//         write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();
//
//         for y in 0..self.height() {
//             for x in 0..self.width {
//                 write!(self.stdout, "{}", cursor::Goto(x + 2, y + 2)).unwrap();
//                 if self.get(x, y).mine {
//                     self.stdout.write_all(MINE.as_bytes()).unwrap();
//                 }
//             }
//         }
//     }
//
//     /// Game over!
//     fn game_over(&mut self) {
//         //Goto top left corner
//         write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();
//
//         self.stdout.write_all(GAME_OVER.as_bytes()).unwrap();
//         self.stdout.flush().unwrap();
//
//         loop {
//             // Repeatedly read a single byte.
//             match self.stdin.next().unwrap().unwrap() {
//                 Key::Char('r') => {
//                     // Replay!
//                     self.restart();
//                     return;
//                 },
//                 Key::Char('q') => return,
//                 _ => {},
//             }
//         }
//     }
//
//     /// Restart (replay) the game.
//     fn restart(&mut self) {
//         self.reset();
//         self.start();
//     }
//
//     /// Calculate the adjacent cells.
//     fn adjacent(&self, x: u16, y: u16) -> [(u16, u16); 8] {
//         let left = self.left(x);
//         let right = self.right(x);
//         let up = self.up(y);
//         let down = self.down(y);
//
//         [
//             // Left-up
//             (left, up),
//             // Up
//             (x, up),
//             // Right-up
//             (right, up),
//             // Left
//             (left, y),
//             // Right
//             (right, y),
//             // Left-down
//             (left, down),
//             // Down
//             (x, down),
//             // Right-down
//             (right, down)
//         ]
//     }
//
//     /// Calculate the height (number of rows) of the grid.
//     fn height(&self) -> u16 {
//         (self.grid.len() / self.width as usize) as u16
//     }
//
//     /// Calculate the y coordinate of the cell "above" a given y coordinate.
//     ///
//     /// This wraps when _y = 0_.
//     fn up(&self, y: u16) -> u16 {
//         if y == 0 {
//             // Upper bound reached. Wrap around.
//             self.height() - 1
//         } else {
//             y - 1
//         }
//     }
//     /// Calculate the y coordinate of the cell "below" a given y coordinate.
//     ///
//     /// This wraps when _y = h - 1_.
//     fn down(&self, y: u16) -> u16 {
//         if y + 1 == self.height() {
//             // Lower bound reached. Wrap around.
//             0
//         } else {
//             y + 1
//         }
//     }
//     /// Calculate the x coordinate of the cell "left to" a given x coordinate.
//     ///
//     /// This wraps when _x = 0_.
//     fn left(&self, x: u16) -> u16 {
//         if x == 0 {
//             // Lower bound reached. Wrap around.
//             self.width - 1
//         } else {
//             x - 1
//         }
//     }
//     /// Calculate the x coordinate of the cell "left to" a given x coordinate.
//     ///
//     /// This wraps when _x = w - 1_.
//     fn right(&self, x: u16) -> u16 {
//         if x + 1 == self.width {
//             // Upper bound reached. Wrap around.
//             0
//         } else {
//             x + 1
//         }
//     }
// }
//

type Pod = kubectl::Pod<kubectl::PodLabels>;
type Pods = Vec<Pod>;

struct State {
    pods: Option<Pods>,
    index_highlighted: usize,
    error: Option<anyhow::Error>,
    update_id: u64,
}

impl State {
    pub fn new () -> Self {
        Self {
            index_highlighted: 0,
            pods: None,
            error: None,
            update_id: 0,
        }
    }

    pub fn get_pods(self: &mut Self) -> Result<()> {
        let res = process::Command::new("kubectl")
            .env("KUBECONFIG", "/Users/baspar/.kube/config-multipass")
            .arg("get").arg("pods")
            .arg("-n").arg("mlisa-core")
            .arg("-o").arg("json")
            .output()?;

        let res = res.stdout.as_slice();
        let res = std::str::from_utf8(res)?;
        let res: kubectl::Response<kubectl::Labels> = serde_json::from_str(res)?;
        let mut pods: Vec<Pod<>> = res.items
            .iter()
            .filter_map(|item| {
                match &item.metadata.labels {
                    kubectl::Labels::PodLabels (labels)  => {
                        Some(kubectl::Pod {
                            status: item.status.clone(),
                            spec: item.spec.clone(),
                            metadata: kubectl::MetaData {
                                labels: labels.clone(),
                                name: item.metadata.name.clone()
                            }
                        })
                    },
                    kubectl::Labels::OtherLabels {} => None
                }
            })
            .collect();
        pods.sort_by_key(|pod| pod.metadata.name.clone());
        self.pods = Some(pods);
        self.update_id += 1;
        Ok(())
    }

    pub fn render(self: &mut Self, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
        let mut stdout = stdout.lock();
        write!(stdout, "{}", clear::All).unwrap();
        write!(stdout, "{}", cursor::Goto(1, 0)).unwrap();
        write!(stdout, "Update #{:?}", self.update_id).unwrap();
        if let Some(err) = &self.error {
            write!(stdout, "Error: {:?}", err).unwrap();
        }

        if let Some(pods) = &self.pods {
            let columns: Vec<Box<dyn Fn(&Pod) -> Option<String>>> = vec![
                Box::new(|pod| Some(pod.metadata.name.clone())),
                Box::new(|pod| match pod.status.phase.as_str() {
                    "Running" => Some(format!("{}Running{}", color::Fg(color::Green), color::Fg(color::Reset))),
                    "Pending" => Some(format!("{}Pending{}", color::Fg(color::Yellow), color::Fg(color::Reset))),
                    text      => Some(format!("{}{}{}", color::Fg(color::Red), text, color::Fg(color::Reset))),
                }),
                Box::new(|pod| pod.metadata.labels.patched.clone()),
            ];

            for index in 0..pods.len() {
                    write!(stdout, "{}", cursor::Goto(1, (index + 2) as u16)).unwrap();
                    write!(stdout, "{} ", if index == self.index_highlighted { ">" } else { " " }).unwrap();
            }

            let columns_x: Vec<u16> = columns
                .iter()
                .map(|column_fn| pods
                    .iter()
                    .map(|pod| column_fn(pod)
                        .unwrap_or_else(|| "".to_string())
                        .len() as u16
                    )
                    .max()
                    .unwrap()
                )
                .collect();

            for (index, pod) in pods.iter().enumerate() {
                let mut x = 3;
                for (col_index, column_fn) in columns.iter().enumerate() {
                    if let Some(value) = column_fn(pod) {
                        write!(stdout, "{}{}",
                            cursor::Goto(
                                x,
                                (index + 2) as u16
                            ),
                            value).unwrap();
                    }
                    x += columns_x[col_index] + 3;
                }
            }
        } else {
            write!(stdout, "Nothing").unwrap();
        }
        stdout.flush().unwrap();
    }
}

enum Event {
    Render,
    Quit
}

fn handle_input(state: Arc<Mutex<State>>, send: std::sync::mpsc::Sender<Event>) -> Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut keys = stdin.keys();
    loop {
        match keys.next().unwrap().unwrap() {
            Key::Down | Key::Char('j') => {
                let mut state = state.lock().unwrap();
                if let Some(pods) = &state.pods {
                    if !pods.is_empty() {
                        state.index_highlighted = (pods.len() - 1).min(state.index_highlighted + 1);
                    }
                }
                send.send(Event::Render)?;
            },
            Key::Up | Key::Char('k') => {
                let mut state = state.lock().unwrap();
                if state.index_highlighted > 0 {
                    state.index_highlighted -= 1;
                }
                send.send(Event::Render)?;
            },
            Key::Char('q') => {
                send.send(Event::Quit)?;
                break
            },
            _ => {},
        }
    }
    Ok(())
}

fn handle_pull(state: Arc<Mutex<State>>, send: std::sync::mpsc::Sender<Event>) -> Result<()> {
    loop {
        if let Ok(mut state) = state.lock() {
            if let Err(error) = state.get_pods() {
                state.error = Some(error);
            }
            send.send(Event::Render)?;
        }
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
    Ok(())
}

fn main() -> Result<()> {
    let (sender, receiver) = channel();

    let state = Arc::new(Mutex::new(State::new()));

    let s = state.clone();
    let send = sender.clone();
    std::thread::spawn(move || {
        handle_input(s, send).unwrap();
    });

    let s = state.clone();
    std::thread::spawn(move || {
        handle_pull(s, sender).unwrap();
    });

    let mut stdout = io::stdout().into_raw_mode()?;
    state.lock().unwrap().render(&mut stdout);
    for event in receiver {
        match event {
            Event::Quit => {break},
            Event::Render => {
                state.lock().unwrap().render(&mut stdout);
            }
        }
    }

    Ok(())
}

