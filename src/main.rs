use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    if argc < 2 {
        eprintln!("No file specified!");
        std::process::exit(1);
    }

    let mut file = File::open(&args[1])?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut offset = 0;

    buffer.chunks(16).for_each(|chunk| {
        print!("{:08x} | ", offset);
        offset += chunk.len() as u32;
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        print!("  ");

        for byte in chunk {
            let c = *byte as char;
            if c.is_ascii_alphanumeric() {
                print!("{}", c);
            }
            else {
                print!(".");
            }
        }
        println!();
    });

    Ok(())
}
