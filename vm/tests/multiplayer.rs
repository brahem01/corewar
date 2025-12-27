mod common;
use common::Instruction::*;
use common::*;

#[test]
fn two() {
    let mut vm = build_vm_more(vec!["pierino_add", "pierino_add"]);
    // live 10
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[1].live_status.executed, true);
    assert_eq!(vm.processes[1].live_status.player_id, -1);
    assert_eq!(vm.processes[1].live_status.nbr_live, 1);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ld 5
    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[1].registers[2 - 1], 2);
    assert_eq!(vm.processes[0].registers[2 - 1], 2);

    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[1].registers[3 - 1], 3);
    assert_eq!(vm.processes[0].registers[3 - 1], 3);
    // add
    run_inst(&mut vm, Add);
    assert_eq!(vm.processes[1].registers[4 - 1], 5);
    assert_eq!(vm.processes[0].registers[4 - 1], 5);

    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[1].registers[3 - 1], 0);
    assert_eq!(vm.processes[1].pc.get(), 2048);
    assert_eq!(vm.processes[0].registers[3 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn three() {
    let mut vm = build_vm_more(vec!["pierino_add", "pierino_add", "pierino_add"]);
    // live 10
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[2].live_status.executed, true);
    assert_eq!(vm.processes[2].live_status.player_id, -1);
    assert_eq!(vm.processes[2].live_status.nbr_live, 1);
    assert_eq!(vm.processes[1].live_status.executed, true);
    assert_eq!(vm.processes[1].live_status.player_id, -1);
    assert_eq!(vm.processes[1].live_status.nbr_live, 1);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ld 5
    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[2].registers[2 - 1], 2);
    assert_eq!(vm.processes[1].registers[2 - 1], 2);
    assert_eq!(vm.processes[0].registers[2 - 1], 2);

    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[2].registers[3 - 1], 3);
    assert_eq!(vm.processes[1].registers[3 - 1], 3);
    assert_eq!(vm.processes[0].registers[3 - 1], 3);
    // add
    run_inst(&mut vm, Add);
    assert_eq!(vm.processes[2].registers[4 - 1], 5);
    assert_eq!(vm.processes[1].registers[4 - 1], 5);
    assert_eq!(vm.processes[0].registers[4 - 1], 5);

    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[2].registers[3 - 1], 0);
    assert_eq!(vm.processes[2].pc.get(), 2730);
    assert_eq!(vm.processes[1].registers[3 - 1], 0);
    assert_eq!(vm.processes[1].pc.get(), 1365);
    assert_eq!(vm.processes[0].registers[3 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}

#[test]
fn four() {
    let mut vm = build_vm_more(vec![
        "pierino_add",
        "pierino_add",
        "pierino_add",
        "pierino_add",
    ]);
    // live 10
    run_inst(&mut vm, Live);
    assert_eq!(vm.processes[3].live_status.executed, true);
    assert_eq!(vm.processes[3].live_status.player_id, -1);
    assert_eq!(vm.processes[3].live_status.nbr_live, 1);
    assert_eq!(vm.processes[2].live_status.executed, true);
    assert_eq!(vm.processes[2].live_status.player_id, -1);
    assert_eq!(vm.processes[2].live_status.nbr_live, 1);
    assert_eq!(vm.processes[1].live_status.executed, true);
    assert_eq!(vm.processes[1].live_status.player_id, -1);
    assert_eq!(vm.processes[1].live_status.nbr_live, 1);
    assert_eq!(vm.processes[0].live_status.executed, true);
    assert_eq!(vm.processes[0].live_status.player_id, -1);
    assert_eq!(vm.processes[0].live_status.nbr_live, 1);
    //ld 5
    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[3].registers[2 - 1], 2);
    assert_eq!(vm.processes[2].registers[2 - 1], 2);
    assert_eq!(vm.processes[1].registers[2 - 1], 2);
    assert_eq!(vm.processes[0].registers[2 - 1], 2);

    run_inst(&mut vm, Ld);
    assert_eq!(vm.processes[3].registers[3 - 1], 3);
    assert_eq!(vm.processes[2].registers[3 - 1], 3);
    assert_eq!(vm.processes[1].registers[3 - 1], 3);
    assert_eq!(vm.processes[0].registers[3 - 1], 3);
    // add
    run_inst(&mut vm, Add);
    assert_eq!(vm.processes[3].registers[4 - 1], 5);
    assert_eq!(vm.processes[2].registers[4 - 1], 5);
    assert_eq!(vm.processes[1].registers[4 - 1], 5);
    assert_eq!(vm.processes[0].registers[4 - 1], 5);

    run_inst(&mut vm, Ld);
    //ld 5 zjmp 20
    run_inst(&mut vm, Zjmp);
    assert_eq!(vm.processes[3].registers[3 - 1], 0);
    assert_eq!(vm.processes[3].pc.get(), 1024 * 3);
    assert_eq!(vm.processes[2].registers[3 - 1], 0);
    assert_eq!(vm.processes[2].pc.get(), 2048);
    assert_eq!(vm.processes[1].registers[3 - 1], 0);
    assert_eq!(vm.processes[1].pc.get(), 1024);
    assert_eq!(vm.processes[0].registers[3 - 1], 0);
    assert_eq!(vm.processes[0].pc.get(), 0);
}
