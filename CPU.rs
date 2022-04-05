//This is an implemwntation for only the immmediate memory mode

//Declare dependencies
use std::io::stdin;


//virtual twin of the CPU hardware
pub struct CPU {
    pub a: u8,          //accumulator
    pub x: u8,          //indexX
    pub y: u8,          //indexY
//    pub p: u8,          //status register  (todo)
//    pub s: u8,          //stack register  (todo)
    pub pc: u16,        //program counter
}

impl CPU{
    pub fn new() -> Self {
        CPU{
            register_a: 0,
            //status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>){
        self.program_counter = 0;

        loop{
            let opcode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opcode {
                //LDA
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter +=1;
                    self.register_a = param;    
                }
                //TAX
                0xAA => {
                    self.program_counter +=1;
                    self.register_x = self.register_a;
                }
                //INX
                0xE8 => {
                    self.program_counter +=1;
                    self.register_x+=1;
                }
                //BRK
                0x00 => {
                    break;
                }
                _ => todo!()
            }
        }
    }

    // pub fn io(){
    //     stdin().read_line(&mut input_string)
    // 	.ok()
    //     .expect("Failed to read line");
    // }
}


mod test {
    use super::*;
  
    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
    }
}

test.test_0xa9_lda_immidiate_load_data()