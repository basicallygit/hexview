use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;

const BUFFER_SIZE: usize = 4096;

struct HexViewer {
    buffer: [u8; BUFFER_SIZE],
    offset: u32,
    reader: BufReader<File>,
}

impl HexViewer {
    fn new(filename: &str) -> std::io::Result<Self> {
        Ok(Self {
            buffer: [0; BUFFER_SIZE],
            offset: 0,
            reader: BufReader::new(File::open(filename)?),
        })
    }

    fn mainloop(&mut self) -> std::io::Result<()> {
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
                    if c.is_ascii_alphanumeric() || c.is_ascii_punctuation() {
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

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    if argc < 2 {
        eprintln!("No file specified!");
        exit(1);
    }

    let mut viewer = HexViewer::new(&args[1])?;
    viewer.mainloop()?;

    Ok(())
}
