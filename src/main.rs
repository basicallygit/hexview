use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::process::exit;

const BUFFER_SIZE: usize = 4096;

struct HexViewer {
    buffer: [u8; BUFFER_SIZE],
    offset: u32,
    reader: BufReader<File>,
}

impl HexViewer {
    fn new(filename: &str) -> io::Result<Self> {
        Ok(Self {
            buffer: [0; BUFFER_SIZE],
            offset: 0,
            reader: BufReader::new(File::open(filename)?),
        })
    }

    fn mainloop(&mut self) -> io::Result<()> {
        loop {
            let bytes_read = self.reader.read(&mut self.buffer)?;

            if bytes_read == 0 {
                break;
            }

            self.buffer[..bytes_read].chunks(16).for_each(|chunk| {
                print!("{:08x} | ", self.offset);
                self.offset += chunk.len() as u32;
                for byte in chunk {
                    print!("{:02x} ", byte);
                }
                print!("  ");

                for byte in chunk {
                    let c = *byte as char;
                    if c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c == ' ' {
                        print!("{}", c);
                    } else {
                        print!(".");
                    }
                }
                println!();
            });
        }

        Ok(())
    }
}

#[allow(non_snake_case)]
fn print_extra_info(filename: &str) -> io::Result<()> {
    let mut header_data = [0u8; 6];
    File::open(filename)?.read_exact(&mut header_data)?;

    match header_data {
        [0x7F, 0x45, 0x4C, 0x46, word_size, endianness] => {
            print!("[ELF] ");
            match word_size {
                0x01 => print!("32-bit"),
                0x02 => print!("64-bit"),
                _ => print!("Unknown word size"),
            }
            print!(", ");

            match endianness {
                0x01 => print!("little endian"),
                0x02 => print!("big endian"),
                _ => print!("Unknown endianness"),
            }
            println!();

            Ok(())
        }
        _ => Ok(()),
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    if argc < 2 {
        eprintln!("No file specified!");
        exit(1);
    }

    if cfg!(target_os = "linux") {
        print_extra_info(&args[1]).ok();
    }

    let mut viewer = HexViewer::new(&args[1])?;
    viewer.mainloop()?;

    Ok(())
}
