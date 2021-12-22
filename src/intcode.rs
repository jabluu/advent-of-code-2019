
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
        let instr_ptr = self.instruction_pointer;

        let opcode = self.memory[instr_ptr].into();

        let result = match opcode {
            OpCode::ADD => {
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

                None
            },
            OpCode::MUL => {
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
