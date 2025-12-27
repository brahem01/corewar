use std::env;
use std::sync::mpsc::{ self, Receiver, Sender };
use std::thread;
use vm::visualization::gui::run_gui;
use vm::parse_arguments;
use vm::{ Arena, MEM_SIZE, Process, VirtualMachine, VmSnapshot };

// #[cfg(feature = "gui")]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The OS initialize the stack with arguments;
    // specifically, it will fill in the parameters to
    // the main() function, i.e., argc and the argv array.
    let args: Vec<String> = env::args().collect();

    let arena = Arena::new();

    let (players, cycles_to_stop, verbos) = parse_arguments(args)?;
    let players_count = players.len();
    // the loading process is done eagerly as old days
    // To understand how lazy loading of pieces of code and data works,
    // you’ll have to understand the machinery of paging and swapping,
    let mut processes = vec![];
    for (i, player) in players.clone().iter().enumerate() {
        let process = Process::new(player.clone().id, i, ((MEM_SIZE + 1) / players_count) * i,);

        // println!("{}", players[i]);
        // println!("{}", process);
        //////println!("{}", arena);
        processes.push(process);
    }
    let mut vm = VirtualMachine::create(
        arena.clone(),
        processes,
        players.clone(),
        cycles_to_stop,
        verbos
    );
    vm.print_match_intro(&players);
    for (i, player) in players.iter().enumerate() {
        vm.load_player(player.clone(), ((MEM_SIZE + 1) / players_count) * i);
    }

    if verbos {
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
        // #[cfg(feature = "gui")]
        run_gui(rx);
    }else{
        vm.run();
    }
    Ok(())
    // end of the game, declare winner or no winner
    // vm.declare_winner();
}