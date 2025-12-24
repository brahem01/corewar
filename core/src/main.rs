use std::env;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use vm::parse_arguments;
use vm::{Arena, MEM_SIZE, Process, VirtualMachine, VmSnapshot};

#[cfg(feature = "gui")]
use visualization::run_gui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments
    let args: Vec<String> = env::args().skip(1).collect();

    let players = parse_arguments(&args)?;
    let players_count = players.len();

    if players_count < 2 || players_count > 4 {
        eprintln!("Error: number of players must be between 2 and 4");
        std::process::exit(1);
    }

    println!("For this match the players will be:");
    for (i, player) in players.iter().enumerate() {
        println!(
            "Player {} ({} bytes): {} ({})",
            i + 1,
            player.size,
            player.name,
            player.comment
        );
    }

    // Init arena & processes
    let arena = Arena::new();

    let mut processes = Vec::with_capacity(players_count);
    for (i, player) in players.clone().iter().rev().enumerate() {
        let process = Process::new(
            player.clone().id,
            player.name.clone(),
            i + 1,
            MEM_SIZE / players_count * i,
        );
        processes.push(process)
    }

    let mut vm = VirtualMachine::create(arena, processes);

    for (i, player) in players.iter().enumerate() {
        vm.load_player(player.clone(), (MEM_SIZE / players_count) * i);
    }

    // Channel: VM → GUI
    let (tx, rx): (Sender<VmSnapshot>, Receiver<VmSnapshot>) = mpsc::channel();
    vm.set_sender(tx);

    // GUI must run on MAIN THREAD
    // VM can run in background thread
    // spawn VM in a background thread
    let mut vm_thread = vm;
    let _vm_handle = thread::spawn(move || {
        vm_thread.run();
    });

    // run GUI on main thread
    #[cfg(feature = "gui")]
    run_gui(rx);
    
    // optional: wait for VM thread to finish
    // Note: GUI may close first; VM can detect dropped channel
    //_vm_handle.join().unwrap();

    Ok(())
}
