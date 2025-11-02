use std::{collections::VecDeque, str::FromStr};

#[derive(Clone)]
pub struct IntcodeProgram {
	prgm: Vec<i64>,
	instr_ptr: usize,

	pub input: VecDeque<i64>,
	pub output: Vec<i64>
}

#[derive(PartialEq)]
enum OpCodeType {
	HALT,
	ADD,			// 01
	MULT,			// 02
	READ,			// 03
	WRITE,			// 04
	JUMP_IF_TRUE,	// 05
	JUMP_IF_FALSE,	// 06
	LESS_THAN,		// 07
	EQUALS,			// 08

	// Invalid / unknown op code
	ERR
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
			99 => OpCodeType::HALT,
			 _ => OpCodeType::ERR
		}
	}
}

#[derive(PartialEq)]
enum ParameterMode {
	Positional,
	Immediate
}

impl From<i64> for ParameterMode {
	fn from(value: i64) -> Self {
		if value == 0 {
			ParameterMode::Positional
		} else {
			ParameterMode::Immediate
		}
	}
}

struct Instruction {
	opcode : OpCodeType,
	arg1 : Option<i64>,
	arg2 : Option<i64>,
	arg3 : Option<i64>
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
			input: input.unwrap_or(VecDeque::new()),
			output: vec![],
		}
	}

	// Reads the current value inside the intcode program at the given position
	pub fn read(&self, index: usize) -> u64 {
		u64::try_from(self.prgm[index]).unwrap_or(0)
	}

	// Writes the value to the intcode program at the given position
	pub fn write(&mut self, index: usize, value: i64) {
		self.prgm[index] = value;
	}

	// Runs the intcode program
	pub fn run(&mut self) {
		self.instr_ptr = 0;
		
		while let Some(instr) = self.next_instruction() {
			if instr.opcode == OpCodeType::HALT {
				break;
			}

			self.apply_instruction(&instr);
		}
	}

	// Retrieve the value at the current instruction pointer and move the pointer forward:
	fn next_value(&mut self, mode: ParameterMode) -> Option<i64> {
		let val = self.prgm[self.instr_ptr];
		self.instr_ptr += 1;

		if mode == ParameterMode::Immediate {
			Some(val)
		} else if mode == ParameterMode::Positional {
			Some(self.prgm[usize::try_from(val).unwrap()])
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
			},
			OpCodeType::ADD | OpCodeType::MULT | OpCodeType::LESS_THAN | OpCodeType::EQUALS if arg_no == 3 => {
				// OUT parameter for binary operations: adress given is always read as immediate value
				self.next_value(ParameterMode::Immediate)
			},
			OpCodeType::READ if arg_no == 1 => {
				// OUT parameter for READ: adress given is always read as immediate value
				self.next_value(ParameterMode::Immediate)
			},
			OpCodeType::WRITE if arg_no == 1 => {
				self.next_value(param_mode)
			},
			OpCodeType::JUMP_IF_TRUE | OpCodeType::JUMP_IF_FALSE if arg_no <= 2 => {
				self.next_value(param_mode)
			},
			_ => None
		}
	}

	fn next_instruction(&mut self) -> Option<Instruction> {
		// Opcode is always read in immediate mode:
		let opcode = self.next_value(ParameterMode::Immediate);

		if let Some(opcode) = opcode {
			let arg1 = self.get_argument(opcode, 1);
			let arg2 = self.get_argument(opcode, 2);
			let arg3 = self.get_argument(opcode, 3);

			Some(Instruction { opcode: OpCodeType::from(opcode), arg1, arg2, arg3 })
		} else {
			None
		}
	}

	// Applies the given instruction to the intcode program
	fn apply_instruction(&mut self, instr: &Instruction) {
		match instr.opcode {
			OpCodeType::ADD => self.prgm[usize::try_from(instr.arg3.unwrap()).unwrap()] = instr.arg1.unwrap() + instr.arg2.unwrap(),
			OpCodeType::MULT => self.prgm[usize::try_from(instr.arg3.unwrap()).unwrap()] = instr.arg1.unwrap() * instr.arg2.unwrap(),
			OpCodeType::READ => {
				let input_val = self.input.pop_front();
				if let Some(val) = input_val {
					self.prgm[usize::try_from(instr.arg1.unwrap()).unwrap()] = val;
				}
			},
			OpCodeType::WRITE => {
				self.output.push(instr.arg1.unwrap());
			},
			OpCodeType::JUMP_IF_TRUE if instr.arg1.unwrap() != 0 => {
				self.instr_ptr = usize::try_from(instr.arg2.unwrap()).unwrap();
			},
			OpCodeType::JUMP_IF_FALSE if instr.arg1.unwrap() == 0 => {
				self.instr_ptr = usize::try_from(instr.arg2.unwrap()).unwrap();
			},
			OpCodeType::LESS_THAN => {
				let output = if instr.arg1.unwrap() < instr.arg2.unwrap() { 1 } else { 0 };
				self.prgm[usize::try_from(instr.arg3.unwrap()).unwrap()] = output;
			},
			OpCodeType::EQUALS => {
				let output = if instr.arg1.unwrap() == instr.arg2.unwrap() { 1 } else { 0 };
				self.prgm[usize::try_from(instr.arg3.unwrap()).unwrap()] = output;
			},
			_ => {}
		};
	}

	
}

impl From<Vec<i64>> for IntcodeProgram {
	fn from(value: Vec<i64>) -> Self {
		Self { prgm: value, instr_ptr: 0, input: VecDeque::new(), output: vec![] }
	}
}