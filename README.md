# My Synth

This is a small sawtooth midi synth. It will read key on and key off midi messages from a midi port and play the corresponding sawtooth note until released. It can only play one note at a time so if multiple key events are given it may have strange behavior.

## Running

To build and run the project use the following command
`cargo run --release`

## Usage

The synth will ask you to choose a midi import port if there are multiple. It will provide a list with names of the ports. Simply enter the number to the left of the name in the console to choose that port.
