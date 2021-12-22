
#[derive(Clone)]
enum Signal {
    HALT,
}

enum OpCode {
    ADD,
    MUL,
    HALT,
}

impl From<i32> for OpCode {
    fn from(id: i32) -> Self {
        match id {
            1 => Self::ADD,
            2 => Self::MUL,
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
    address: usize,
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

        let (opcode, remainder) = f(remainder, 2);
        let (pmode1, remainder) = f(remainder, 1);
        let (pmode2, remainder) = f(remainder, 1);
        let (pmode3, remainder) = f(remainder, 1);

        Self {
            address,
            opcode: opcode.into(),
            pmode1: pmode1.into(),
            pmode2: pmode2.into(),
            pmode3: pmode3.into(),
        }
    }
}

#[derive(Clone)]
pub struct Computer {
    pub memory: Vec<i32>,
    instruction_pointer: usize,
}

impl Computer {
    pub fn new(program: &[i32]) -> Self {
        Self {
            memory: program.to_vec(),
            instruction_pointer: 0,
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

    fn execute_instruction(&mut self) -> Option<Signal> {
        let Instruction {
            address,
            opcode,
            pmode1,
            pmode2,
            pmode3
        } = Instruction::from(&*self);

        let result = match opcode {
            OpCode::ADD => {
                let param_addr = (
                    self.memory[address+1] as usize,
                    self.memory[address+2] as usize,
                );

                let result_addr = self.memory[address+3] as usize;

                let param = (
                    self.memory[param_addr.0],
                    self.memory[param_addr.1],
                );

                let result = param.0 + param.1;
                self.memory[result_addr] = result;

                None
            },
            OpCode::MUL => {
                let param_addr = (
                    self.memory[address+1] as usize,
                    self.memory[address+2] as usize,
                );

                let result_addr = self.memory[address+3] as usize;

                let param = (
                    self.memory[param_addr.0],
                    self.memory[param_addr.1],
                );

                let result = param.0 * param.1;
                self.memory[result_addr] = result;

                None
            },
            OpCode::HALT => {
                Some(Signal::HALT)
            },
        };

        self.instruction_pointer += opcode.size();

        result
    }
}
