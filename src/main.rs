use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process::exit;

const BUFFER_SIZE: usize = 10240; //10KB read at a time default

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

    fn mainloop(&mut self) {
        while let Ok(bytes_read) = self.reader.read(&mut self.buffer) {
            if bytes_read == 0 {
                break;
            }

            let mut space = false;
            self.buffer[..bytes_read].chunks(16).for_each(|chunk| {
                print!("{:08x}: ", self.offset);
                self.offset += chunk.len() as u32;
                for byte in chunk {
                    print!("{:02x}", byte);
                    if space {
                        print!(" ");
                    }
                    space = !space;
                }
                print!(" ");

                for byte in chunk {
                    let c = *byte;
                    if c > 32 && c < 127 {
                        //
                        print!("{}", c as char);
                    } else {
                        print!(".");
                    }
                }
                println!();
            });
        }
    }

    fn raw(&mut self) {
        while let Ok(bytes_read) = self.reader.read(&mut self.buffer) {
            if bytes_read == 0 {
                break;
            }

            self.buffer[..bytes_read].chunks(16).for_each(|chunk| {
                for byte in chunk {
                    print!("{:02x}", byte);
                }
                println!();
            });
        }
    }
}

fn import(filename: &str) -> io::Result<()> {
    let reader = BufReader::new(File::open(filename)?);
    for line in reader.lines().map(|l| l.unwrap()) {
        for byte in line.as_bytes().chunks(2) {
            if byte.len() != 2 {
                eprintln!("Bad hex value.");
                exit(1);
            }

            let val = u8::from_str_radix(&String::from_utf8_lossy(byte), 16).unwrap();
            io::stdout().write_all(&[val]).unwrap();
        }
        io::stdout().flush().unwrap();
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    if argc < 2 {
        eprintln!("No file specified!");
        exit(1);
    }

    if &args[1] == "--help" {
        println!("Usage: hexview [filename] (-r/--raw)");
        return Ok(());
    }

    if &args[1] == "import" {
        if argc < 3 {
            eprintln!("import: no file specified");
            exit(1);
        }
        import(&args[2])?;
        exit(0);
    }

    let mut viewer = HexViewer::new(&args[1])?;

    if args.contains(&String::from("-r")) || args.contains(&String::from("--raw")) {
        viewer.raw();
    } else {
        viewer.mainloop();
    }

    Ok(())
}
