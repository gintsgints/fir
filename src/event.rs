use std::thread::JoinHandle;

use crossterm::event::{read, KeyEvent, KeyEventKind};
use anyhow::{Result, anyhow};

enum Event {
    Error,
    Tick,
    Key(KeyEvent),
}

struct EventHandler {
    rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
    task: Option<JoinHandle<()>>,
}

impl EventHandler {
    fn new() -> Self {
        let tick_rate = std::time::Duration::from_millis(250);
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut interval = tokio::time::interval(tick_rate);
            loop {
                let delay = interval.tick();
                let crossterm_event = reader.next().fuse();
                if crossterm::event::poll(tick_rate).unwrap() {
                    match read().unwrap() {
                        crossterm::event::Event::Key(key) => {
                            if key.kind == KeyEventKind::Press {
                                tx.send(Event::Key(key)).unwrap();
                            };
                        }
                        _ => {}
                    }
                }
            }
        });

        Self { rx }
    }

    async fn next(&mut self) -> Result<Event> {
        self.rx
            .recv()
            .await.ok_or(anyhow!("Unable to get event"))
    }
}
