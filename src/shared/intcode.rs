use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

#[derive(Clone)]
pub struct IntcodeProgram {
    prgm: Vec<i64>,
    instr_ptr: usize,
    relative_base_offset: i64,

    pub input: VecDeque<i64>,
    pub output: Vec<i64>,

    extended_memory: HashMap<usize, i64>,
}

#[derive(PartialEq)]
#[allow(non_camel_case_types)]
enum OpCodeType {
    HALT,
    ADD,           // 01
    MULT,          // 02
    READ,          // 03
    WRITE,         // 04
    JUMP_IF_TRUE,  // 05
    JUMP_IF_FALSE, // 06
    LESS_THAN,     // 07
    EQUALS,        // 08
    MOVE_REL_OFFS, // 09

    // Invalid / unknown op code
    ERR,
}

impl From<i64> for OpCodeType {
    fn from(value: i64) -> Self {
        // Last two digits:
        match value % 100 {
             1 => OpCodeType::ADD,
             2 => OpCodeType::MULT,
             3 => OpCodeType::READ,
             4 => OpCodeType::WRITE,
             5 => OpCodeType::JUMP_IF_TRUE,
             6 => OpCodeType::JUMP_IF_FALSE,
             7 => OpCodeType::LESS_THAN,
             8 => OpCodeType::EQUALS,
             9 => OpCodeType::MOVE_REL_OFFS,
            99 => OpCodeType::HALT,
             _ => OpCodeType::ERR,
        }
    }
}

#[derive(PartialEq)]
enum ParameterMode {
    Positional,
    Immediate,
    Relative,
}

impl From<i64> for ParameterMode {
    fn from(value: i64) -> Self {
        if value == 0 {
            ParameterMode::Positional
        } else if value == 1 {
            ParameterMode::Immediate
        } else {
            ParameterMode::Relative
        }
    }
}

struct Instruction {
    opcode: OpCodeType,
    arg1: Option<i64>,
    arg2: Option<i64>,
    arg3: Option<i64>,
}

impl Instruction {
    fn get_arg_count(&self) -> usize {
        let mut arg_count: usize = 0;
		if self.arg1.is_some() { arg_count += 1; }
		if self.arg2.is_some() { arg_count += 1; }
		if self.arg3.is_some() { arg_count += 1; }
        arg_count
    }
}

#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum InstructionResult {
    RUNNING,
    WAIT_FOR_INPUT,
    HALT,
    EOF,
    ERROR,
}

impl IntcodeProgram {
    // Creates a new intcode program
    pub fn new(code: &String, input: Option<VecDeque<i64>>) -> Self {
        Self {
            prgm: code
                .trim_end()
                .split(',')
                .map(|s| i64::from_str(s).unwrap())
                .collect(),
            instr_ptr: 0,
            relative_base_offset: 0,
            input: input.unwrap_or(VecDeque::new()),
            output: vec![],
            extended_memory: HashMap::new(),
        }
    }

    // Reads the current value inside the intcode program at the given position
    pub fn read(&self, index: usize) -> i64 {
        *self.prgm.get(index).unwrap_or_else(|| {
            // If out of bound from code, try extended memory:
            self.extended_memory.get(&index).unwrap_or(&0)
        })
    }

    // Writes the value to the intcode program at the given position
    pub fn write(&mut self, index: usize, value: i64) {
        if index < self.prgm.len() {
            self.prgm[index] = value;
        } else {
            // If out of bound from code, try extended memory:
            self.extended_memory.entry(index).insert_entry(value);
        }
    }

    // Runs the intcode program
    pub fn run(&mut self) -> InstructionResult {
        self.instr_ptr = 0;
        self.run_continue()
    }

    // Continues a paused intcode program
    pub fn run_continue(&mut self) -> InstructionResult {
        while let Some(instr) = self.next_instruction() {
            let instr_res: InstructionResult = self.apply_instruction(&instr);

            if instr_res != InstructionResult::RUNNING {
                if instr_res == InstructionResult::WAIT_FOR_INPUT {
                    // Reset instruction pointer so current instruction can be run again when program is continued:
                    self.instr_ptr -= instr.get_arg_count() + 1;
                }
                return instr_res;
            }
        }

        InstructionResult::EOF
    }

    // Retrieve the value at the current instruction pointer and move the pointer forward:
    fn next_value(&mut self, mode: ParameterMode) -> Option<i64> {
        let val = self.read(self.instr_ptr);
        self.instr_ptr += 1;

        if mode == ParameterMode::Immediate {
            Some(val)
        } else if mode == ParameterMode::Positional {
            Some(self.read(usize::try_from(val).unwrap()))
        } else if mode == ParameterMode::Relative {
            Some(self.read(usize::try_from(val + self.relative_base_offset).unwrap()))
        } else {
            None
        }
    }

    // Retrieves an argument for the given opcode, based on the parameter mode
    fn get_argument(&mut self, opcode: i64, arg_no: u32) -> Option<i64> {
        let opcode_type = OpCodeType::from(opcode);
        let param_mode = ParameterMode::from((opcode / (100 * 10i64.pow(arg_no - 1))) % 10);

        match opcode_type {
            OpCodeType::ADD | OpCodeType::MULT | OpCodeType::LESS_THAN | OpCodeType::EQUALS if arg_no <= 2 => {
                // IN parameter for binary operations:
                self.next_value(param_mode)
            }
            OpCodeType::ADD | OpCodeType::MULT | OpCodeType::LESS_THAN | OpCodeType::EQUALS if arg_no == 3 => {
                // adress given is always read as immediate value, but the resulting position depends on parameter mode
                let address = self.next_value(ParameterMode::Immediate);
                if let Some(addr_val) = address && param_mode == ParameterMode::Relative {
                    Some(addr_val + self.relative_base_offset)
                } else {
                    address
                }
            }
            OpCodeType::READ if arg_no == 1 => {
                // adress given is always read as immediate value, but the resulting position depends on parameter mode
                let address = self.next_value(ParameterMode::Immediate);
                if let Some(addr_val) = address && param_mode == ParameterMode::Relative {
                    Some(addr_val + self.relative_base_offset)
                } else {
                    address
                }
            }
            OpCodeType::WRITE if arg_no == 1 => {
				self.next_value(param_mode)
			}
            OpCodeType::JUMP_IF_TRUE | OpCodeType::JUMP_IF_FALSE if arg_no <= 2 => {
                self.next_value(param_mode)
            }
			OpCodeType::MOVE_REL_OFFS if arg_no == 1 => {
                self.next_value(param_mode)
            }
            _ => None,
        }
    }

    fn next_instruction(&mut self) -> Option<Instruction> {
        // Opcode is always read in immediate mode:
        let opcode = self.next_value(ParameterMode::Immediate);

        if let Some(opcode) = opcode {
            let arg1 = self.get_argument(opcode, 1);
            let arg2 = self.get_argument(opcode, 2);
            let arg3 = self.get_argument(opcode, 3);

            Some(Instruction {
                opcode: OpCodeType::from(opcode),
                arg1,
                arg2,
                arg3,
            })
        } else {
            None
        }
    }

    // Applies the given instruction to the intcode program
    fn apply_instruction(&mut self, instr: &Instruction) -> InstructionResult {
        match instr.opcode {
            OpCodeType::HALT => InstructionResult::HALT,
            OpCodeType::ADD => {
                self.write(
                    usize::try_from(instr.arg3.unwrap()).unwrap(),
                    instr.arg1.unwrap() + instr.arg2.unwrap(),
                );
                InstructionResult::RUNNING
            }
            OpCodeType::MULT => {
                self.write(
                    usize::try_from(instr.arg3.unwrap()).unwrap(),
                    instr.arg1.unwrap() * instr.arg2.unwrap(),
                );
                InstructionResult::RUNNING
            }
            OpCodeType::READ => {
                let input_val = self.input.pop_front();
                if let Some(val) = input_val {
                    self.write(usize::try_from(instr.arg1.unwrap()).unwrap(), val);
                    InstructionResult::RUNNING
                } else {
                    InstructionResult::WAIT_FOR_INPUT
                }
            }
            OpCodeType::WRITE => {
                self.output.push(instr.arg1.unwrap());
                InstructionResult::RUNNING
            }
            OpCodeType::JUMP_IF_TRUE => {
                if instr.arg1.unwrap() != 0 {
                    self.instr_ptr = usize::try_from(instr.arg2.unwrap()).unwrap();
                }
                InstructionResult::RUNNING
            }
            OpCodeType::JUMP_IF_FALSE => {
                if instr.arg1.unwrap() == 0 {
                    self.instr_ptr = usize::try_from(instr.arg2.unwrap()).unwrap();
                }
                InstructionResult::RUNNING
            }
            OpCodeType::LESS_THAN => {
				let output = if instr.arg1.unwrap() < instr.arg2.unwrap() { 1 } else { 0 };
                self.write(usize::try_from(instr.arg3.unwrap()).unwrap(), output);
                InstructionResult::RUNNING
            }
            OpCodeType::EQUALS => {
				let output = if instr.arg1.unwrap() == instr.arg2.unwrap() { 1 } else { 0 };
                self.write(usize::try_from(instr.arg3.unwrap()).unwrap(), output);
                InstructionResult::RUNNING
            }
            OpCodeType::MOVE_REL_OFFS => {
                self.relative_base_offset += instr.arg1.unwrap();
                InstructionResult::RUNNING
            }
            _ => InstructionResult::ERROR,
        }
    }
}

impl From<Vec<i64>> for IntcodeProgram {
    fn from(value: Vec<i64>) -> Self {
        Self {
            prgm: value,
            instr_ptr: 0,
            relative_base_offset: 0,
            input: VecDeque::new(),
            output: vec![],
            extended_memory: HashMap::new(),
        }
    }
}
