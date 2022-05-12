pub struct CPU {
    pub register_a : u8,
    pub register_x : u8,
    pub register_y : u8,
    pub status_p: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF]
}

impl CPU {
    pub fn new() -> Self{
        CPU {
            register_a : 0,
            register_x : 0,
            register_y : 0,
            status_p: 0,
            program_counter: 0,
            memory: [0x0000; 0xFFFF], 
        }
    }

    fn load(&self, input: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + input.len())].copy_from_slice(&input[..]);
        self.program_counter = 0x8000;
    }


    fn mem_read(&self, addr: u16) -> u8 {
        let data = self.memory[addr as usize];
        self.program_counter +=1;
        data
    }

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }
    
    fn mem_write(&self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

    fn update_zero_and_negative_flags(&self, result: u8) { //idontunderstandthis. updtes zero and negative flags in status.
        if result == 0 {
            self.status_p = self.status_p | 0b0000_0010;
        } else {
            self.status_p = self.status_p & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status_p = self.status_p | 0b1000_0000;
        } else {
            self.status_p = self.status_p & 0b0111_1111;
        }
    }

    fn interpret(&self){
        self.program_counter = 0;

        //CPU Cycle is an infinite loop with checks
        loop {
            let opcode = self.mem_read(self.program_counter);
            
            match opcode {
                //LDA 
                0xA9 => {
                    let param = self.fetch_opcode();
                    self.register_a = param;
                    self.update_zero_and_negative_flags(self.register_a);
                }
                
                //TAX
                0xAA => {
                    self.register_x = self.register_a;
                    self.update_zero_and_negative_flags(self.register_x);
                }

                //INX
                0xe8 => {
                    self.register_x = self.register_x.wrapping_add(1);
                    self.update_zero_and_negative_flags(self.register_x);
                }

                //BRK
                0x00 => return,

                _ => todo!(),
            }
        }
    }
}







 
 impl CPU {
 
    //  fn mem_read(&self, addr: u16) -> u8 {
    //      self.memory[addr as usize]
    //  }
 
     
 
     pub fn load_and_run(&mut self, program: Vec<u8>) {
         self.load(program);
         self.run()
     }
 
    //  pub fn load(&mut self, program: Vec<u8>) {
    //      self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
    //      self.program_counter = 0x8000;
    //  }
 
     pub fn run(&mut self) {
     // note: we move  intialization of program_counter from here to load function
         loop {
             let opcode = self.mem_read(self.program_counter);
             self.program_counter += 1;
 
             match opcode {
                 //..
             }
         }
     }
 }






































#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.input = vec![0xa9, 0x05, 0x00];
        cpu.interpret();
        assert_eq!(cpu.register_a, 5);
        assert!(cpu.status_p & 0b0000_0010 == 0);
        assert!(cpu.status_p & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.input = vec![0xa9, 0x00, 0x00];
        cpu.interpret();
        assert!(cpu.status_p & 0b0000_0010 == 0b10);
    }
    
    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.input = vec![0xa9, 0xff, 0x00];
        cpu.interpret();
        assert!(cpu.status_p & 0b1000_0000 == 0b1000_0000);

    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.input = vec![0xaa, 0x00];
        cpu.interpret();

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.input = vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00];
        cpu.interpret();

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.input = vec![0xe8, 0xe8, 0x00];
        cpu.interpret();

        assert_eq!(cpu.register_x, 1)
    }
}
