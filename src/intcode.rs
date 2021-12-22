
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

    fn fetch_instruction(&self) -> Instruction {
        self.into()
    }

    fn resolve_paramter_address(
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
        let Instruction {
            opcode,
            pmode1,
            pmode2,
            pmode3
        } = self.fetch_instruction();

        let result = match opcode {
            OpCode::ADD => {
                let paddr1 = self.resolve_paramter_address(1, pmode1);
                let paddr2 = self.resolve_paramter_address(2, pmode2);
                let paddr3 = self.resolve_paramter_address(3, pmode3);
                self.memory[paddr3] = self.memory[paddr1] + self.memory[paddr2];
                None
            },
            OpCode::MUL => {
                let paddr1 = self.resolve_paramter_address(1, pmode1);
                let paddr2 = self.resolve_paramter_address(2, pmode2);
                let paddr3 = self.resolve_paramter_address(3, pmode3);
                self.memory[paddr3] = self.memory[paddr1] * self.memory[paddr2];
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
