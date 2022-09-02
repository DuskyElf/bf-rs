use std::{env, process::exit, fs::File, io::Read};

#[derive(Clone)]
enum OpCode {
    Increment,
    Decrement,
    IncrementPointer,
    DecrementPointer,
    Read,
    Write,
    LoopBegin,
    LoopEnd,
}

enum Instruction {
    Increment,
    Decrement,
    IncrementPointer,
    DecrementPointer,
    Read,
    Write,
    Loop(Vec<Instruction>)
}

fn main() {
    let file_path = ui();
    let source = load_porgram(file_path);

    // Processing the Source
    let op_codes = lex(source);
    let instructions = parse(op_codes);

    // Virtual Environment
    let mut memory = vec![0u8; 1024];
    let mut head = 512;

    run(&instructions, &mut memory, &mut head);
}

fn ui() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: bf-rs <file.bf>");
        exit(400);
    }

    args[1].clone()
}

fn load_porgram(file_path: String) -> String {
    let mut file = match File::open(file_path) {
        Ok(t) => t,
        Err(e) => {
            println!("Error: {}", e);
            exit(404);
        }
    };
    let mut source = String::new();
    file.read_to_string(&mut source).expect("Failed to read the file.");
    source
}

fn lex(source: String) -> Vec<OpCode> {
    let mut op_codes = Vec::new();

    for symbol in source.chars() {
        match symbol {
            '+' => op_codes.push(OpCode::Increment),
            '-' => op_codes.push(OpCode::Decrement),
            '>' => op_codes.push(OpCode::IncrementPointer),
            '<' => op_codes.push(OpCode::DecrementPointer),
            ',' => op_codes.push(OpCode::Read),
            '.' => op_codes.push(OpCode::Write),
            '[' => op_codes.push(OpCode::LoopBegin),
            ']' => op_codes.push(OpCode::LoopEnd),
            _ => (),
        };
    };

    op_codes
}

fn parse(op_codes: Vec<OpCode>) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;

    for (i, op) in op_codes.iter().enumerate() {
        if loop_stack == 0 {
            let instr = match op {
                OpCode::Increment => Some(Instruction::Increment),
                OpCode::Decrement => Some(Instruction::Decrement),
                OpCode::IncrementPointer => Some(Instruction::IncrementPointer),
                OpCode::DecrementPointer => Some(Instruction::DecrementPointer),
                OpCode::Read => Some(Instruction::Read),
                OpCode::Write => Some(Instruction::Write),

                OpCode::LoopBegin => {
                    loop_start = i;
                    loop_stack += 1;
                    None
                },

                OpCode::LoopEnd => {
                    println!("Error: loop ending at #{} has no beginning", i);
                    exit(420);
                }
            };

            match instr {
                Some(instr) => program.push(instr),
                None => ()
            }
        } else {
            match op {
                OpCode::LoopBegin => {
                    loop_stack += 1;
                },
                OpCode::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        program.push(Instruction::Loop(parse(op_codes[loop_start+1..i].to_vec())));
                    }
                },
                _ => (),
            }
        }
    }

    if loop_stack != 0 {
        println!("Error: loop starting at #{} has no matching ending!", loop_start);
        exit(420);
    }

    program
}

fn run(instructions: &Vec<Instruction>, memory: &mut Vec<u8>, ptr: &mut usize) {
    for instr in instructions {
        match instr {
            Instruction::Increment => memory[*ptr] += 1,
            Instruction::Decrement => memory[*ptr] -= 1,
            Instruction::IncrementPointer => *ptr += 1,
            Instruction::DecrementPointer => *ptr -= 1,
            Instruction::Read => {
                let mut input = [0u8; 1];
                std::io::stdin().read_exact(&mut input).expect("Failed to read from stdin.");
                memory[*ptr] = input[0];
            },
            Instruction::Write => print!("{}", memory[*ptr] as char),
            Instruction::Loop(looping_insturctions) => {
                while memory[*ptr] != 0 {
                    run(&looping_insturctions, memory, ptr)
                }
            }
        }
    }
}