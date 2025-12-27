mod common;

use vm::{VirtualMachine, process};

use common::*;

fn running_vm(vm: &mut VirtualMachine) {
    for process in &mut vm.processes {
        if process.state() == process::State::NoInstruction {
            process.fetch_decode(&mut vm.arena, vm.cycle_count);
        }
    }
    vm.cycle();
    vm.cycle_logic();
    vm.cycle_count += 1;
}

#[test]
fn cycles_add() {
    let mut vm = build_vm("pierino_add");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (23990, 1036),
        (35397, 986),
        (46254, 936),
        (56561, 886),
        (66318, 836),
        (75525, 786),
        (84182, 736),
        (92289, 686),
        (99846, 636),
        (106853, 586),
        (113310, 536),
        (119217, 486),
        (124574, 436),
        (129381, 386),
        (133638, 336),
        (137345, 286),
        (140502, 236),
        (143109, 186),
        (145166, 136),
        (146673, 86),
        (147630, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count != target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 147778);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino add"
    );
}

#[test]
fn cycles_crab() {
    let mut vm = build_vm("crab");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![(1, 1536), (100, 1536), (1000, 1536), (3073, 1536)];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 3074);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "walker"
    );
}
#[test]
fn cycles_ldi_ind_reg() {
    let mut vm = build_vm("pierino_ldi_ind_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (21342, 1236),
        (34949, 1186),
        (48006, 1136),
        (60513, 1086),
        (72470, 1036),
        (83877, 986),
        (94734, 936),
        (105041, 886),
        (114798, 836),
        (124005, 786),
        (132662, 736),
        (140769, 686),
        (148326, 636),
        (155333, 586),
        (161790, 536),
        (167697, 486),
        (173054, 436),
        (177861, 386),
        (182118, 336),
        (185825, 286),
        (188982, 236),
        (191589, 186),
        (193646, 136),
        (195153, 86),
        (196110, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 196221);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_lldi_reg_reg_reg() {
    let mut vm = build_vm("pierino_lldi_reg_reg_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (16907, 1486),
        (33264, 1436),
        (49071, 1386),
        (64328, 1336),
        (79035, 1286),
        (93192, 1236),
        (106799, 1186),
        (119856, 1136),
        (132363, 1086),
        (144320, 1036),
        (155727, 986),
        (166584, 936),
        (176891, 886),
        (186648, 836),
        (195855, 786),
        (204512, 736),
        (212619, 686),
        (220176, 636),
        (227183, 586),
        (233640, 536),
        (239547, 486),
        (244904, 436),
        (249711, 386),
        (253968, 336),
        (257675, 286),
        (260832, 236),
        (263439, 186),
        (265496, 136),
        (267003, 86),
        (267960, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 267997);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_sti_reg_reg_dir() {
    let mut vm = build_vm("pierino_sti_reg_reg_dir");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (21342, 1236),
        (34949, 1186),
        (48006, 1136),
        (60513, 1086),
        (72470, 1036),
        (83877, 986),
        (94734, 936),
        (105041, 886),
        (114798, 836),
        (124005, 786),
        (132662, 736),
        (140769, 686),
        (148326, 636),
        (155333, 586),
        (161790, 536),
        (167697, 486),
        (173054, 436),
        (177861, 386),
        (182118, 336),
        (185825, 286),
        (188982, 236),
        (191589, 186),
        (193646, 136),
        (195153, 86),
        (196110, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 196221);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]
fn cycles_empty_player() {
    let mut vm = build_vm("empty_player");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![(1, 1536), (100, 1536), (1000, 1536), (1536, 1536)];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 1537);
    assert!(vm.winners.is_empty());
}
#[test]

fn cycles_ldi_reg_dir() {
    let mut vm = build_vm("pierino_ldi_reg_dir");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (21342, 1236),
        (34949, 1186),
        (48006, 1136),
        (60513, 1086),
        (72470, 1036),
        (83877, 986),
        (94734, 936),
        (105041, 886),
        (114798, 836),
        (124005, 786),
        (132662, 736),
        (140769, 686),
        (148326, 636),
        (155333, 586),
        (161790, 536),
        (167697, 486),
        (173054, 436),
        (177861, 386),
        (182118, 336),
        (185825, 286),
        (188982, 236),
        (191589, 186),
        (193646, 136),
        (195153, 86),
        (196110, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 196221);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_or_ind_ind() {
    let mut vm = build_vm("pierino_or_ind_ind");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (16081, 886),
        (16968, 836),
        (17805, 786),
        (26462, 736),
        (34569, 686),
        (42126, 636),
        (49133, 586),
        (55590, 536),
        (61497, 486),
        (66854, 436),
        (71661, 386),
        (75918, 336),
        (79625, 286),
        (82782, 236),
        (85389, 186),
        (87446, 136),
        (88953, 86),
        (89910, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 90169);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_sti_reg_reg_reg() {
    let mut vm = build_vm("pierino_sti_reg_reg_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (21342, 1236),
        (34949, 1186),
        (48006, 1136),
        (60513, 1086),
        (72470, 1036),
        (83877, 986),
        (94734, 936),
        (105041, 886),
        (114798, 836),
        (124005, 786),
        (132662, 736),
        (140769, 686),
        (148326, 636),
        (155333, 586),
        (161790, 536),
        (167697, 486),
        (173054, 436),
        (177861, 386),
        (182118, 336),
        (185825, 286),
        (188982, 236),
        (191589, 186),
        (193646, 136),
        (195153, 86),
        (196110, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 196221);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_ldi_reg_reg() {
    let mut vm = build_vm("pierino_ldi_reg_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (21342, 1236),
        (34949, 1186),
        (48006, 1136),
        (60513, 1086),
        (72470, 1036),
        (83877, 986),
        (94734, 936),
        (105041, 886),
        (114798, 836),
        (124005, 786),
        (132662, 736),
        (140769, 686),
        (148326, 636),
        (155333, 586),
        (161790, 536),
        (167697, 486),
        (173054, 436),
        (177861, 386),
        (182118, 336),
        (185825, 286),
        (188982, 236),
        (191589, 186),
        (193646, 136),
        (195153, 86),
        (196110, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 196221);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_or_ind_reg() {
    let mut vm = build_vm("pierino_or_ind_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (16081, 886),
        (16968, 836),
        (17805, 786),
        (26462, 736),
        (34569, 686),
        (42126, 636),
        (49133, 586),
        (55590, 536),
        (61497, 486),
        (66854, 436),
        (71661, 386),
        (75918, 336),
        (79625, 286),
        (82782, 236),
        (85389, 186),
        (87446, 136),
        (88953, 86),
        (89910, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 90169);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_st_reg() {
    let mut vm = build_vm("pierino_st_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (16081, 886),
        (16968, 836),
        (17805, 786),
        (26462, 736),
        (34569, 686),
        (42126, 636),
        (49133, 586),
        (55590, 536),
        (61497, 486),
        (66854, 436),
        (71661, 386),
        (75918, 336),
        (79625, 286),
        (82782, 236),
        (85389, 186),
        (87446, 136),
        (88953, 86),
        (89910, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 90169);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_and_ind_ind() {
    let mut vm = build_vm("pierino_and_ind_ind");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (16081, 886),
        (16968, 836),
        (17805, 786),
        (26462, 736),
        (34569, 686),
        (42126, 636),
        (49133, 586),
        (55590, 536),
        (61497, 486),
        (66854, 436),
        (71661, 386),
        (75918, 336),
        (79625, 286),
        (82782, 236),
        (85389, 186),
        (87446, 136),
        (88953, 86),
        (89910, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 90169);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}

#[test]
fn cycles_ld() {
    let mut vm = build_vm("pierino_ld");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (16081, 886),
        (16968, 836),
        (17805, 786),
        (26462, 736),
        (34569, 686),
        (42126, 636),
        (49133, 586),
        (55590, 536),
        (61497, 486),
        (66854, 436),
        (71661, 386),
        (75918, 336),
        (79625, 286),
        (82782, 236),
        (85389, 186),
        (87446, 136),
        (88953, 86),
        (89910, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 90169);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino ld"
    );
}

#[test]
fn cycles_or_reg_ind() {
    let mut vm = build_vm("pierino_or_reg_ind");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (16081, 886),
        (16968, 836),
        (17805, 786),
        (26462, 736),
        (34569, 686),
        (42126, 636),
        (49133, 586),
        (55590, 536),
        (61497, 486),
        (66854, 436),
        (71661, 386),
        (75918, 336),
        (79625, 286),
        (82782, 236),
        (85389, 186),
        (87446, 136),
        (88953, 86),
        (89910, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 90169);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_sub() {
    let mut vm = build_vm("pierino_sub");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (23990, 1036),
        (35397, 986),
        (46254, 936),
        (56561, 886),
        (66318, 836),
        (75525, 786),
        (84182, 736),
        (92289, 686),
        (99846, 636),
        (106853, 586),
        (113310, 536),
        (119217, 486),
        (124574, 436),
        (129381, 386),
        (133638, 336),
        (137345, 286),
        (140502, 236),
        (143109, 186),
        (145166, 136),
        (146673, 86),
        (147630, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 147778);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_and_ind_reg() {
    let mut vm = build_vm("pierino_and_ind_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (16081, 886),
        (16968, 836),
        (17805, 786),
        (26462, 736),
        (34569, 686),
        (42126, 636),
        (49133, 586),
        (55590, 536),
        (61497, 486),
        (66854, 436),
        (71661, 386),
        (75918, 336),
        (79625, 286),
        (82782, 236),
        (85389, 186),
        (87446, 136),
        (88953, 86),
        (89910, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
}
#[test]

fn cycles_lld_dir_reg() {
    let mut vm = build_vm("pierino_lld_dir_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (16081, 886),
        (25838, 836),
        (35045, 786),
        (43702, 736),
        (51809, 686),
        (59366, 636),
        (66373, 586),
        (72830, 536),
        (78737, 486),
        (84094, 436),
        (88901, 386),
        (93158, 336),
        (96865, 286),
        (100022, 236),
        (102629, 186),
        (104686, 136),
        (106193, 86),
        (107150, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 107335);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_or_reg_reg() {
    let mut vm = build_vm("pierino_or_reg_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (17955, 886),
        (27712, 836),
        (36919, 786),
        (45576, 736),
        (53683, 686),
        (61240, 636),
        (68247, 586),
        (74704, 536),
        (80611, 486),
        (85968, 436),
        (90775, 386),
        (95032, 336),
        (98739, 286),
        (101896, 236),
        (104503, 186),
        (106560, 136),
        (108067, 86),
        (109024, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 109209);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_test() {
    let mut vm = build_vm("pierino_test");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (4511, 1436),
        (20318, 1386),
        (35575, 1336),
        (50282, 1286),
        (64439, 1236),
        (78046, 1186),
        (91103, 1136),
        (103610, 1086),
        (115567, 1036),
        (126974, 986),
        (137831, 936),
        (148138, 886),
        (157895, 836),
        (167102, 786),
        (175759, 736),
        (183866, 686),
        (191423, 636),
        (198430, 586),
        (204887, 536),
        (210794, 486),
        (216151, 436),
        (220958, 386),
        (225215, 336),
        (228922, 286),
        (232079, 236),
        (234686, 186),
        (236743, 136),
        (238250, 86),
        (239207, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 239244);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino_test"
    );
}
#[test]

fn cycles_and_reg_ind() {
    let mut vm = build_vm("pierino_and_reg_ind");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (16081, 886),
        (16968, 836),
        (17805, 786),
        (26462, 736),
        (34569, 686),
        (42126, 636),
        (49133, 586),
        (55590, 536),
        (61497, 486),
        (66854, 436),
        (71661, 386),
        (75918, 336),
        (79625, 286),
        (82782, 236),
        (85389, 186),
        (87446, 136),
        (88953, 86),
        (89910, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 90169);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_lldi_dir_dir_reg() {
    let mut vm = build_vm("pierino_lldi_dir_dir_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (16907, 1486),
        (33264, 1436),
        (49071, 1386),
        (64328, 1336),
        (79035, 1286),
        (93192, 1236),
        (106799, 1186),
        (119856, 1136),
        (132363, 1086),
        (144320, 1036),
        (155727, 986),
        (166584, 936),
        (176891, 886),
        (186648, 836),
        (195855, 786),
        (204512, 736),
        (212619, 686),
        (220176, 636),
        (227183, 586),
        (233640, 536),
        (239547, 486),
        (244904, 436),
        (249711, 386),
        (253968, 336),
        (257675, 286),
        (260832, 236),
        (263439, 186),
        (265496, 136),
        (267003, 86),
        (267960, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 267997);
    assert_eq!(*vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]
fn cycles_pierino() {
    let mut vm = build_vm("pierino");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)

    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (16081, 886),
        (16968, 836),
        (17805, 786),
        (18592, 736),
        (19329, 686),
        (26886, 636),
        (33893, 586),
        (40350, 536),
        (46257, 486),
        (51614, 436),
        (56421, 386),
        (60678, 336),
        (64385, 286),
        (67542, 236),
        (70149, 186),
        (72206, 136),
        (73713, 86),
        (74670, 36),
        (75077, 0),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 75078);
    assert_eq!(*vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_xor_ind_ind() {
    let mut vm = build_vm("pierino_xor_ind_ind");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (16081, 886),
        (16968, 836),
        (17805, 786),
        (26462, 736),
        (34569, 686),
        (42126, 636),
        (49133, 586),
        (55590, 536),
        (61497, 486),
        (66854, 436),
        (71661, 386),
        (75918, 336),
        (79625, 286),
        (82782, 236),
        (85389, 186),
        (87446, 136),
        (88953, 86),
        (89910, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 90169);
    assert_eq!(*vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_and_reg_reg() {
    let mut vm = build_vm("pierino_and_reg_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (17955, 886),
        (27712, 836),
        (36919, 786),
        (45576, 736),
        (53683, 686),
        (61240, 636),
        (68247, 586),
        (74704, 536),
        (80611, 486),
        (85968, 436),
        (90775, 386),
        (95032, 336),
        (98739, 286),
        (101896, 236),
        (104503, 186),
        (106560, 136),
        (108067, 86),
        (109024, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 109209);
    assert_eq!(*vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_lldi_dir_reg_reg() {
    let mut vm = build_vm("pierino_lldi_dir_reg_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (16907, 1486),
        (33264, 1436),
        (49071, 1386),
        (64328, 1336),
        (79035, 1286),
        (93192, 1236),
        (106799, 1186),
        (119856, 1136),
        (132363, 1086),
        (144320, 1036),
        (155727, 986),
        (166584, 936),
        (176891, 886),
        (186648, 836),
        (195855, 786),
        (204512, 736),
        (212619, 686),
        (220176, 636),
        (227183, 586),
        (233640, 536),
        (239547, 486),
        (244904, 436),
        (249711, 386),
        (253968, 336),
        (257675, 286),
        (260832, 236),
        (263439, 186),
        (265496, 136),
        (267003, 86),
        (267960, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 267997);
    assert_eq!(*vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_st_ind() {
    let mut vm = build_vm("pierino_st_ind");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (16081, 886),
        (16968, 836),
        (17805, 786),
        (26462, 736),
        (34569, 686),
        (42126, 636),
        (49133, 586),
        (55590, 536),
        (61497, 486),
        (66854, 436),
        (71661, 386),
        (75918, 336),
        (79625, 286),
        (82782, 236),
        (85389, 186),
        (87446, 136),
        (88953, 86),
        (89910, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 90169);
    assert_eq!(*vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_xor_ind_reg() {
    let mut vm = build_vm("pierino_xor_ind_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (16081, 886),
        (16968, 836),
        (17805, 786),
        (26462, 736),
        (34569, 686),
        (42126, 636),
        (49133, 586),
        (55590, 536),
        (61497, 486),
        (66854, 436),
        (71661, 386),
        (75918, 336),
        (79625, 286),
        (82782, 236),
        (85389, 186),
        (87446, 136),
        (88953, 86),
        (89910, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 90169);
    assert_eq!(*vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]
fn cycles_fork() {
    let mut vm = build_vm("pierino_fork");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (16081, 886),
        (16968, 836),
        (17805, 786),
        (18592, 736),
        (19329, 686),
        (20016, 636),
        (20653, 586),
        (21240, 536),
        (21777, 486),
        (22264, 436),
        (22701, 386),
        (23088, 336),
        (26795, 286),
        (29952, 236),
        (32559, 186),
        (34616, 136),
        (36123, 86),
        (37080, 36),
        (37487, 0),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 37488);
    assert_eq!(*vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_lldi_ind_dir_reg() {
    let mut vm = build_vm("pierino_lldi_ind_dir_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (16907, 1486),
        (33264, 1436),
        (49071, 1386),
        (64328, 1336),
        (79035, 1286),
        (93192, 1236),
        (106799, 1186),
        (119856, 1136),
        (132363, 1086),
        (144320, 1036),
        (155727, 986),
        (166584, 936),
        (176891, 886),
        (186648, 836),
        (195855, 786),
        (204512, 736),
        (212619, 686),
        (220176, 636),
        (227183, 586),
        (233640, 536),
        (239547, 486),
        (244904, 436),
        (249711, 386),
        (253968, 336),
        (257675, 286),
        (260832, 236),
        (263439, 186),
        (265496, 136),
        (267003, 86),
        (267960, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 267997);
    assert_eq!(*vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_sti_reg_dir_dir() {
    let mut vm = build_vm("pierino_sti_reg_dir_dir");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (21342, 1236),
        (34949, 1186),
        (48006, 1136),
        (60513, 1086),
        (72470, 1036),
        (83877, 986),
        (94734, 936),
        (105041, 886),
        (114798, 836),
        (124005, 786),
        (132662, 736),
        (140769, 686),
        (148326, 636),
        (155333, 586),
        (161790, 536),
        (167697, 486),
        (173054, 436),
        (177861, 386),
        (182118, 336),
        (185825, 286),
        (188982, 236),
        (191589, 186),
        (193646, 136),
        (195153, 86),
        (196110, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 196221);
    assert_eq!(*vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_xor_reg_ind() {
    let mut vm = build_vm("pierino_xor_reg_ind");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (16081, 886),
        (16968, 836),
        (17805, 786),
        (26462, 736),
        (34569, 686),
        (42126, 636),
        (49133, 586),
        (55590, 536),
        (61497, 486),
        (66854, 436),
        (71661, 386),
        (75918, 336),
        (79625, 286),
        (82782, 236),
        (85389, 186),
        (87446, 136),
        (88953, 86),
        (89910, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 90169);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_ldi_dir_dir() {
    let mut vm = build_vm("pierino_ldi_dir_dir");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (10946, 1186),
        (24003, 1136),
        (36510, 1086),
        (48467, 1036),
        (59874, 986),
        (70731, 936),
        (81038, 886),
        (90795, 836),
        (100002, 786),
        (108659, 736),
        (116766, 686),
        (124323, 636),
        (131330, 586),
        (137787, 536),
        (143694, 486),
        (149051, 436),
        (153858, 386),
        (158115, 336),
        (161822, 286),
        (164979, 236),
        (167586, 186),
        (169643, 136),
        (171150, 86),
        (172107, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 172144);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_lldi_ind_reg_reg() {
    let mut vm = build_vm("pierino_lldi_ind_reg_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (16907, 1486),
        (33264, 1436),
        (49071, 1386),
        (64328, 1336),
        (79035, 1286),
        (93192, 1236),
        (106799, 1186),
        (119856, 1136),
        (132363, 1086),
        (144320, 1036),
        (155727, 986),
        (166584, 936),
        (176891, 886),
        (186648, 836),
        (195855, 786),
        (204512, 736),
        (212619, 686),
        (220176, 636),
        (227183, 586),
        (233640, 536),
        (239547, 486),
        (244904, 436),
        (249711, 386),
        (253968, 336),
        (257675, 286),
        (260832, 236),
        (263439, 186),
        (265496, 136),
        (267003, 86),
        (267960, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 267997);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_sti_reg_dir_reg() {
    let mut vm = build_vm("pierino_sti_reg_dir_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (21342, 1236),
        (34949, 1186),
        (48006, 1136),
        (60513, 1086),
        (72470, 1036),
        (83877, 986),
        (94734, 936),
        (105041, 886),
        (114798, 836),
        (124005, 786),
        (132662, 736),
        (140769, 686),
        (148326, 636),
        (155333, 586),
        (161790, 536),
        (167697, 486),
        (173054, 436),
        (177861, 386),
        (182118, 336),
        (185825, 286),
        (188982, 236),
        (191589, 186),
        (193646, 136),
        (195153, 86),
        (196110, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 196221);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_xor_reg_reg() {
    let mut vm = build_vm("pierino_xor_reg_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (17955, 886),
        (27712, 836),
        (36919, 786),
        (45576, 736),
        (53683, 686),
        (61240, 636),
        (68247, 586),
        (74704, 536),
        (80611, 486),
        (85968, 436),
        (90775, 386),
        (95032, 336),
        (98739, 286),
        (101896, 236),
        (104503, 186),
        (106560, 136),
        (108067, 86),
        (109024, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        // here problem 1536 != 1486
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 109209);
    assert_eq!(*vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_ldi_dir_reg() {
    let mut vm = build_vm("pierino_ldi_dir_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (21342, 1236),
        (34949, 1186),
        (48006, 1136),
        (60513, 1086),
        (72470, 1036),
        (83877, 986),
        (94734, 936),
        (105041, 886),
        (114798, 836),
        (124005, 786),
        (132662, 736),
        (140769, 686),
        (148326, 636),
        (155333, 586),
        (161790, 536),
        (167697, 486),
        (173054, 436),
        (177861, 386),
        (182118, 336),
        (185825, 286),
        (188982, 236),
        (191589, 186),
        (193646, 136),
        (195153, 86),
        (196110, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 196221);
    assert_eq!(*vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_lld_ind_reg() {
    let mut vm = build_vm("pierino_lld_ind_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (9709, 1186),
        (10896, 1136),
        (12033, 1086),
        (13120, 1036),
        (14157, 986),
        (15144, 936),
        (16081, 886),
        (25838, 836),
        (35045, 786),
        (43702, 736),
        (51809, 686),
        (59366, 636),
        (66373, 586),
        (72830, 536),
        (78737, 486),
        (84094, 436),
        (88901, 386),
        (93158, 336),
        (96865, 286),
        (100022, 236),
        (102629, 186),
        (104686, 136),
        (106193, 86),
        (107150, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 107335);
    assert_eq!(*vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_sti_reg_ind_dir() {
    let mut vm = build_vm("pierino_sti_reg_ind_dir");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![(1537, 1536), (3024, 1536)];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 3074);
    assert_eq!(*vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}

#[test]
fn cycles_ldi_ind_dir() {
    let mut vm = build_vm("pierino_ldi_ind_dir");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (1537, 1486),
        (3024, 1436),
        (4461, 1386),
        (5848, 1336),
        (7185, 1286),
        (8472, 1236),
        (10946, 1186),
        (24003, 1136),
        (36510, 1086),
        (48467, 1036),
        (59874, 986),
        (70731, 936),
        (81038, 886),
        (90795, 836),
        (100002, 786),
        (108659, 736),
        (116766, 686),
        (124323, 636),
        (131330, 586),
        (137787, 536),
        (143694, 486),
        (149051, 436),
        (153858, 386),
        (158115, 336),
        (161822, 286),
        (164979, 236),
        (167586, 186),
        (169643, 136),
        (171150, 86),
        (172107, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 172144);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_lldi_reg_dir_reg() {
    let mut vm = build_vm("pierino_lldi_reg_dir_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![
        (16907, 1486),
        (33264, 1436),
        (49071, 1386),
        (64328, 1336),
        (79035, 1286),
        (93192, 1236),
        (106799, 1186),
        (119856, 1136),
        (132363, 1086),
        (144320, 1036),
        (155727, 986),
        (166584, 936),
        (176891, 886),
        (186648, 836),
        (195855, 786),
        (204512, 736),
        (212619, 686),
        (220176, 636),
        (227183, 586),
        (233640, 536),
        (239547, 486),
        (244904, 436),
        (249711, 386),
        (253968, 336),
        (257675, 286),
        (260832, 236),
        (263439, 186),
        (265496, 136),
        (267003, 86),
        (267960, 36),
    ];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 267997);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
#[test]

fn cycles_sti_reg_ind_reg() {
    let mut vm = build_vm("pierino_sti_reg_ind_reg");

    // A list of checkpoints: (target_cycle, expected_cycles_to_die)
    let checkpoints = vec![(1, 1536), (100, 1536), (1000, 1536), (3073, 1536)];

    for (target_cycle, expected_die) in checkpoints {
        // Run the VM until we reach the target cycle
        while vm.cycle_count < target_cycle {
            running_vm(&mut vm);
        }

        // Assertions at the specific checkpoint
        assert_eq!(
            vm.cycle_count, target_cycle,
            "Cycle count mismatch at checkpoint"
        );
        running_vm(&mut vm);
        assert_eq!(
            vm.cycles_to_die, expected_die,
            "cycles_to_die mismatch at cycle {}",
            target_cycle
        );
    }
    while vm.processes_alive() {
        running_vm(&mut vm);
    }
    assert_eq!(vm.cycle_count, 3074);
    assert_eq!(vm.winners.iter().next().unwrap() * -1, 1);
    assert_eq!(
        &vm.get_player(*vm.winners.iter().next().unwrap()).unwrap(),
        "pierino"
    );
}
