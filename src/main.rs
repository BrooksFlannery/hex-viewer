const CHUNK_SIZE: usize = 16;
const HEX: &[u8; 16] = b"0123456789abcdef";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: hex-editor <file>");
        std::process::exit(1);
    }

    let file_path = &args[1];

    let bytes = std::fs::read(file_path).unwrap_or_else(|e| {
        eprintln!("Failed to read {file_path}: {e}");
        std::process::exit(1);
    });

    for (i, chunk) in bytes.chunks(CHUNK_SIZE).enumerate() {
        let offset = i * CHUNK_SIZE;
        let mut hex_part = String::with_capacity(chunk.len() * 3);
        let mut ascii_part = String::with_capacity(chunk.len());

        for (j, byte) in chunk.iter().enumerate() {
            hex_part.push(HEX[(byte >> 4) as usize] as char);
            hex_part.push(HEX[(byte & 0x0f) as usize] as char);
            if j % 2 == 1 {
                hex_part.push(' ');
            }

            if (0x20..=0x7e).contains(byte) {
                ascii_part.push(*byte as char);
            } else {
                ascii_part.push('.');
            }
        }

        println!("{:08x}: {:<40} {}", offset, hex_part, ascii_part);
    }
}
