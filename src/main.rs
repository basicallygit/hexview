use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
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
        let mut stdout = BufWriter::new(io::stdout());

        while let Ok(bytes_read) = self.reader.read(&mut self.buffer) {
            if bytes_read == 0 {
                break;
            }

            let mut space = false;
            self.buffer[..bytes_read].chunks(16).for_each(|chunk| {
                write!(stdout, "{:08x}", self.offset).unwrap();
                self.offset += chunk.len() as u32;

                for byte in chunk {
                    write!(stdout, "{:02x}", byte).unwrap();
                    if space {
                        write!(stdout, " ").unwrap();
                    }
                    space = !space;
                }

                write!(stdout, " ").unwrap();

                for byte in chunk {
                    let c = *byte;
                    if c > 32 && c < 127 {
                        //
                        write!(stdout, "{}", c as char).unwrap();
                    } else {
                        write!(stdout, ".").unwrap();
                    }
                }
                writeln!(stdout).unwrap();
            });
        }
        stdout.flush().unwrap();
    }

    fn raw(&mut self) {
        let mut stdout = BufWriter::new(io::stdout());

        while let Ok(bytes_read) = self.reader.read(&mut self.buffer) {
            if bytes_read == 0 {
                break;
            }

            self.buffer[..bytes_read].chunks(16).for_each(|chunk| {
                for byte in chunk {
                    write!(stdout, "{:02x}", byte).unwrap();
                }
                writeln!(stdout).unwrap();
            });
        }
    }
}

fn import(filename: &str) -> io::Result<()> {
    let reader = BufReader::new(File::open(filename)?);
    let mut stdout = BufWriter::new(io::stdout());

    for line in reader.lines().map(|l| l.unwrap()) {
        for byte in line.as_bytes().chunks(2) {
            let val = u8::from_str_radix(&String::from_utf8_lossy(byte), 16).unwrap();
            stdout.write_all(&[val]).unwrap();
        }
    }
    stdout.flush().unwrap();

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
