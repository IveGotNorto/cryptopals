use std::io::{self, BufRead};
use std::convert::TryInto;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.pop();

    let tmp = get_hex_indices(&mut line);
    println!("MAPPED: {:?}", map_hex_indices(tmp));
}

fn string_to_hex(input: &String) -> Vec<u8> {

    let mut tmp: Vec<u8> = Vec::new();
    let mut buff: u8 = 0;
    let mut reg: u8;

    let len = input.len();
    
    for (i, c) in input.to_uppercase().chars().into_iter().enumerate() {
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

fn get_hex_indices(input: &mut String) -> Vec<u8> {

    let mut hex_bytes = string_to_hex(&input);
    //println!("HEX: {:?}", hex_bytes);
    // Length of hex array in bits
    let length: i32 = (hex_bytes.len() * 8).try_into().unwrap();
    let round: i32 = (f64::from(length) / f64::from(24)).ceil() as i32;
    let pad = ((round * 24) - length) / 8;

    let mut b64_bytes: Vec<u8> = Vec::new();
    let mask: u8 = 0b0011_1111;
    let mut buff: u32 = 0;

    //println!("BITS: {}", length);

    println!("PAD: {}", pad);
    let mut i = 0;
    while i != pad {
        //println!("PADDING ADDED");
        hex_bytes.push(0);
        i+=1;
    }


    i = 0;
    let mut j = 0;
    let mut seq = hex_bytes.iter();

    let mut tmp: u32;

    while i < (hex_bytes.len() / 3).try_into().unwrap() {

        while j < 3 {
            buff <<= 8;
            tmp = *seq.next().unwrap() as u32;
            buff = buff | tmp;        
            j+=1;
        }
        j = 0;

        while j < 4 {
            b64_bytes.push((buff & (mask as u32)).try_into().unwrap());
            //println!("MASKED: {:#034b}", (buff & (mask as u32)));
            buff >>= 6;
            //println!("BUFF: {:#034b}", buff);
            j+=1; 
        }
        
        buff = 0;
        j = 0;
        i+=1;
    }

    //b64_bytes.reverse();
    b64_bytes
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
