use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// the text to be converted into hexadecimal
    text: String,
}

fn main() {
    let args = Args::parse();
    
    // converts text into an u8 vector
    let bytes: Vec<u8> = args.text.bytes().collect();

    // the number of bytes represented per line in the terminal.
    let width: usize = 16;

    // chunks the vector into slices of 16
    let byte_chunks: Vec<&[u8]> = bytes.chunks(16).collect();

    for byte_chunk in byte_chunks {
        // every byte will be represented by 2 hexadecimal digits, followed by a
        // space (a total of 3 characters each). The 8th byte will be followed
        // by 2 spaces instead of 1 (+1), and the last byte will be followed by
        // no spaces (-1).
        let mut hex_line = String::with_capacity(width * 3);

        for byte in byte_chunk {
            let hex = format!("{:02x} ", byte);
            hex_line.push_str(&hex);
        }

        // trims trailing whitespace
        hex_line.remove(hex_line.len() - 1);
        
        if 23 < hex_line.len() {
            hex_line.insert(23, ' ');
        }

        println!("{}", hex_line);
    }
}
