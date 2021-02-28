mod mem;

use std::fs::File;
use std::io::{Read, Write};
use std::ops::{Index, IndexMut};
use std::borrow::BorrowMut;
use std::io;


struct CPU {
    code: Vec<u8>,
    mem: mem::Mem,
    ip: usize,
    dp: usize,
}

impl CPU {
    fn new(code: Vec<u8>, mem: mem::Mem) -> Self {
        Self {
            code,
            mem,
            ip: 0,
            dp: 0,
        }
    }

    fn inc_dp(&mut self) {
        self.dp += 1;
    }

    fn dec_dp(&mut self) {
        self.dp -= 1;
    }

    fn inc_dp_value(&mut self) {
        self.mem[self.dp].wrapping_add(1);
    }

    fn dec_dp_value(&mut self) {
        self.mem[self.dp].wrapping_sub(1);
    }

    fn out_dp_value(&mut self) {
        print!("{}", self.mem[self.dp] as u8 as char);
        io::stdout().flush().unwrap();
    }

    fn inp_dp_value(&mut self) {
        eprint!("Input: ");
        let value= match std::io::stdin().bytes().next() {
            None => Ok(0), // On EOF
            Some(v) => v,
        }.unwrap();
        // Discard all other chars
        std::io::stdin().bytes().map(|result| result.unwrap()).take_while(|byte| *byte as char != '\n');
        println!("Read {}", value as char);
        self.mem[self.ip] = value as i8;
    }

    fn add(u: usize, i: i8) -> usize {
        if i.is_negative() {
            u - i.abs() as usize
        } else {
            u + i as usize
        }
    }

    fn jump(&mut self, inverse: bool) {
        let step = match inverse {
            true => -1,
            false => 1,
        };
        self.ip = Self::add(self.ip, step);
        let mut braces = 1;
        while braces > 0 {
            match self.code[self.ip].into() {
                '[' => braces += step,
                ']' => braces -= step,
                _ => (),
            };
            self.ip = Self::add(self.ip, step);
        }
    }

    fn jump_zero(&mut self) {
        match self.mem[self.dp] {
            0 => self.jump(false),
            _ => (),
        }
    }

    fn jump_nonzero(&mut self) {
        match self.mem[self.dp] {
            0 => (),
            _ => self.jump(true),
        }
    }

    fn execute(&mut self, opcode: u8) {
        // let opcode = opcode.into();
        match opcode as char {
            '>' => self.inc_dp(),
            '<' => self.dec_dp(),
            '+' => self.inc_dp_value(),
            '-' => self.dec_dp_value(),
            '.' => self.out_dp_value(),
            ',' => self.inp_dp_value(),
            '[' => self.jump_zero(),
            ']' => self.jump_nonzero(),
            _ => (),
        };
    }

    fn fetch_decode_execute(&mut self) {
        // Fetch
        let opcode = self.code[self.ip];
        // Decode + Execute
        self.execute(opcode);
        self.ip += 1;
    }

    fn run(&mut self) {
        while self.ip < self.code.len() {
            self.fetch_decode_execute();
        }
    }
}

fn main() -> std::io::Result<()> {
    let filename = "/home/linus/code/rust/brainfuck/src/hello_world.bf";
    let mut file = File::open(filename)?;
    let mut code = Vec::with_capacity(512);
    file.read_to_end(&mut code)?;
    let mut cpu = CPU::new(code, mem::Mem::new(512));
    cpu.run();
    Ok(())
}
