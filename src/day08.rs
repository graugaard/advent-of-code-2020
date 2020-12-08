use crate::day08::Operation::{Acc, Jmp, NoOp};
use crate::puzzle_input;
use std::collections::HashSet;

#[derive(PartialOrd, PartialEq, Debug)]
enum Operation {
    NoOp(i64),
    Acc(i64),
    Jmp(i64),
}

impl Operation {
    fn to_op(instruction: &str) -> Operation {
        let (op, val) = instruction.split_at(4);
        let val = val.trim_start_matches("+").parse::<i64>().unwrap();
        match op {
            "nop " => NoOp(val),
            "acc " => Acc(val),
            "jmp " => Jmp(val),
            _ => panic!("Unknown op"),
        }
    }
}

struct Computer<'a> {
    instructions: &'a [Operation],
    instr_ptr: i64,
    accumulator: i64,
    executed_steps: HashSet<usize>,
    halted: bool,
}

impl<'a> Computer<'a> {
    fn parse_program(instructions: &str) -> Vec<Operation> {
        instructions.lines().map(Operation::to_op).collect()
    }

    fn init(instructions: &'a [Operation]) -> Self {
        Computer {
            instructions,
            instr_ptr: 0,
            accumulator: 0,
            executed_steps: HashSet::new(),
            halted: false,
        }
    }

    fn execute_step(&mut self) {
        match self.instructions[self.instr_ptr as usize] {
            Acc(val) => {
                self.accumulator += val;
                self.instr_ptr += 1;
            }
            NoOp(_) => {
                self.instr_ptr += 1;
            }
            Jmp(offset) => {
                self.instr_ptr += offset;
            }
        }
    }

    fn instr_ptr(&self) -> i64 {
        self.instr_ptr
    }

    fn accumulated(&self) -> i64 {
        self.accumulator
    }

    fn execute_until_end(&mut self) {
        while !self.halted {
            if self.instr_ptr as usize >= self.instructions.len() {
                self.halted = true;
                return;
            }
            match self.executed_steps.contains(&(self.instr_ptr as usize)) {
                true => {
                    self.halted = true;
                }
                false => {
                    self.executed_steps.insert(self.instr_ptr as usize);
                    self.execute_step();
                }
            }
        }
    }

    fn ended_without_loop(&self) -> Option<bool> {
        if !self.halted {
            return None;
        }
        Some(self.instr_ptr as usize >= self.instructions.len())
    }

    fn halted(&self) -> bool {
        self.halted
    }
}

pub fn print_solution() {
    let puzzle = puzzle_input::read_input("day08");
    let instrs = Computer::parse_program(&puzzle);
    let mut comp = Computer::init(&instrs);
    comp.execute_until_end();

    println!("Day 08 Solution Part 1: {}", comp.accumulated());
    println!(
        "Day 08 Solution Part 2: {}",
        find_acc_of_non_loop_machine(&puzzle)
    );
}

fn find_acc_of_non_loop_machine(puzzle: &str) -> i64 {
    let mut instr = Computer::parse_program(puzzle);
    for i in 0..instr.len() {
        match instr[i] {
            NoOp(val) => {
                instr[i] = Jmp(val);
                let mut comp = Computer::init(&instr);
                comp.execute_until_end();
                if Some(true) == comp.ended_without_loop() {
                    return comp.accumulator;
                }
                instr[i] = NoOp(val);
            }
            Jmp(val) => {
                instr[i] = NoOp(val);
                let mut comp = Computer::init(&instr);
                comp.execute_until_end();
                if Some(true) == comp.ended_without_loop() {
                    return comp.accumulator;
                }
                instr[i] = Jmp(val);
            }
            _ => {}
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::day08::Operation::{Acc, Jmp, NoOp};
    use crate::day08::{Computer, Operation};

    #[test]
    fn translating_nop_op() {
        assert_eq!(Operation::to_op("nop +0"), NoOp(0));
    }

    #[test]
    fn translating_acc_op() {
        assert_eq!(Operation::to_op("acc +2"), Acc(2));
    }

    #[test]
    fn translating_jmp_acc() {
        assert_eq!(Operation::to_op("jmp -42"), Jmp(-42));
    }

    #[test]
    fn freshly_initialized_computer() {
        let comp = Computer::parse_program("nop +0");
        let comp = Computer::init(&comp);
        assert_eq!(comp.instr_ptr(), 0);
        assert_eq!(comp.accumulated(), 0);
        assert_eq!(comp.halted(), false);
    }

    #[test]
    fn execute_nop_step() {
        let comp = Computer::parse_program("nop +0");
        let mut comp = Computer::init(&comp);
        comp.execute_step();
        assert_eq!(comp.instr_ptr(), 1);
        assert_eq!(comp.accumulated(), 0);
    }

    #[test]
    fn execute_acc_step() {
        let comp = Computer::parse_program("acc +4");
        let mut comp = Computer::init(&comp);
        comp.execute_step();
        assert_eq!(comp.instr_ptr(), 1);
        assert_eq!(comp.accumulated(), 4);
        assert_eq!(comp.ended_without_loop(), None);
    }

    #[test]
    fn execute_multiple_steps() {
        let comp = Computer::parse_program("acc +4\nnop +123\nacc -3\nnop +0\nnop +12\nacc +1");
        let mut comp = Computer::init(&comp);
        for _ in 0..6 {
            comp.execute_step();
        }

        assert_eq!(comp.accumulated(), 2);
    }

    #[test]
    fn add_jmp_steps() {
        let comp = Computer::parse_program("jmp +2\nacc +1\nnop +0");
        let mut comp = Computer::init(&comp);
        comp.execute_step();
        assert_eq!(comp.instr_ptr(), 2);
        assert_eq!(comp.accumulated(), 0);
    }

    #[test]
    fn execute_until_loop_ended() {
        let comp = Computer::parse_program("jmp +2\nacc +1\nnop +0\njmp -2");
        let mut comp = Computer::init(&comp);
        comp.execute_until_end();
        assert_eq!(comp.halted(), true);
        assert_eq!(comp.accumulated(), 1);
        assert_eq!(comp.instr_ptr(), 2);
        assert_eq!(comp.ended_without_loop(), Some(false))
    }

    #[test]
    fn execute_until_end_for_non_loop_ends() {
        let comp = Computer::parse_program("jmp +2\nacc +1\nnop +0\nacc +5");
        let mut comp = Computer::init(&comp);
        comp.execute_until_end();
        assert_eq!(comp.instr_ptr(), 4);
        assert_eq!(comp.accumulated(), 5);
        assert_eq!(comp.ended_without_loop(), Some(true));
    }

    #[test]
    fn execute_test_example() {
        let comp = Computer::parse_program(
            "nop +0\n\
acc +1\n\
jmp +4\n\
acc +3\n\
jmp -3\n\
acc -99\n\
acc +1\n\
jmp -4\n\
acc +6",
        );
        let mut comp = Computer::init(&comp);
        comp.execute_until_end();
        assert_eq!(comp.accumulated(), 5);
    }
}
