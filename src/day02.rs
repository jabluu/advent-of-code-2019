// https://adventofcode.com/2019/day/2

pub fn part1() -> i32 {
    #[derive(Clone)]
    enum State {
        EXEC,
        HALT,
    }

    #[derive(Clone)]
    struct Computer {
        program: Vec<i32>,
        counter: usize,
        state: State,
    }

    impl Computer {
        fn new(program: &[i32]) -> Self {
            Self {
                program: program.to_vec(),
                counter: 0,
                state: State::EXEC,
            }
        }

        fn run(&mut self) -> &Self {
            loop {
                match self.state {
                    State::EXEC => {
                        self.run_cycle();
                    },
                    State::HALT => {
                        break;
                    }
                }
            }

            self
        }

        fn run_cycle(&mut self) {
            let opcode = self.program[self.counter];

            match opcode {
                1 => {
                    let src = (
                        self.program[self.counter+1] as usize,
                        self.program[self.counter+2] as usize,
                    );

                    let dst = self.program[self.counter+3] as usize;

                    let arg = (
                        self.program[src.0],
                        self.program[src.1],
                    );

                    self.program[dst] = arg.0 + arg.1;
                    self.counter += 4;
                },
                2 => {
                    let src = (
                        self.program[self.counter+1] as usize,
                        self.program[self.counter+2] as usize,
                    );

                    let dst = self.program[self.counter+3] as usize;

                    let arg = (
                        self.program[src.0],
                        self.program[src.1],
                    );

                    self.program[dst] = arg.0 * arg.1;
                    self.counter += 4;
                },
                99 => {
                    self.state = State::HALT;
                    self.counter += 1;
                },
                _ => panic!("unrecognized opcode: {}", opcode),
            }
        }
    }

    assert_eq!(
        [2,0,0,0,99].to_vec(),
        Computer::new(&[1,0,0,0,99]).run().program
    );

    assert_eq!(
        [2,3,0,6,99].to_vec(),
        Computer::new(&[2,3,0,3,99]).run().program
    );

    assert_eq!(
        [2,4,4,5,99,9801].to_vec(),
        Computer::new(&[2,4,4,5,99,0]).run().program
    );

    assert_eq!(
        [30,1,1,4,2,5,6,0,99].to_vec(),
        Computer::new(&[1,1,1,4,99,5,6,0,99]).run().program
    );

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let handle = File::open("input/day02/input.txt").unwrap();
    let buffer = BufReader::new(handle);

    let computers = buffer.lines()
        .map(|line| {
            line.unwrap().split(',').map(|item| {
                item.parse::<i32>().unwrap()
            }).collect::<Vec<i32>>()
        })
        .map(|program| {
            Computer::new(&program)
        })
        .collect::<Vec<Computer>>();

    let mut computer = computers[0].clone();

    computer.program[1] = 12;
    computer.program[2] = 2;

    computer.run().program[0]
}

pub fn part2() -> i32 {
    #[derive(Clone)]
    enum State {
        EXEC,
        HALT,
    }

    #[derive(Clone)]
    struct Computer {
        memory: Vec<i32>,
        instruction_pointer: usize,
        state: State,
    }

    impl Computer {
        fn new(program: &[i32]) -> Self {
            Self {
                memory: program.to_vec(),
                instruction_pointer: 0,
                state: State::EXEC,
            }
        }

        fn with_inputs(&mut self, noun: i32, verb: i32) -> &mut Self {
            self.memory[1] = noun;
            self.memory[2] = verb;
            self
        }

        fn execute_program(&mut self) -> &Self {
            loop {
                match self.state {
                    State::EXEC => {
                        self.execute_instruction();
                    },
                    State::HALT => {
                        break;
                    }
                }
            }

            self
        }

        fn execute_instruction(&mut self) {
            let instr_ptr = self.instruction_pointer;

            let opcode = self.memory[instr_ptr];

            match opcode {
                1 => {
                    let param_addr = (
                        self.memory[instr_ptr+1] as usize,
                        self.memory[instr_ptr+2] as usize,
                    );

                    let result_addr = self.memory[instr_ptr+3] as usize;

                    let param = (
                        self.memory[param_addr.0],
                        self.memory[param_addr.1],
                    );

                    let result = param.0 + param.1;
                    self.memory[result_addr] = result;

                    self.instruction_pointer += 4;
                },
                2 => {
                    let param_addr = (
                        self.memory[instr_ptr+1] as usize,
                        self.memory[instr_ptr+2] as usize,
                    );

                    let result_addr = self.memory[instr_ptr+3] as usize;

                    let param = (
                        self.memory[param_addr.0],
                        self.memory[param_addr.1],
                    );

                    let result = param.0 * param.1;
                    self.memory[result_addr] = result;

                    self.instruction_pointer += 4;
                },
                99 => {
                    self.state = State::HALT;
                    self.instruction_pointer += 1;
                },
                _ => panic!("unrecognized opcode: {}", opcode),
            }
        }
    }

    assert_eq!(
        [2,0,0,0,99].to_vec(),
        Computer::new(&[1,0,0,0,99]).execute_program().memory
    );

    assert_eq!(
        [2,3,0,6,99].to_vec(),
        Computer::new(&[2,3,0,3,99]).execute_program().memory
    );

    assert_eq!(
        [2,4,4,5,99,9801].to_vec(),
        Computer::new(&[2,4,4,5,99,0]).execute_program().memory
    );

    assert_eq!(
        [30,1,1,4,2,5,6,0,99].to_vec(),
        Computer::new(&[1,1,1,4,99,5,6,0,99]).execute_program().memory
    );

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let handle = File::open("input/day02/input.txt").unwrap();
    let buffer = BufReader::new(handle);

    let computers = buffer.lines()
        .map(|line| {
            line.unwrap().split(',').map(|item| {
                item.parse::<i32>().unwrap()
            }).collect::<Vec<i32>>()
        })
        .map(|program| {
            Computer::new(&program)
        })
        .collect::<Vec<Computer>>();

    let computer = computers[0].clone();

    for noun in 0..99 {
        for verb in 0..99 {
            let result = computer.clone()
                .with_inputs(noun, verb)
                .execute_program()
                .memory[0];

            if result == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    0
}