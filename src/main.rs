//! This crate is a simple implementation of minesweeper. It is carefully documented to encourage
//! newbies to add new games to the repository.

extern crate termion;

use termion::{color, clear, cursor};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

// use std::env;
use std::io::{self,Write};
use std::process;

mod kubectl;

use anyhow::Result;

use std::sync::{Arc,Mutex};
use std::sync::mpsc::channel;

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

    pub fn set_pods(self: &mut Self, mut pods: Pods<>) {
        pods.sort_by_key(|pod| pod.metadata.name.clone());
        self.pods = Some(pods);
        self.update_id += 1;
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
                    .unwrap_or_else(|| 0)
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
    while let Ok(key) = keys.next().unwrap() {
        match key {
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

fn get_pods () -> Result<Pods<>> {
    let res = process::Command::new("kubectl")
        .env("KUBECONFIG", "/Users/baspar/.kube/config-multipass")
        .arg("get").arg("pods")
        .arg("-n").arg("mlisa-core")
        .arg("-o").arg("json")
        .output()?;

    let res = res.stdout.as_slice();
    let res = std::str::from_utf8(res)?;
    let res: kubectl::Response<kubectl::Labels> = serde_json::from_str(res)?;
    let pods: Vec<Pod<>> = res.items
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

    Ok(pods)
}

fn handle_pull(state: Arc<Mutex<State>>, send: std::sync::mpsc::Sender<Event>) -> Result<()> {
    loop {
        let res = get_pods();
        if let Ok(mut state) = state.lock() {
            match res {
                Err(error) => state.error = Some(error),
                Ok(pods) => state.set_pods(pods)
            }
            send.send(Event::Render)?;
        }
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
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
