use std::str::FromStr;

#[derive(Clone)]
pub struct IntcodeProgram {
	prgm: Vec<u64>,
	instr_ptr: usize,

	input: Vec<u64>,
	output: Vec<u64>
}

#[derive(PartialEq)]
enum OpCodeType {
	HALT,
	ADD,
	MULT,

	// Invalid / unknown op code
	ERR
}

#[derive(PartialEq)]
enum ParameterMode {
	Positional,
	Immediate
}

struct Instruction {
	opcode : OpCodeType,
	arg1 : Option<u64>,
	arg2 : Option<u64>,
	arg3 : Option<u64>
}

impl IntcodeProgram {
	// Creates a new intcode program
	pub fn new(code: &String, input: Option<Vec<u64>>) -> Self {
		Self {
			prgm: code
				.trim_end()
				.split(',')
				.map(|s| u64::from_str(s).unwrap())
				.collect(),
			instr_ptr: 0,
			input: input.unwrap_or(vec![]),
			output: vec![],
		}
	}

	// Reads the current value inside the intcode program at the given position
	pub fn read(&self, index: usize) -> u64 {
		self.prgm[index]
	}

	// Writes the value to the intcode program at the given position
	pub fn write(&mut self, index: usize, value: u64) {
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
	fn next_value(&mut self, mode: ParameterMode) -> Option<u64> {
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
	fn get_argument(&mut self, opcode: u64, arg_no: u32) -> Option<u64> {
		let opcode_type = OpCodeType::from(opcode);
		let param_mode = ParameterMode::from((opcode / (100 * 10u64.pow(arg_no - 1))) % 10);

		match opcode_type {
			OpCodeType::ADD | OpCodeType::MULT if arg_no <= 2 => {
				// IN parameter for ADD and MULT
				self.next_value(param_mode)
			},
			OpCodeType::ADD | OpCodeType::MULT if arg_no == 3 => {
				// OUT parameter for ADD and MULT: adress given is always read as immediate value
				self.next_value(ParameterMode::Immediate)
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
			_ => {}
		};
	}

	
}

impl From<Vec<u64>> for IntcodeProgram {
	fn from(value: Vec<u64>) -> Self {
		Self { prgm: value, instr_ptr: 0, input: vec![], output: vec![] }
	}
}

impl From<u64> for ParameterMode {
	fn from(value: u64) -> Self {
		if value == 0 {
			ParameterMode::Positional
		} else {
			ParameterMode::Immediate
		}
	}
}

impl From<u64> for OpCodeType {
	fn from(value: u64) -> Self {
		// Last two digits:
		match value % 100 {
			 1 => OpCodeType::ADD,
			 2 => OpCodeType::MULT,
			99 => OpCodeType::HALT,
			 _ => OpCodeType::ERR
		}
	}
}