use vm::process::Process;
use std::env;
use vm::config::MEM_SIZE;
use vm::utils::parse_arguments;
use vm::arena::Arena;
use vm::VirtualMachine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The OS initialize the stack with arguments;
    // specifically, it will fill in the parameters to
    // the main() function, i.e., argc and the argv array.
    let args: Vec<String> = env::args().collect();

    let arena = Arena::new();

    let players = parse_arguments(&args)?;
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
    let players_count = players.len();

    // the loading process is done eagerly as old days
    // To understand how lazy loading of pieces of code and data works,
    // you’ll have to understand the machinery of paging and swapping,
    let mut processes = vec![];
    for (i, player) in players.clone().iter().enumerate() {
        let process = Process::new(
            player.clone().id,
            player.name.clone(),
            0,
            MEM_SIZE % players_count * i,
        );
        processes.push(process)
    }

    let mut vm = VirtualMachine::create(arena.clone(), processes);
    for (i, player) in players.iter().enumerate() {
        vm.load_player(player.clone(), (MEM_SIZE / players_count) * i);
    }
    vm.run();
    Ok(())
}
