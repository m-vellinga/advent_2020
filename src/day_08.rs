use std::collections::HashSet;

pub fn run(input: String) {
    let instructions = convert_input(input);

    println!("Day 8 Part 1: {}", part_1(&instructions));
    println!("Day 8 Part 2: {}", part_2(&instructions));
}

struct Instruction {
    operation: String,
    argument: i64,
}

fn convert_input(input: String) -> Vec<Instruction> {
    let mut instructions = vec![];
    for line in input.lines() {
        let line = line.trim();
        let line_parts: Vec<&str> = line.split(" ").collect();
        let instruction = *line_parts.get(0).unwrap();
        let argument: i64 = line_parts.get(1).unwrap().parse().unwrap();

        instructions.push(Instruction {
            operation: String::from(instruction),
            argument: argument,
        });
    }
    instructions
}

fn part_1(instructions: &Vec<Instruction>) -> i64 {
    let mut accumulator: i64 = 0;
    let mut played_instructions = HashSet::new();

    let mut instruction = instructions.first().unwrap();
    let mut index = 0;

    while played_instructions.insert(index) {
        match &instruction.operation[..] {
            "acc" => {
                index += 1;
                accumulator += instruction.argument;
            }
            "jmp" => index += instruction.argument,
            "nop" => index += 1,
            _ => panic!("Unkown instruction"),
        }
        instruction = instructions.get(index as usize).unwrap();
    }
    accumulator
}

fn part_2(instructions: &Vec<Instruction>) -> i64 {
    for (i, instruction) in instructions.iter().enumerate() {
        match &instruction.operation[..] {
            "jmp" => (),
            "nop" => (),
            _ => continue,
        }

        let mut accumulator: i64 = 0;
        let mut played_instructions = HashSet::new();

        let mut next_instruction = instructions.first().unwrap();
        let mut index = 0;

        while played_instructions.insert(index) {
            match &next_instruction.operation[..] {
                "acc" => {
                    index += 1;
                    accumulator += next_instruction.argument;
                }
                "jmp" => {
                    index += if index == i as i64 {
                        1
                    } else {
                        next_instruction.argument
                    }
                }
                "nop" => {
                    index += if index == i as i64 {
                        next_instruction.argument
                    } else {
                        1
                    }
                }
                _ => panic!("Unkown instruction"),
            }
            if index == instructions.len() as i64 {
                return accumulator;
            }
            next_instruction = instructions.get(index as usize).unwrap();
        }
    }

    panic!("Not found!");
}

#[test]
fn test_part_1() {
    let input = String::from(
        "nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6",
    );

    let instructions = convert_input(input);
    let answer = part_1(&instructions);

    assert_eq!(answer, 5);
}

#[test]
fn test_part_2() {
    let input = String::from(
        "nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6",
    );

    let instructions = convert_input(input);
    let answer = part_2(&instructions);

    assert_eq!(answer, 8);
}
