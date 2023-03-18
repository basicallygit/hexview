use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;

type Result<T> = std::io::Result<T>;

const BUFFER_SIZE: usize = 4096;

struct HexViewer {
    buffer: [u8; BUFFER_SIZE],
    offset: u32,
    reader: BufReader<File>,
}

impl HexViewer {
    fn new(filename: &str) -> Result<Self> {
        Ok(Self {
            buffer: [0; BUFFER_SIZE],
            offset: 0,
            reader: BufReader::new(File::open(filename)?),
        })
    }

    fn mainloop(&mut self) -> Result<()> {
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

fn is_binary_or_object(filename: &str) -> Result<(bool, u8, u8)> {
    let mut magic_number = [0; 6];
    File::open(filename)?.read_exact(&mut magic_number)?;

    match magic_number {
        //Magic number for ELF / Shared Objects
        [0x7F, 0x45, 0x4C, 0x46, bitmode, endian] => Ok((true, bitmode, endian)),
        _ => Ok((false, 0, 0)),
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
        if let Ok((true, bitmode, endian)) = is_binary_or_object(&args[1]) {
            print!("{}: ELF / Shared Object", &args[1]);
            match bitmode {
                0x01 => print!(", 32-bit"),
                0x02 => print!(", 64-bit"),
                _ => print!(", unknown bitmode"),
            }
            match endian {
                0x01 => print!(", little endian"),
                0x02 => print!(", big endian"),
                _ => print!(", unknown endian"),
            }
            println!();
        }
    }

    let mut viewer = HexViewer::new(&args[1])?;
    viewer.mainloop()?;

    Ok(())
}
