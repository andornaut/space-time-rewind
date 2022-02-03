use super::command::Command;
use crossterm::event::{poll, read, Event};
use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
    time::Duration,
};

const INPUT_POLL_TIMEOUT_MS: u64 = 50;

pub fn receive_input_commands(rx: &Receiver<Command>) -> Vec<Command> {
    let mut commands = Vec::new();
    loop {
        let command = rx.try_recv();
        if command.is_err() {
            break;
        }
        let command = command.expect("Can receive a command from the input thread");
        commands.push(command);
    }
    commands
}

pub fn send_input_commands(tx: Sender<Command>) {
    let timeout = Duration::from_millis(INPUT_POLL_TIMEOUT_MS);
    thread::spawn(move || loop {
        if poll(timeout).expect("Can poll for input") {
            // `poll()` returned true, so an event is available,
            // so the following call to `read()` will not block.
            if let Event::Key(key) = read().expect("Can read key events") {
                tx.send(Command::from(key)).unwrap();
            }
        }
    });
}
