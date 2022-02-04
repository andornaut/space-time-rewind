use super::command::Command;
use crossterm::event::{read, Event};
use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
};

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
    thread::spawn(move || loop {
        // Blocking read
        if let Event::Key(key) = read().expect("Can read key events") {
            let command = Command::from(key);
            if let Command::Continue = command {
                continue;
            }
            tx.send(command).unwrap();
        }
    });
}
