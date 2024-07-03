use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, ErrorKind, Read, Write};
use std::process::ExitCode;

trait IsAsciiPrintable {
    fn is_ascii_printable(&self) -> bool;
}

impl IsAsciiPrintable for u8 {
    #[inline]
    fn is_ascii_printable(&self) -> bool {
        matches!(self, 32..=126)
    }
}

const BUFFER_SIZE: usize = 10240; //10KB buffer read

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

    fn display(&mut self) {
        let mut stdout = BufWriter::new(io::stdout());

        while let Ok(bytes_read) = self.reader.read(&mut self.buffer) {
            if bytes_read == 0 {
                break;
            }

            let mut space = false;
            self.buffer[..bytes_read].chunks(16).for_each(|chunk| {
                write!(stdout, "{:08x}: ", self.offset).unwrap();
                self.offset += chunk.len() as u32;

                // Display the raw hex values
                for byte in chunk {
                    write!(stdout, "{:02x}", byte).unwrap();
                    if space {
                        write!(stdout, " ").unwrap();
                    }
                    space = !space;
                }

                write!(stdout, " ").unwrap();

                // Display any printable characters
                for byte in chunk {
                    let c = *byte;
                    if c.is_ascii_printable() {
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
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    if argc < 2 {
        eprintln!("No file specified!");
        return ExitCode::FAILURE;
    }

    if &args[1] == "--help" {
        println!("Usage: hexview [filename]");
        return ExitCode::SUCCESS;
    }

    let mut viewer = match HexViewer::new(&args[1]) {
        Ok(v) => v,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => eprintln!("{}: No such file or directory", &args[1]),
                ErrorKind::PermissionDenied => eprintln!("{}: Permission denied", &args[1]),
                _ => unreachable!(),
            }
            return ExitCode::FAILURE;
        }
    };

    viewer.display();

    ExitCode::SUCCESS
}
