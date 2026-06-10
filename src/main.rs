use midir::{Ignore, MidiInput};
use note::Note;
use rodio::{
    Player,
    source::{SawtoothWave, Source},
};
use std::fmt;
use std::io::{Write, stdin, stdout};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{collections::VecDeque, error::Error};

mod note;

type Velocity = u8;

type Commands = Arc<Mutex<VecDeque<MidiEvent>>>;

#[derive(Debug, Clone)]
enum MidiEvent {
    KeyUp(Note, Velocity),
    KeyDown(Note, Velocity),
    Other,
}

impl fmt::Display for MidiEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            MidiEvent::KeyUp(note, velocity) => {
                write!(f, "KeyUp - {} with velocity {}", note, velocity)
            }
            MidiEvent::KeyDown(note, velocity) => {
                write!(f, "KeyDown - {} with velocity {}", note, velocity)
            }
            MidiEvent::Other => write!(f, "Other event"),
        }
    }
}

fn handle_midi_message(stamp: u64, message: &[u8], commands: &mut Commands) {
    let midi_event = match message[0] {
        0b1000_0000..=0b1000_1111 => MidiEvent::KeyUp(Note::from(message[1]), message[2]),
        0b1001_0000..=0b1001_1111 if message[2] == 0 => {
            MidiEvent::KeyUp(Note::from(message[1]), message[2])
        }
        0b1001_0000..=0b1001_1111 => MidiEvent::KeyDown(Note::from(message[1]), message[2]),
        _ => MidiEvent::Other,
    };

    println!("{}", midi_event);
    commands.lock().unwrap().push_back(midi_event);
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut midi_in = MidiInput::new("midir reading input")?;
    let commands = Arc::new(Mutex::new(VecDeque::new()));
    midi_in.ignore(Ignore::SysexAndActiveSense);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid input port selected")?
        }
    };
    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port)?;

    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        handle_midi_message,
        commands.clone(),
    )?;

    println!("Connection open, reading input from '{}'", in_port_name);

    let mut sink_handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    sink_handle.log_on_drop(false);
    let player = Player::connect_new(sink_handle.mixer());
    loop {
        let mut cur_commands = commands.lock().unwrap();

        if cur_commands.len() == 0 {
            continue;
        }

        let midi_event = cur_commands.pop_front().unwrap();

        // Basic playing
        match midi_event {
            MidiEvent::KeyDown(n, _) => {
                player.append(
                    SawtoothWave::new(Note::freq(n))
                        .amplify_decibel(-3.0)
                        .linear_gain_ramp(Duration::from_millis(10), 1.0, 0.0, false),
                );
            }
            MidiEvent::KeyUp(n, _) => {
                player.append(
                    SawtoothWave::new(Note::freq(n))
                        .amplify_decibel(-3.0)
                        .linear_gain_ramp(Duration::from_millis(10), 1.0, 0.0, false)
                        .take_duration(Duration::from_millis(10)),
                );
                player.skip_one();
            }
            MidiEvent::Other => break,
        };
    }

    Ok(())
}
