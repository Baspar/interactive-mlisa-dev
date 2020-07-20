extern crate termion;

use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;

use std::io;
use std::process;

mod kubectl;

mod state;
use state::{State, Event, Pod, Pods};

use anyhow::Result;

use std::sync::{Arc,Mutex};
use std::sync::mpsc::channel;

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

fn get_pods () -> Result<Pods> {
    let res = process::Command::new("kubectl")
        .env("KUBECONFIG", "/Users/baspar/.kube/config-multipass")
        .arg("get").arg("pods")
        .arg("-n").arg("mlisa-core")
        .arg("-o").arg("json")
        .output()?;

    let res = res.stdout.as_slice();
    let res = std::str::from_utf8(res)?;
    let res: kubectl::Response<kubectl::Labels> = serde_json::from_str(res)?;
    let pods: Vec<Pod> = res.items
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
