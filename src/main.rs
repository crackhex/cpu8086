use std::fs::File as File;
use std::io::Read as Read;
use std::path::Path;

fn read_file_buf(path: &Path, buf: &mut Vec<u8>) -> Result<usize, std::io::Error> {
    let file = File::open(path);
    match file {
        Ok(mut f) => {
            let size = f.read_to_end(buf)?;
            //println!("Read {} bytes", size);
            Ok(size)
        },
        Err(e) => {
            Err(e)
        }
    }
}

fn cpu_mov_parse(buf: &[u8]) -> () {
    //let length = buf.len();
    let _ = buf.iter().enumerate().for_each(|(i, &x)| {

    });
}
fn main() {
    let mut buf: Vec<u8> = vec![];
    let path = Path::new("files/listing_37");
    match read_file_buf(&path, &mut buf) {
        Ok(n) => {
            println!("Read {} bytes", n);
            println!("{:?}", buf);
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    let x= cpu_mov_parse(&buf);
    println!("{:?}", x);
}
