use nom::branch::alt;
use nom::character::complete;
use nom::character::complete::{alpha1, newline, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

enum Instruction {
    Noop,
    AddX(i32),
}

fn no_arg_instruction(input: &str) -> IResult<&str, Instruction> {
    map_res(alpha1, |word| match word {
        "noop" => Ok(Instruction::Noop),
        _ => Err(()),
    })(input)
}

fn arg_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, func) = map_res(alpha1, |word| match word {
        "addx" => Ok(Instruction::AddX),
        _ => Err(()),
    })(input)?;
    let (input, _) = space1(input)?;
    let (input, amt) = complete::i32(input)?;
    Ok((input, func(amt)))
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, alt((arg_instruction, no_arg_instruction)))(input)
}

enum Busy {
    No,
    AddX(i32, u32),
}

struct Cpu {
    instructions: Vec<Instruction>,
    index: usize,
    x: i32,
    clock: i32,
    busy: Busy,
    strengths: Vec<i32>,
}

impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            index: 0,
            x: 1,
            clock: 0,
            busy: Busy::No,
            strengths: vec![],
        }
    }

    fn cycle(&mut self) -> bool {
        self.clock += 1;
        if self.clock >= 20 && (self.clock - 20) % 40 == 0 {
            self.strengths.push(self.clock * self.x);
        }
        match self.busy {
            Busy::No => {
                match *self.instructions.get(self.index).unwrap() {
                    Instruction::Noop => {}
                    Instruction::AddX(x) => {
                        self.busy = Busy::AddX(x, 1);
                    }
                }
                self.index += 1;
            }
            Busy::AddX(x, 1) => {
                self.x += x;
                self.busy = Busy::No;
            }
            Busy::AddX(..) => {
                unreachable!()
            }
        }
        self.index != self.instructions.len()
    }

    fn draw(&self) -> bool {
        (self.x - 1..=self.x + 1).contains(&(self.clock % 40))
    }
}

pub fn part1(input: &str) -> String {
    let (_, data) = instructions(input).unwrap();
    let mut cpu = Cpu::new(data);
    while cpu.cycle() {}
    cpu.strengths.iter().sum::<i32>().to_string()
}

pub fn part2(input: &str) -> String {
    let (_, data) = instructions(input).unwrap();
    let mut cpu = Cpu::new(data);
    let mut screen = vec![];
    while cpu.cycle() {
        screen.push(cpu.draw());
    }
    let text: Vec<char> = screen.iter().map(|x| if *x { '#' } else { ' ' }).collect();
    text.chunks(40)
        .map(|x| "\n".to_string() + &x.iter().collect::<String>())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), "13140");
    }

    #[test]
    fn test_part2() {
        part2(DATA1);
    }
}
