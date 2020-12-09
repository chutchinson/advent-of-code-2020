mod prelude;
use prelude::*;

type Instruction<'a> = (&'a str, isize);

fn execute(program: &Vec<Instruction>, acc: &mut isize) -> bool {
    let mut pc = 0;
    let mut visited = HashSet::new();

    while pc < program.len() {
        let op = program[pc];
        if visited.contains(&pc) {
            return false;
        }
        visited.insert(pc);
        match op {
            ("nop", _) => { pc += 1 },
            ("acc", value) => { pc += 1; *acc += value },
            ("jmp", offset) => { pc = (pc as isize + offset) as usize },
            _ => ()
        }
    }

    return true;
}

fn patch_and_execute(program: &Vec<Instruction>, acc: &mut isize) -> bool {
    let count = program.len();
    for index in 0..count {
        *acc = 0;
        let mut program = program.clone();
        let (ref mut op, _) = program[index];
        *op = if *op == "nop" { "jmp" } else { "nop" };
        if execute(&program, acc) {
            return true;
        }
    }
    false
}

fn compile(code: &str) -> Vec<Instruction> {
    code.lines()
        .map(|line| {
            let mut parts = line.split(" ");
            let op = parts.next().unwrap();
            let value = parts.next().unwrap().parse::<isize>().unwrap();
            (op, value)
        })
        .collect_vec()
}

fn main() {
    let mut code = String::new();
    let _ = stdin().lock().read_to_string(&mut code);

    let program = compile(&code);
    let mut acc = 0;

    execute(&program, &mut acc);
    println!("{}", acc);

    patch_and_execute(&program, &mut acc);
    println!("{}", acc);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn code() -> &'static str {
        return "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6\n";
    }

    #[test]
    fn example_1() {
        let mut acc = 0;
        let program = compile(code());
        execute(&program, &mut acc);
        assert_eq!(5, acc);
    }

    #[test]
    fn example_2() {
        let mut acc = 0;
        let program = compile(code());
        let terminated = patch_and_execute(&program, &mut acc);
        assert_eq!(8, acc);
        assert_eq!(true, terminated);
    }
}