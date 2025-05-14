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

    // chunks the vector into slices of 16
    let byte_chunks: Vec<&[u8]> = bytes.chunks(16).collect();

    // initializes a string vector containing the hex values of each byte in
    // byte_chunks
    let mut hex_chunks: Vec<String> = Vec::new();

    for byte_chunk in byte_chunks {
        let mut hex_chunk = String::new();

        for byte in byte_chunk {
            let hex = format!("{:02x} ", byte);
            hex_chunk.push_str(&hex);
        }

        // trims trailing whitespace
        hex_chunk = hex_chunk.trim_end().to_string();

        // forces double-space (instead of a single-space) after the first 8
        // bytes, but only if the hex_chunk has at least 8 hex bytes
        hex_chunk = match hex_chunk.split_at_checked(23) {
            Some((first, last)) => format!("{first} {last}"),
            None => hex_chunk,
        };

        hex_chunks.push(hex_chunk);
    }

    // prints the hex_chunks
    println!("{}", hex_chunks.join("\n"));
}
