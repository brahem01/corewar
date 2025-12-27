use super::Process;
use crate::*;
use std::fmt::Display;
use std::fmt::Formatter;
impl Display for Process {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // create a table
        let mut table = Table::new();
        let mut table2 = Table::new();

        // add headers
        table.add_header("PC");
        table.add_header("Carry");
        table.add_header("Current Instruction");
        table.add_header("Remaining Cycles");
        table.add_header("Lives Status");
        table2.add_header("Registers");
        let registers_str = self
            .registers
            .iter()
            .enumerate()
            .map(|(i, val)| format!("R{}:{}", i + 1, val))
            .collect::<Vec<_>>()
            .join(", ");

        // prepare live status as string
        let live_status_str = format!(
            "executed: {}, player_id: {}, nbr_live: {}",
            self.live_status.executed, self.live_status.player_id, self.live_status.nbr_live
        );

        // prepare current instruction as string
        let current_inst_str = match &self.current_instruction {
            Some(inst) => format!("{:?}", inst),
            None => "None".to_string(),
        };

        // add row with all fields converted to strings
        table.add_row(&vec![
            self.pc.get().to_string(),
            self.carry.to_string(),
            current_inst_str,
            self.remaining_cycles.to_string(),
            live_status_str,
            //registers_str,
        ]);
        table2.add_row(&vec![registers_str]);

        // //print the table
        println!("{table}");
        // println!("{table2}");

        Ok(())
    }
}
