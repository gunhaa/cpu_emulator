// CHIP-8 시스템의 일부분
fn main() {

    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        program_counter: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    // cpu.registers[2] = 10;
    // cpu.registers[3] = 10;

    // let mem = &mut cpu.memory;
    // 2byte에서 첫 4비트(레지스터 연산)와 마지막 4비트는 연산 종류(+)를 의미하며, 
    // 가운데 8비트 두개는 사용 레지스터를 의미한다
    // ex 0x8034 => 84를 이용한 연산(+), 0번 레지스터와 3번레지스터를 이용(84를 이용한 연산, +)
    // mem[0] = 0x80; mem[1] = 0x14;
    // mem[2] = 0x80; mem[3] = 0x24;
    // mem[4] = 0x80; mem[5] = 0x34;

    let mem = &mut cpu.memory;

    // CALL 0x100
    mem[0x000] = 0x21; mem[0x001] = 0x00;
    // CALL 0x100
    mem[0x002] = 0x21; mem[0x003] = 0x00;
    // RET 0x0000 (프로그램 종료)
    mem[0x004] = 0x00; mem[0x005] = 0x00;

    // ADD V0, V1
    mem[0x100] = 0x80; mem[0x101] = 0x14;
    // ADD V0, V1
    mem[0x102] = 0x80; mem[0x103] = 0x14;
    // 복귀
    mem[0x104] = 0x00; mem[0x105] = 0xEE;

    cpu.run();

    // assert_eq!(cpu.registers[0], 35);
    assert_eq!(cpu.registers[0], 45);

    println!("5+ (10*2) + (10*2) = {}", cpu.registers[0]);
}

#[derive(Debug)]
struct CPU {
    registers: [u8; 16],
    program_counter: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
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

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!");
        }

        stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    fn ret(&mut self){
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }
        
        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.program_counter = addr as usize
    }



    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.program_counter += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let nnn = opcode & 0x0FFF;
            //let kk = (opcode & 0x00FF) as u8;
            
            match (c,x,y,d) {
                (0, 0, 0, 0)     => { return; },
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x,y),
                _                => todo!("opcode {:04x}", opcode),
            } 
        }
    }
}