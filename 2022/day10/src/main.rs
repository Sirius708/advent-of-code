#[derive(Debug)]
enum Instruction {
    NoOp,
    AddX(i32),
}

impl Instruction {
    pub fn get_cycles(&self) -> i32 {
        match self {
            Instruction::NoOp => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

fn next_special_cycle(current: i32) -> i32 {
    if current < 20 {
        return 20;
    }
    current + (40 - ((current - 20) % 40))
}

fn main() {
    let input_lines = util::get_input_lines();
    let instructions = input_lines.into_iter().map(|line| {
        if line.starts_with("noop") {
            Instruction::NoOp
        } else if line.starts_with("addx") {
            Instruction::AddX(line.split_once(' ').unwrap().1.parse().unwrap())
        } else {
            unreachable!()
        }
    });

    let mut register_x = 1;
    let mut cycles = 0;
    let mut signal_strength_sum = 0;

    let mut screen = [[false; 40]; 6];

    for inst in instructions {
        let inst_cycles = inst.get_cycles();
        let special_cycle = next_special_cycle(cycles);

        if (cycles..=(cycles + inst_cycles)).contains(&special_cycle) {
            signal_strength_sum += special_cycle * register_x;
        }

        for k in cycles..(cycles + inst_cycles) {
            let x = k % 40;
            let y = k / 40;
            if ((register_x - 1)..=(register_x + 1)).contains(&x) {
                screen[y as usize][x as usize] = true;
            }
        }

        cycles += inst_cycles;

        if let Instruction::AddX(val) = inst {
            register_x += val;
        }
    }

    println!("Signal strength sum: {}", signal_strength_sum);
    println!();

    for row in screen {
        for col in row {
            if col {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
