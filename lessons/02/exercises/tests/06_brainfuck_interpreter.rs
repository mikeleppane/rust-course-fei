//! Run this file with `cargo test --test 06_brainfuck_interpreter`.

// TODO (bonus): Create an interpreter for the [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) language.
// The Brainfuck program will be parsed out of a string and represented as a struct.
//
// Handle both parsing and execution errors using enums representing error conditions,
// see tests for details.
// A parsing error can be either an unknown instruction or an unpaired loop instruction.
// An execution error can be either that the program tries to read input, but there is no more
// input available, or when the program executes more than 10000 instructions (which probably
// signals an infinite loop).
//
// Hint: Put `#[derive(Debug, Eq, PartialEq)]` on top of `ParseError`, `ExecuteError` and `Program`
// (and any other custom types nested inside them) so that asserts in tests work.

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    UnknownInstruction { location: usize, instruction: char },
    UnmatchedLoop { location: usize },
}

#[derive(Debug, Eq, PartialEq)]
pub enum ExecuteError {
    NoInputLeft,
    InfiniteLoop,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    /// Executes the Brainfuck program with the given input and memory.
    ///
    /// # Errors
    ///
    /// Returns `ExecuteError::NoInputLeft` if the program tries to read input but there is no more input available.
    /// Returns `ExecuteError::InfiniteLoop` if the program executes more than 10000 instructions.
    pub fn execute(&self, input: Vec<u8>, memory: Vec<u8>) -> Result<String, ExecuteError> {
        const MAX_INSTRUCTIONS: usize = 10000;
        let mut memory = memory;
        let mut input = input.into_iter();
        let mut output = String::new();
        let mut instruction_pointer = 0;
        let mut memory_pointer = 0;
        let mut instruction_counter = 0;
        while instruction_pointer < self.instructions.len() {
            instruction_counter += 1;
            if instruction_counter > MAX_INSTRUCTIONS {
                return Err(ExecuteError::InfiniteLoop);
            }
            match &self.instructions[instruction_pointer] {
                Instruction::MoveRight => {
                    memory_pointer += 1;
                    if memory_pointer >= memory.len() {
                        memory.push(0);
                    }
                }
                Instruction::MoveLeft => {
                    if memory_pointer == 0 {
                        memory.insert(0, 0);
                    } else {
                        memory_pointer -= 1;
                    }
                }
                Instruction::Increment => {
                    memory[memory_pointer] = memory[memory_pointer].wrapping_add(1);
                }
                Instruction::Decrement => {
                    memory[memory_pointer] = memory[memory_pointer].wrapping_sub(1);
                }
                Instruction::Output => {
                    output.push(memory[memory_pointer] as char);
                }
                Instruction::Input => {
                    if let Some(byte) = input.next() {
                        memory[memory_pointer] = byte;
                    } else {
                        return Err(ExecuteError::NoInputLeft);
                    }
                }
                Instruction::Loop { start, end } => {
                    if memory[memory_pointer] == 0 {
                        instruction_pointer = *end;
                    } else {
                        instruction_pointer = *start;
                        continue;
                    }
                }
            }
            instruction_pointer += 1;
        }
        Ok(output)
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    Loop { start: usize, end: usize },
}

fn parse_program(program_text: &str) -> Result<Program, ParseError> {
    let mut program = Program::new();
    let mut loop_stack = Vec::new();
    for (location, instruction) in program_text.chars().enumerate() {
        match instruction {
            '>' => program.instructions.push(Instruction::MoveRight),
            '<' => program.instructions.push(Instruction::MoveLeft),
            '+' => program.instructions.push(Instruction::Increment),
            '-' => program.instructions.push(Instruction::Decrement),
            '.' => program.instructions.push(Instruction::Output),
            ',' => program.instructions.push(Instruction::Input),
            '[' => loop_stack.push(program.instructions.len()),
            ']' => {
                if let Some(start) = loop_stack.pop() {
                    let current_pos = program.instructions.len();
                    program.instructions.push(Instruction::Loop {
                        start,
                        end: current_pos,
                    });
                    if let Instruction::Loop { end, .. } = &mut program.instructions[start] {
                        *end = current_pos;
                    }
                } else {
                    return Err(ParseError::UnmatchedLoop { location });
                }
            }
            _ => {
                return Err(ParseError::UnknownInstruction {
                    location,
                    instruction,
                })
            }
        }
    }
    if loop_stack.is_empty() {
        Ok(program)
    } else {
        Err(ParseError::UnmatchedLoop {
            location: loop_stack.pop().unwrap() + 1,
        })
    }
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::{parse_program, ExecuteError, ParseError};

    #[test]
    fn parse_empty() {
        check_output("", "", "");
    }

    #[test]
    fn parse_unknown_instruction() {
        assert!(matches!(
            parse_program(">p"),
            Err(ParseError::UnknownInstruction {
                location: 1,
                instruction: 'p'
            })
        ));
    }

    #[test]
    fn parse_unmatched_loop_start() {
        assert_eq!(
            parse_program(">++[+>][++>"),
            Err(ParseError::UnmatchedLoop { location: 7 })
        );
    }

    #[test]
    fn parse_unmatched_loop_end() {
        assert_eq!(
            parse_program(">++[+>][++>]+]"),
            Err(ParseError::UnmatchedLoop { location: 13 })
        );
    }

    #[test]
    fn missing_input() {
        let program = parse_program(",").unwrap();
        let result = program.execute(vec![], vec![0; 30000]);
        assert_eq!(result, Err(ExecuteError::NoInputLeft));
    }

    #[test]
    fn infinite_loop() {
        let program = parse_program("+[]").unwrap();
        let result = program.execute(vec![], vec![0; 30000]);
        assert_eq!(result, Err(ExecuteError::InfiniteLoop));
    }

    #[test]
    fn copy_input() {
        check_output(",.>,.>,.>,.>,.", "hello", "hello");
    }

    #[test]
    fn output_exclamation_mark() {
        check_output("+++++++++++++++++++++++++++++++++.", "", "!");
    }

    #[test]
    fn three_exclamation_marks() {
        check_output(">+++++++++++++++++++++++++++++++++<+++[>.<-]", "", "!!!");
    }

    #[test]
    fn hello_world() {
        check_output("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.", "", "Hello World!\n");
    }

    fn check_output(program_text: &str, input: &str, expected_output: &str) {
        let program = parse_program(program_text);
        match program {
            Ok(program) => {
                let result = program
                    .execute(input.to_string().into_bytes(), vec![0; 30000])
                    .unwrap_or_else(|_| panic!("Cannot execute program {program_text}"));
                assert_eq!(result, expected_output);
            }
            Err(error) => {
                panic!("Error occurred while parsing program {program_text}: {error:?}");
            }
        }
    }
}
