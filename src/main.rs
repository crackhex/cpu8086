use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Default)]
struct FirstByte {
    byte: u8,
}
#[derive(Default)]
struct SecondByte {
    byte: u8,
}
#[derive(Default)]
struct ThirdByte {
    byte: u8,
}
#[derive(Default)]
struct FourthByte {
    byte: u8,
}
#[derive(Default)]
struct FifthByte {
    byte: u8,
}
#[derive(Default)]
struct SixthByte {
    byte: u8,
}
#[derive(Default)]
struct Instruction {
    first_byte: FirstByte,
    second_byte: SecondByte,
    third_byte: Option<ThirdByte>,
    fourth_byte: Option<FourthByte>,
    fifth_byte: Option<FifthByte>,
    sixth_byte: Option<SixthByte>,
}
impl FirstByte {
    const fn read_opcode(&self) -> u8 {
        self.byte >> 2
    }
    const fn read_reg_direction(&self) -> bool {
        (self.byte & 0b00000010) == 0b00000010
    }
    const fn read_word_or_byte(&self) -> bool {
        (self.byte & 0b00000001) == 0b00000001
    }
}
impl SecondByte {
    const fn read_reg_mem(&self) -> u8 {
        self.byte >> 6
    }
    const fn read_reg_op_opcode(&self) -> u8 {
        (self.byte & 0b00111000) >> 3
    }
    const fn read_reg_ea_calc(&self) -> u8 {
        self.byte & 0b00000111
    }
}
fn read_file_buf(path: &Path, buf: &mut Vec<u8>) -> Result<usize, std::io::Error> {
    let file = File::open(path);
    match file {
        Ok(mut f) => {
            let size = f.read_to_end(buf)?;
            //println!("Read {} bytes", size);
            Ok(size)
        }
        Err(e) => Err(e),
    }
}

fn parse_byte_stream(buf: &[u8]) -> Vec<Instruction> {
    //let length = buf.len();
    let mut instruction_vec: Vec<Instruction> = vec![];
    let mut idx = 0;
    while idx < buf.len() {
        let mut instruction = Instruction::default();
        instruction.first_byte.byte = buf[idx];
        instruction.second_byte.byte = buf[idx + 1];
        idx += 2;
        let opcode = instruction.first_byte.read_opcode();
        instruction_vec.push(instruction);
    }
    instruction_vec
}
fn main() {
    let mut buf: Vec<u8> = vec![];
    let path = Path::new("files/listing_37");
    match read_file_buf(path, &mut buf) {
        Ok(n) => {
            println!("Read {} bytes", n);
            println!("{:?}", buf);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    let x = parse_byte_stream(&buf);
    println!("{:#06b}", x[0].first_byte.read_opcode());
}
