use std::env;
use std::cmp;
use std::io;
use std::io::BufRead;
use std::io::Write;

const ESCAPE_LENGTH: usize = 9;

fn encode(mut src: &[u8], dst: &mut Vec<u8>) {
    loop {
        let copy_len = cmp::min(ESCAPE_LENGTH - 1, src.len());
        dst.extend_from_slice(&src[0..copy_len]);
        src = &src[copy_len..];
        if src.len() == 0 {
            let pad_size = ESCAPE_LENGTH - 1 - copy_len;
            if pad_size > 0 {
                dst.resize(dst.len() + pad_size, 0);
            }
            dst.push(copy_len as u8);
            break;
        }
        dst.push(ESCAPE_LENGTH as u8);
    }
}

fn decode(mut src: &[u8], dst: &mut Vec<u8>) {
    let mut rest = src.len() - src.len() / ESCAPE_LENGTH - ESCAPE_LENGTH + 1 + src[src.len() - 1] as usize;
    while rest > 0 {
        let copy_len = cmp::min(ESCAPE_LENGTH - 1, rest);
        dst.extend_from_slice(&src[0..copy_len]);
        src = &src[ESCAPE_LENGTH..];
        rest -= copy_len;
    }
}

enum Subcmd {
    Encode,
    Decode,
}

fn main() -> io::Result<()> {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("invalid args");
        return Err(io::Error::from(io::ErrorKind::Other));
    }
    args.next().unwrap();
    let subcmd = args.next().unwrap();
    let subcmd = match subcmd.as_str() {
        "encode" => Subcmd::Encode,
        "decode" => Subcmd::Decode,
        _ => {
            eprintln!("invalid args");
            return Err(io::Error::from(io::ErrorKind::Other));
        },
    };
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut stdout = io::BufWriter::new(stdout);
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut stdin = io::BufReader::new(stdin);
    let mut line = vec![];
    let mut encoded = vec![];
    loop {
        line.clear();
        let read = stdin.read_until(b'\n', &mut line)?;
        if read == 0 {
            break;
        }
        let line = &line[0..line.len() - 1];
        encoded.clear();
        match subcmd {
            Subcmd::Encode => encode(line, &mut encoded),
            Subcmd::Decode => decode(line, &mut encoded),
        }
        stdout.write(&encoded)?;
        stdout.write("\n".as_bytes())?;
    }
    Ok(())
}
