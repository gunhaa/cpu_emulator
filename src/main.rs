// CHIP-8 시스템의 일부분
fn main() {

    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        program_counter: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;
    // 2byte에서 첫 4비트(레지스터 연산)와 마지막 4비트는 연산 종류(+)를 의미하며, 
    // 가운데 8비트 두개는 사용 레지스터를 의미한다
    // ex 0x8034 => 84를 이용한 연산(+), 0번 레지스터와 3번레지스터를 이용(84를 이용한 연산, +)
    mem[0] = 0x80; mem[1] = 0x14;
    mem[2] = 0x80; mem[3] = 0x24;
    mem[4] = 0x80; mem[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);
    println!("5+10+10+10 = {}", cpu.registers[0]);
}

#[derive(Debug)]
struct CPU {
    registers: [u8; 16],
    program_counter: usize,
    memory: [u8; 0x1000],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        // self.current_operation

        let p = self.program_counter;

        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p+1] as u16;

        op_byte1 << 8 | op_byte2
    }

    // (c,x,y,d) == (0x8, 0, 1, 0x4)이기 때문에 add_xy(0, 1)
    // 1,4번째가 명령을 결정하고 2,3번째가 레지스터의 위치를 결정함
    fn add_xy(&mut self, x: u8, y: u8){
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize]= val;

        if overflow {
            self.registers[0xf] = 1;
        } else {
            self.registers[0xf] = 0;
        }
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.program_counter += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;
            
            match (c,x,y,d) {
                (0, 0, 0, 0)     => { return; },
                (0x8, _, _, 0x4) => self.add_xy(x,y),
                _                => todo!("opcode {:04x}", opcode),
            } 
        }
    }
}