type Number = u32;

pub fn csv_to_numbers() -> Vec<Number> {
    csv_from!("aoc02")
        .map(|val| val.parse::<Number>().unwrap_or(99))
        .collect()
}

pub fn part_1() -> Number {
    *run(&mut csv_to_numbers(), 12, 2).first().unwrap()
}

pub fn part_2() -> Number {
    for noun in 0..99 {
        for verb in 0..99 {
            if *run(&mut csv_to_numbers(), noun, verb).first().unwrap() == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    return 0
}

pub fn run(program: &mut Vec<Number>, noun: Number, verb: Number) -> Vec<Number> {
    let mut index = 0;

    program[1] = noun;
    program[2] = verb;

    while program[index] != 99 {
        process_opcode(program, index);
        index += 4;
    }

    program.to_vec()

}

pub fn process_opcode(data: &mut Vec<Number>, index: usize) {
    let destination: usize = data[index+3] as usize;
    let result = match data[index] {
        1 => data[data[index+1] as usize] + data[data[index+2] as usize],
        2 => data[data[index+1] as usize] * data[data[index+2] as usize],
        _ => unreachable!()
    };
    data[destination] = result;
}

#[cfg(test)]
mod aoc02_test;
