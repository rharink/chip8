const ARITHMETIC_AND_LOGIC: u8 = 0x8;
const ADD_XY: u8 = 0x4;
const HALT: u8 = 0x0;

struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 4096],
}

impl CPU {
    fn run(&mut self) {
        loop {
            let op_byte1 = self.memory[self.position_in_memory] as u16;
            let op_byte2 = self.memory[self.position_in_memory + 1] as u16;
            let raw_op   = op_byte1 << 8 | op_byte2;

            debug_n("op_byte1".to_string(), op_byte1);
            debug_n("op_byte2".to_string(), op_byte2);
            debug_n("raw_op".to_string(), raw_op);

            let op_major = ((raw_op & 0xF000) >> 12) as u8;
            debug_mask_shift("op_major", raw_op, 0xF000, 12);

            let x =        ((raw_op & 0x0F00) >> 8)  as u8;
            debug_mask_shift("x       ", raw_op, 0x0F00, 8);

            let y =        ((raw_op & 0x00F0) >> 4)  as u8;
            debug_mask_shift("y       ", raw_op, 0x00F0, 4);

            let op_minor =  (raw_op & 0x000F) as u8;
            debug_mask_shift("op_minor", raw_op, 0x000F, 0);

            self.position_in_memory += 2;

            match (op_major, op_minor) {
                (HALT, HALT) => { return; },
                (ARITHMETIC_AND_LOGIC, ADD_XY) => self.add_xy(x, y),
                _ => unimplemented!(),
            }

            println!("--------------------------------------------");
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory:0,
    };


    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    cpu.memory[0] = 0x80; cpu.memory[1] = 0x14;
    cpu.memory[2] = 0x80; cpu.memory[3] = 0x24;
    cpu.memory[4] = 0x80; cpu.memory[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);

}

fn debug_n(c: String, n: u16) {
    println!("{}: \t\t\t0x{:04X}\t{:#018b}\t{}", c, n, n, n);
}


fn debug_mask_shift(c: &str, n: u16, m: u16, s: u16) {
    debug_n(format!("{}:n", c), n);
    debug_n(format!("{}:m", c), m);
    debug_n(format!("{}:s", c), s);
    debug_n(format!("{}:m-", c), n & m);
    debug_n(format!("{}:ms-", c), (n & m) >> s);
}

