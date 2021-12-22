use std::{io::{self, Write}, result};

#[derive(Clone)]
enum Signal {
    HALT,
}

#[derive(Debug)]
enum OpCode {
    ADD, // 1
    MUL, // 2
    INPUT, // 3
    OUTPUT, // 4
    JIT, // 5
    JIF, // 6
    LT, // 7
    EQ, // 8
    HALT, // 99
}

impl From<i32> for OpCode {
    fn from(id: i32) -> Self {
        match id {
            1 => Self::ADD,
            2 => Self::MUL,
            3 => Self::INPUT,
            4 => Self::OUTPUT,
            5 => Self::JIT,
            6 => Self::JIF,
            7 => Self::LT,
            8 => Self::EQ,
            99 => Self::HALT,
            _ => panic!("unrecognized opcode id: {}", id),
        }
    }
}

impl OpCode {
    fn size(&self) -> usize {
        match self {
            Self::ADD => 4,
            Self::MUL => 4,
            Self::INPUT => 2,
            Self::OUTPUT => 2,
            Self::JIT => 3,
            Self::JIF => 3,
            Self::LT => 4,
            Self::EQ => 4,
            Self::HALT => 1,
        }
    }
}

enum ParameterMode {
    POSITION,
    IMMEDIATE,
}

impl From<i32> for ParameterMode {
    fn from(id: i32) -> Self {
        match id {
            0 => Self::POSITION,
            1 => Self::IMMEDIATE,
            _ => panic!("unrecognized parameter mode id: {}", id),
        }
    }
}

struct Instruction {
    opcode: OpCode,
    pmode1: ParameterMode,
    pmode2: ParameterMode,
    pmode3: ParameterMode,
}

impl From<&Computer> for Instruction {
    fn from(computer: &Computer) -> Self {
        let address = computer.instruction_pointer;
        let remainder = computer.memory[address];

        fn f(x: i32, n: u32) -> (i32, i32) {
            let d = 10i32.pow(n);
            (x % d, x / d)
        }

        let (opcode_id, remainder) = f(remainder, 2);
        let (pmode1_id, remainder) = f(remainder, 1);
        let (pmode2_id, remainder) = f(remainder, 1);
        let (pmode3_id, remainder) = f(remainder, 1);

        Self {
            opcode: opcode_id.into(),
            pmode1: pmode1_id.into(),
            pmode2: pmode2_id.into(),
            pmode3: pmode3_id.into(),
        }
    }
}

#[derive(Clone)]
pub struct Computer {
    pub memory: Vec<i32>,
    instruction_pointer: usize,
    pub output: i32,
}

impl Computer {
    pub fn new(program: &[i32]) -> Self {
        Self {
            memory: program.to_vec(),
            instruction_pointer: 0,
            output: 0,
        }
    }

    pub fn with_inputs(&mut self, noun: i32, verb: i32) -> &mut Self {
        self.memory[1] = noun;
        self.memory[2] = verb;
        self
    }

    pub fn execute_program(&mut self) -> &Self {
        loop {
            let signal = self.execute_instruction();

            match &signal {
                Some(Signal::HALT) => {
                    break;
                },
                None => (),
            }
        }

        self
    }

    fn fetch_instruction(&self) -> Instruction {
        self.into()
    }

    fn resolve_parameter_address(
        &self,
        offset: usize,
        pmode: ParameterMode
    ) -> usize {
        let address = self.instruction_pointer;

        match pmode {
            ParameterMode::POSITION => self.memory[address + offset] as usize,
            ParameterMode::IMMEDIATE => address + offset,
        }
    }

    fn execute_instruction(&mut self) -> Option<Signal> {
        let ip = self.instruction_pointer;

        let Instruction {
            opcode,
            pmode1,
            pmode2,
            pmode3
        } = self.fetch_instruction();

        let (ip, signal) = match opcode {
            OpCode::ADD => {
                let paddr1 = self.resolve_parameter_address(1, pmode1);
                let paddr2 = self.resolve_parameter_address(2, pmode2);
                let paddr3 = self.resolve_parameter_address(3, pmode3);
                self.memory[paddr3] = self.memory[paddr1] + self.memory[paddr2];
                (ip + opcode.size(), None)
            },
            OpCode::MUL => {
                let paddr1 = self.resolve_parameter_address(1, pmode1);
                let paddr2 = self.resolve_parameter_address(2, pmode2);
                let paddr3 = self.resolve_parameter_address(3, pmode3);
                self.memory[paddr3] = self.memory[paddr1] * self.memory[paddr2];
                (ip + opcode.size(), None)
            },
            OpCode::INPUT => {
                print!("Enter a System ID: ");
                std::io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.strip_suffix("\n").unwrap();
                let value = input.parse::<i32>().unwrap();

                let paddr1 = self.resolve_parameter_address(1, pmode1);
                self.memory[paddr1] = value;

                (ip + opcode.size(), None)
            },
            OpCode::OUTPUT => {
                let paddr1 = self.resolve_parameter_address(1, pmode1);
                self.output = self.memory[paddr1];
                (ip + opcode.size(), None)
            },
            OpCode::JIT => {
                let paddr1 = self.resolve_parameter_address(1, pmode1);
                let paddr2 = self.resolve_parameter_address(2, pmode2);

                let ip = match self.memory[paddr1] {
                    0 => ip + opcode.size(),
                    _ => self.memory[paddr2] as usize,
                };

                (ip, None)
            },
            OpCode::JIF => {
                let paddr1 = self.resolve_parameter_address(1, pmode1);
                let paddr2 = self.resolve_parameter_address(2, pmode2);

                let ip = match self.memory[paddr1] {
                    0 => self.memory[paddr2] as usize,
                    _ => ip + opcode.size(),
                };

                (ip, None)
            },
            OpCode::LT => {
                let paddr1 = self.resolve_parameter_address(1, pmode1);
                let paddr2 = self.resolve_parameter_address(2, pmode2);
                let paddr3 = self.resolve_parameter_address(3, pmode3);

                let result = self.memory[paddr1] < self.memory[paddr2];

                self.memory[paddr3] = match result {
                    true => 1,
                    false => 0,
                };

                (ip + opcode.size(), None)
            },
            OpCode::EQ => {
                let paddr1 = self.resolve_parameter_address(1, pmode1);
                let paddr2 = self.resolve_parameter_address(2, pmode2);
                let paddr3 = self.resolve_parameter_address(3, pmode3);

                let result = self.memory[paddr1] == self.memory[paddr2];

                self.memory[paddr3] = match result {
                    true => 1,
                    false => 0,
                };

                (ip + opcode.size(), None)
            },
            OpCode::HALT => {
                (ip + opcode.size(), Some(Signal::HALT))
            },
        };

        self.instruction_pointer = ip;

        signal
    }
}
