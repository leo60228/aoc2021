// aoc 2019 day 5 star 2 with ASCII output

use aoc2021::prelude::*;
use std::convert::{TryFrom, TryInto};
use std::io;
use std::iter;

pub fn decode(mode: isize, mut ip: usize, memory: &[isize]) -> impl Iterator<Item = isize> + '_ {
    let mut idx = 0;
    iter::from_fn(move || {
        ip += 1;
        let arg = memory[ip];
        let arg_mode = (mode / 10isize.pow(idx)) % 10;
        idx += 1;
        Some(match arg_mode {
            0 => memory[usize::try_from(arg).unwrap()],
            1 => arg,
            _ => unimplemented!(),
        })
    })
}

pub fn exec(memory: &mut [isize]) {
    let mut ip = 0;
    loop {
        let instr = memory[ip];
        let opcode = instr % 100;
        let mode = instr / 100;
        let mut decoder = decode(mode, ip, memory);
        let arity = match opcode {
            1 => {
                let a = decoder.next().unwrap();
                let b = decoder.next().unwrap();
                let c = memory[ip + 3];
                drop(decoder);

                memory[usize::try_from(c).unwrap()] = a + b;
                3
            }
            2 => {
                let a = decoder.next().unwrap();
                let b = decoder.next().unwrap();
                let c = memory[ip + 3];
                drop(decoder);

                memory[usize::try_from(c).unwrap()] = a * b;
                3
            }
            3 => {
                let a = memory[ip + 1];
                drop(decoder);

                print!("input: ");
                io::stdout().flush().unwrap();
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();

                memory[usize::try_from(a).unwrap()] = buf.trim().parse().unwrap();
                1
            }
            4 => {
                let a = decoder.next().unwrap();
                drop(decoder);

                print!("{}", char::from(a as u8));
                1
            }
            5 => {
                let a = decoder.next().unwrap();
                let b = decoder.next().unwrap();
                drop(decoder);

                if a != 0 {
                    ip = b.try_into().unwrap();
                    -1
                } else {
                    2
                }
            }
            6 => {
                let a = decoder.next().unwrap();
                let b = decoder.next().unwrap();
                drop(decoder);

                if a == 0 {
                    ip = b.try_into().unwrap();
                    -1
                } else {
                    2
                }
            }
            7 => {
                let a = decoder.next().unwrap();
                let b = decoder.next().unwrap();
                let c = memory[ip + 3];
                drop(decoder);

                memory[usize::try_from(c).unwrap()] = if a < b { 1 } else { 0 };

                3
            }
            8 => {
                let a = decoder.next().unwrap();
                let b = decoder.next().unwrap();
                let c = memory[ip + 3];
                drop(decoder);

                memory[usize::try_from(c).unwrap()] = if a == b { 1 } else { 0 };

                3
            }
            99 => break,
            unimp => unimplemented!("unimplemented opcode {}", unimp),
        };
        ip += (arity + 1) as usize;
    }
}

pub fn main() {
    let mut memory = split_line(',');

    exec(&mut memory);
}

#[cfg(test)]
mod test {
    #[test]
    fn exec() {
        use super::exec;
        fn check(inp: &[isize], out: &[isize]) {
            let mut vec: Vec<_> = inp.into();
            exec(&mut vec);
            assert_eq!(vec, out);
        }
        check(&[1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]);
        check(&[2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]);
        check(&[2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]);
        check(
            &[1, 1, 1, 4, 99, 5, 6, 0, 99],
            &[30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }
}
