use std::env;
use std::fs::File;
use std::io::{self, stdin, stdout, BufReader, BufWriter, Read, Write};
use std::path::Path;
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
                print!("0x{:08x} | ", self.offset);
                self.offset += chunk.len() as u32;
                for (i, byte) in chunk.iter().enumerate() {
                    if i == 8 {
                        print!(" ");
                    }
                    print!("{:02x} ", byte);
                }
                print!("  ");

                print!("|");
                for byte in chunk {
                    let c = *byte as char;
                    if c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c == ' ' {
                        print!("{}", c);
                    } else {
                        print!(".");
                    }
                }
                print!("|");
                println!();
            });
        }

        Ok(())
    }

    fn edit(&mut self, output_file: &str, offset: &str) -> io::Result<()> {
        if !offset.starts_with("0x") {
            eprintln!("{}: Invalid hex value, example: 0xAAAA", offset);
            exit(1);
        }

        if Path::new(output_file).exists() {
            eprintln!("{}: File already exists, aborting..", output_file);
            exit(1);
        }

        let mut out_file = BufWriter::new(File::create(output_file)?);
        let target_offset = u32::from_str_radix(&offset[2..], 16).unwrap();

        loop {
            let bytes_read = self.reader.read(&mut self.buffer)?;

            if bytes_read == 0 {
                break;
            }

            self.buffer[..bytes_read].chunks(16).for_each(|chunk| {
                if self.offset == target_offset {
                    let mut input = String::new();
                    print!("Old bytes: ");
                    for byte in chunk {
                        print!("{:02x} ", byte);
                    }
                    print!("  |");
                    for byte in chunk {
                        let c = *byte as char;
                        if c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c == ' ' {
                            print!("{}", c);
                        } else {
                            print!(".");
                        }
                    }
                    print!("|\nNew bytes: ");
                    stdout().flush().unwrap();

                    stdin().read_line(&mut input).unwrap();

                    let hex_bytes = input
                        .trim()
                        .split(' ')
                        .map(|hex| u8::from_str_radix(hex, 16).unwrap());

                    out_file.write_all(&hex_bytes.collect::<Vec<_>>()).unwrap();
                } else {
                    out_file.write_all(chunk).unwrap();
                }
                self.offset += chunk.len() as u32;
            });
        }

        println!("Successfully wrote new data");
        Ok(())
    }
}

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

            print!("2's complement, ");
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

    if &args[1] == "--help" {
        println!("Usage: hexview [filename] (--edit [offset] [output_file])");
        return Ok(());
    }

    if !Path::new(&args[1]).exists() {
        eprintln!("{}: No such file or directory.", &args[1]);
        exit(1);
    }

    if cfg!(target_os = "linux") {
        print_extra_info(&args[1]).ok();
    }

    let mut viewer = HexViewer::new(&args[1])?;

    if argc == 5 {
        if &args[2] == "--edit" {
            let offset = &args[3];
            let output_file = &args[4];
            viewer.edit(output_file, offset)?;
        } else {
            eprintln!("Usage: hexview [filename] (--edit [offset] [output_file])");
            return Ok(());
        }
    } else {
        viewer.mainloop()?;
    }

    Ok(())
}
