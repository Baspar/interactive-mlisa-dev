use super::kubectl;
use termion::{color, clear, cursor};
use std::io::Write;

pub type Pod = kubectl::Pod<kubectl::PodLabels>;
pub type Pods = Vec<Pod>;

pub struct State {
    pub pods: Option<Pods>,
    pub index_highlighted: usize,
    pub error: Option<anyhow::Error>,
    pub update_id: u64,
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

pub enum Event {
    Render,
    Quit
}
