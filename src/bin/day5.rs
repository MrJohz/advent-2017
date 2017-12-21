fn main() {
    let mut memory = include_str!("inputs/day5.txt")
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let mut program_counter = 0;
    let mut lines_executed = 0;

    loop {
        let jump = memory[program_counter];
        memory[program_counter] += if jump >= 3 { -1 } else { 1 };
        lines_executed += 1;

        program_counter = ((program_counter as isize) + jump) as usize;

        if program_counter >= memory.len() || program_counter < 0 {
            println!("Program terminated after {:?} steps: jmp = {:?}", lines_executed, program_counter);
            break;
        }
    }
}
