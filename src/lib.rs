pub mod conv {

    use std::convert::TryInto;

    pub fn hex_to_b64(input: String) -> String {
        let (arr, pad) = get_hex_indices(input);
        map_hex_to_b64(arr, pad)
    }

    // might want to change a bunch of these to usize instead
    fn get_hex_indices(input: String) -> (Vec<u8>, usize) {

        let mut hex_bytes = map_string_to_hex(input);
        let mut length = hex_bytes.len();
        // Length of hex u8 array in bits
        let b_length: u32 = (hex_bytes.len() * 8).try_into().unwrap();
        // How many 24 bit chunks are needed
        let chunks = (f64::from(b_length) / f64::from(24)).ceil() as u32;
        // Padding to add on
        let pad: usize = (((chunks * 24) - b_length) / 8).try_into().unwrap();

        let mask: u8 = 0b0011_1111;
        let mut buff: u32 = 0;

        // Add padding
        hex_bytes.resize(length + pad, 0);
        length = hex_bytes.len();

        let mut iter = hex_bytes.iter();
        let mut tmp_bytes: [u8; 4] = [0; 4];
        let size: usize = (pad + (chunks * 3) as usize).try_into().unwrap();
        let mut b64_bytes: Vec<u8> = Vec::with_capacity(size);

        let mut i = 0;

        while i < (length / 3).try_into().unwrap() {
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
        (b64_bytes, pad.try_into().unwrap())
    }

    pub fn map_string_to_hex(input: String) -> Vec<u8> {

        assert!(input.len() % 2 == 0, "input length must be even");

        let mut tmp: Vec<u8> = Vec::new();
        let mut buff: u8 = 0;
        let mut reg: u8;

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
        tmp
    }

    pub fn map_u8_to_hex(input: Vec<u8>) -> String {
        let mut tmp: String = String::new();
        let mask: u8 = 0b1111_0000;
        let mut buff: u8 = 0;

        for (i, c) in input.into_iter()
                            .enumerate() {

            buff = mask & c;

            tmp.push(match buff {
               0b0001_0000 => '1',
               0b0010_0000 => '2',
               0b0011_0000 => '3',
               0b0100_0000 => '4',
               0b0101_0000 => '5',
               0b0110_0000 => '6',
               0b0111_0000 => '7',
               0b1000_0000 => '8',
               0b1001_0000 => '9',
               0b1010_0000 => 'A',
               0b1011_0000 => 'B',
               0b1100_0000 => 'C',
               0b1101_0000 => 'D',
               0b1110_0000 => 'E',
               0b1111_0000 => 'F',
               _ => '0'
            });

            buff = 0;
            buff = mask & (c << 4);

            tmp.push(match buff {
               0b0001_0000 => '1',
               0b0010_0000 => '2',
               0b0011_0000 => '3',
               0b0100_0000 => '4',
               0b0101_0000 => '5',
               0b0110_0000 => '6',
               0b0111_0000 => '7',
               0b1000_0000 => '8',
               0b1001_0000 => '9',
               0b1010_0000 => 'A',
               0b1011_0000 => 'B',
               0b1100_0000 => 'C',
               0b1101_0000 => 'D',
               0b1110_0000 => 'E',
               0b1111_0000 => 'F',
               _ => '0'
            });
        }
        tmp    
    }

    pub fn map_hex_to_b64(arr: Vec<u8>, pad: usize) -> String {

        let mut buff = String::new();

        for i in arr.iter()
                    .take(arr.len() - pad) {

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

        for _ in 0..pad {
           buff.push('=');
        }

        buff 
    }
}

pub mod oper {

    pub fn xor(a: String, b: String) -> String {
        assert!(a.len() == b.len(), "input values must be same length");
        let arrA = super::conv::map_string_to_hex(a);
        let arrB = super::conv::map_string_to_hex(b);

        let mut buff = Vec::new();

        for c1 in arrA.iter().zip(arrB.iter()) {
            buff.push(c1.0 ^ c1.1);
        }
        super::conv::map_u8_to_hex(buff)
    }

}
