pub struct CPU {
    pub register_a : u8,
    pub register_x : u8,
    pub register_y : u8,
    pub status_p: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF]
}


#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
   Immediate,
   ZeroPage,
   ZeroPage_X,
   ZeroPage_Y,
   Absolute,
   Absolute_X,
   Absolute_Y,
   Indirect_X,
   Indirect_Y,
   NoneAddressing,
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

    //Reset trigerred on every new insertion of cartrige, sets program counter
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status_p = 0;
 
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    //Standard u8 memory read for ZeroPage Memory Modes.
    fn mem_read(&mut self, addr: u16) -> u8 {
        let data = self.memory[addr as usize];
        self.program_counter +=1;
        data
    }

    //u16 memory read for Absolute Full Address Memory Modes.
    fn mem_read_u16(&mut self, addr: u16) -> u16 {
        let lo = self.mem_read(addr) as u16;
        let hi = self.mem_read(addr + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    //Standard u8 memory write for ZeroPage Memory Modes.
    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    //u16 memory write for Absolute Full Address Memory Modes.
    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

    //Update Status Register Flags
    fn update_zero_and_negative_flags(&mut self, result: u8) { //idontunderstandthis. updtes zero and negative flags in status.
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

    //Addressing Mode based Operand Fetching from Memory, for Multi-Mode Operations
    fn fetch_operand_addr(&mut self, mode: &AddressingMode) -> u16 {

        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage  => self.mem_read(self.program_counter) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);
 
                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);
 
                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    //Implied Mode Operations
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }    
    fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    } 
    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }    
    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
    }    
    fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
    } 
    fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }    
    fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    //Multi Mode Operations
    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.fetch_operand_addr(mode);
        let param = self.mem_read(addr);
        self.register_a = param;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.fetch_operand_addr(mode);
        self.mem_write(addr, self.register_a);
    }
    fn rol(&mut self, mode: &AddressingMode) {
        let addr = self.fetch_operand_addr(mode);
        let param = self.mem_read(addr);
        self.register_a = param << 1;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn ror(&mut self, mode: &AddressingMode) {
        let addr = self.fetch_operand_addr(mode);
        let param = self.mem_read(addr);
        self.register_a = param >> 1;
        self.update_zero_and_negative_flags(self.register_a);
    }

    
    fn interpret(&mut self){
        //CPU Cycle is an infinite loop with checks
        loop {
            let opcode = self.mem_read(self.program_counter);
            
            match opcode {
                //Implied Mode Operations
                0xAA => self.tax(), //TAX
                0x8A => self.txa(), //TXA
                0xCA => self.dex(), //DEX
                0xE8 => self.inx(), //INX
                0xA8 => self.tay(), //TAY
                0x98 => self.tya(), //TYA
                0x88 => self.dey(), //DEY
                0xC8 => self.iny(), //INY

                //Multi Mode Operations
                //LDA 
                0xA9 => self.lda(&AddressingMode::Immediate),                   
                0xA5 => self.lda(&AddressingMode::ZeroPage),                   
                0xB5 => self.lda(&AddressingMode::ZeroPage_X),                   
                0xAD => self.lda(&AddressingMode::Absolute),                   
                0xBD => self.lda(&AddressingMode::Absolute_X),                   
                0xB9 => self.lda(&AddressingMode::Absolute_Y),                   
                0xA1 => self.lda(&AddressingMode::Indirect_X),                   
                0xB1 => self.lda(&AddressingMode::Indirect_Y),                   
                
                //STA
                0x85 => self.sta(&AddressingMode::ZeroPage),
                0x95 => self.sta(&AddressingMode::ZeroPage_X),
                0x8D => self.sta(&AddressingMode::Absolute),
                0x9D => self.sta(&AddressingMode::Absolute_X),
                0x99 => self.sta(&AddressingMode::Absolute_Y),
                0x81 => self.sta(&AddressingMode::Indirect_X),
                0x91 => self.sta(&AddressingMode::Indirect_Y),

                //ROL
                0x2A => self.rol(&AddressingMode::Immediate),                   
                0x26 => self.rol(&AddressingMode::ZeroPage),                   
                0x36 => self.rol(&AddressingMode::ZeroPage_X),                   
                0x2E => self.rol(&AddressingMode::Absolute),                   
                0x3E => self.rol(&AddressingMode::Absolute_X),

                //ROL
                0x6A => self.ror(&AddressingMode::Immediate),                   
                0x66 => self.ror(&AddressingMode::ZeroPage),                   
                0x76 => self.ror(&AddressingMode::ZeroPage_X),                   
                0x6E => self.ror(&AddressingMode::Absolute),                   
                0x7E => self.ror(&AddressingMode::Absolute_X),

                //BRK
                0x00 => return,

                _ => todo!(),
            }
            self.program_counter += 1;
        }
    }


    fn load(&mut self, input: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + input.len())].copy_from_slice(&input[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }


    pub fn run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.interpret()
    }
}






































#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.run(vec![0xa9, 0x05, 0x00]);
        cpu.interpret();
        assert_eq!(cpu.register_a, 5);
        assert!(cpu.status_p & 0b0000_0010 == 0);
        assert!(cpu.status_p & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.run(vec![0xa9, 0x00, 0x00]);
        cpu.interpret();
        assert!(cpu.status_p & 0b0000_0010 == 0b10);
    }
    
    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.run(vec![0xa9, 0xff, 0x00]);
        cpu.interpret();
        assert!(cpu.status_p & 0b1000_0000 == 0b1000_0000);

    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.run(vec![0xaa, 0x00]);
        cpu.interpret();

        assert_eq!(cpu.register_x, 0)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        cpu.interpret();

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.run(vec![0xe8, 0x00]);
        cpu.interpret();

        assert_eq!(cpu.register_x, 1)
    }
}
