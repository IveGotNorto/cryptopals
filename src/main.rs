use std::io::{self, BufRead, Write};
use std::convert::TryInto;

fn main() {
    let mut line = String::new();
    print!("HEX: ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.pop();
    println!("B64: {:?}", hex_to_b64(line));
}

fn hex_to_b64(input: String) -> String {

    map_hex_indices(get_hex_indices(input))
    
}

fn get_hex_indices(input: String) -> Vec<u8> {

    let mut hex_bytes = string_to_hex(input);
    // Length of hex u8 array in bits
    let length: i32 = (hex_bytes.len() * 8).try_into().unwrap();
    // How many 24 bit chunks are needed
    let chunks: i32 = (f64::from(length) / f64::from(24)).ceil() as i32;
    // Padding to add on
    let pad = ((chunks * 24) - length) / 8;

    let mask: u8 = 0b0011_1111;
    let mut buff: u32 = 0;
    let mut i = 0;

    while i != pad {
        //println!("PADDING ADDED");
        hex_bytes.push(0);
        i+=1;
    }

    let mut iter = hex_bytes.iter();
    let size = pad + (round * 3);
    let mut tmp_bytes: [u8; 4] = [0; 4];
    let mut b64_bytes: Vec<u8> = Vec::with_capacity(size.try_into().unwrap());

    i = 0;
    while i < (hex_bytes.len() / 3).try_into().unwrap() {

        for _ in 0..3 {
            buff <<= 8;
            buff = buff | (*iter.next().unwrap() as u32);
        }

        for j in (0..4).rev() {
            tmp_bytes[j] = (buff & (mask as u32)).try_into().unwrap();
            buff >>= 6;
        }
        b64_bytes.extend_from_slice(&tmp_bytes);
        
        buff = 0;
        i+=1;
    }

    b64_bytes
}

fn string_to_hex(input: String) -> Vec<u8> {

    let mut tmp: Vec<u8> = Vec::new();
    let mut buff: u8 = 0;
    let mut reg: u8;

    let len = input.len();
    
    for (i, c) in input.to_uppercase()
                        .chars()
                        .into_iter()
                        .enumerate() {
        reg = match c {
           '1'=> 0b0000_0001,
           '2' => 0b0000_0010,
           '3' => 0b0000_0011,
           '4' => 0b0000_0100,
           '5' => 0b0000_0101,
           '6' => 0b0000_0110,
           '7' => 0b0000_0111,
           '8' => 0b0000_1000,
           '9' => 0b0000_1001,
           'A' => 0b0000_1010,
           'B' => 0b0000_1011,
           'C' => 0b0000_1100,
           'D' => 0b0000_1101,
           'E' => 0b0000_1110,
           'F' => 0b0000_1111,
           _ => 0b0000_0000
        };
        buff <<= 4;
        buff |= reg;

        if i % 2 != 0 {
            tmp.push(buff);
            buff = 0;
        }
    }

    if len % 2 != 0 {
        buff <<= 4;
        tmp.push(buff);
    }

    tmp
}

fn map_hex_indices(arr: Vec<u8>) -> String {

    let mut buff = String::new();

    for i in arr.iter() {
        buff.push(match i {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            4 => 'E',
            5 => 'F',
            6 => 'G',
            7 => 'H',
            8 => 'I',
            9 => 'J',
            10 => 'K',
            11 => 'L',
            12 => 'M',
            13 => 'N',
            14 => 'O',
            15 => 'P',
            16 => 'Q',
            17 => 'R',
            18 => 'S',
            19 => 'T',
            20 => 'U',
            21 => 'V',
            22 => 'W',
            23 => 'X',
            24 => 'Y',
            25 => 'Z',
            26 => 'a',
            27 => 'b',
            28 => 'c',
            29 => 'd',
            30 => 'e',
            31 => 'f',
            32 => 'g',
            33 => 'h',
            34 => 'i',
            35 => 'j',
            36 => 'k',
            37 => 'l',
            38 => 'm',
            39 => 'n',
            40 => 'o',
            41 => 'p',
            42 => 'q',
            43 => 'r',
            44 => 's',
            45 => 't',
            46 => 'u',
            47 => 'v',
            48 => 'w',
            49 => 'x',
            50 => 'y',
            51 => 'z',
            52 => '0',
            53 => '1',
            54 => '2',
            55 => '3',
            56 => '4',
            57 => '5',
            58 => '6',
            59 => '7',
            60 => '8',
            61 => '9',
            62 => '+',
            63 => '/',
            _ => '='
        });
    }
    buff 
}
