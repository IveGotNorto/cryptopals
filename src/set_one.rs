#[allow(dead_code)]
pub mod challenge_one {

    use std::convert::TryInto;

    pub fn hex_to_b64(input: String) -> String {
        let (arr, pad) = get_hex_indices(input);
        map_u8s_to_b64(arr, pad)
    }

    // might want to change a bunch of these to usize instead
    fn get_hex_indices(input: String) -> (Vec<u8>, usize) {

        let mut hex_bytes = hex_string_to_bytes(input);
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
                buff |= *iter.next().unwrap() as u32;
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

    pub fn hex_string_to_bytes(mut input: String) -> Vec<u8> {

        let mut pad_add = false;

        let mut tmp: Vec<u8> = Vec::new();
        let mut buff: u8 = 0;
        let mut reg: u8;

        if input.len() % 2 != 0 {
            input.push('_');
            pad_add = true;
        }
        
        for (i, c) in input.to_uppercase()
                            .chars()
                            .into_iter()
                            .enumerate() {
            reg = match c {
               '1' => 0b0000_0001,
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

        if pad_add {
            tmp.pop();
        }

        tmp
    }

    

    pub fn map_u8s_to_b64(arr: Vec<u8>, pad: usize) -> String {

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

    #[test]
    fn convert_hex_to_base64() {

        let array = [ 
            // input, output
            (
                "f1b8a5b80d41b5d4e42d7f92921c7745", 
                "8biluA1BtdTkLX+Skhx3RQ=="
            ),
            (
                "64f8cf9dbe5fd854730e884b3c2a0dba263a103a1674ce7b", 
                "ZPjPnb5f2FRzDohLPCoNuiY6EDoWdM57"
            ),
            (
                "923070ebba14801b03ed32e8b57ec1608467eadc7e576827091997121cfa9ce2", 
                "kjBw67oUgBsD7TLotX7BYIRn6tx+V2gnCRmXEhz6nOI="
            ),
            
            (
                "4919a6f87d95023c252b5c6267e3b4115bc9c6fb5587490da76ab2b92c02ade14740e9f56ebdd1d7b8df30dde96a", 
                "SRmm+H2VAjwlK1xiZ+O0EVvJxvtVh0kNp2qyuSwCreFHQOn1br3R17jfMN3pag=="
            ),
            (
                "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d", 
                "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
            ),
            // The actual problem solution
            (
                "c057388642050a02f3bb9e95d7baa0b112948770cdbd8692fbe1106afbb89511adf2db9dcd85bcaa497b754ca400d09c6c7457b937ba97db242962c6c378ae53", 
                "wFc4hkIFCgLzu56V17qgsRKUh3DNvYaS++EQavu4lRGt8tudzYW8qkl7dUykANCcbHRXuTe6l9skKWLGw3iuUw=="
            ),

        ];

        for item in array.iter() {
            assert_eq!(hex_to_b64(item.0.to_string()), item.1.to_string());
        }
    }
}

#[allow(dead_code)]
pub mod challenge_two {

    pub fn xor_same_length(a: String, b: String) -> String {
        assert!(a.len() == b.len(), "input values must be same length");
        let arr_a = super::challenge_one::hex_string_to_bytes(a);
        let arr_b = super::challenge_one::hex_string_to_bytes(b);

        let mut buff = Vec::new();

        for c1 in arr_a.iter().zip(arr_b.iter()) {
            buff.push(c1.0 ^ c1.1);
        }
        u8_to_hex(buff)
    }

    pub fn u8_to_hex(input: Vec<u8>) -> String {
        let mut tmp: String = String::new();
        let mask: u8 = 0b1111_0000;
        let mut buff: u8;

        for (_, c) in input.into_iter().enumerate() {

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
               0b1010_0000 => 'a',
               0b1011_0000 => 'b',
               0b1100_0000 => 'c',
               0b1101_0000 => 'd',
               0b1110_0000 => 'e',
               0b1111_0000 => 'f',
               _ => '0'
            });

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
               0b1010_0000 => 'a',
               0b1011_0000 => 'b',
               0b1100_0000 => 'c',
               0b1101_0000 => 'd',
               0b1110_0000 => 'e',
               0b1111_0000 => 'f',
               _ => '0'
            });
        }
        tmp    
    }

    #[test]
    fn fixed_xor() {

        let array = [ 
            // input, input, output
            (
                "1c0111001f010100061a024b53535009181c", 
                "686974207468652062756c6c277320657965",
                "746865206b696420646f6e277420706c6179",
            ),
            (
                "8278c9d86a6ccaa1", 
                "10b06ee3b72508c5",
                "92c8a73bdd49c264",
            ),
            (
                "40bf34d48ba83a7a503f205381cd48b0a2fcecf53c0f364527afab85dec4ce0b", 
                "33aaa5c1028fe0d5172bbc642cc659349bcf0c5f3de0305614739ff58bb2a764",
                "731591158927daaf47149c37ad0b11843933e0aa01ef061333dc34705576696f",
            ),

        ];

        for item in array.iter() {
            assert_eq!(xor_same_length(item.0.to_string(), item.1.to_string()), item.2.to_string());
        }
    }
}

#[allow(dead_code)]
pub mod challenge_three {

    use std::collections::HashMap;
    use std::cmp::Ordering;

    pub fn decrypt_hex_string(input: String) -> String {
        let hex = super::challenge_one::hex_string_to_bytes(input);

        let mut possibles = Vec::new();
        let mut buff = Vec::new();

        for c in 0u8..255 {
            buff.clear();
            for d in hex.iter() {
                buff.push(c ^ d);
            }
            possibles.push(u8_to_ascii(buff.to_owned()).to_uppercase());
        }

        get_best_score(possibles)
    }

    pub fn get_best_score(texts: Vec<String>) -> String {

        let mut scores = Vec::new();

        for p in texts.clone() {

            let length = p.len();
            let mut score = 0.0;
            let mut totals = HashMap::new();
            let mut percs = HashMap::new();

            for c in p.chars() {
                match totals.get(&c) {
                    Some(value) => totals.insert(c, value + 1),
                    None => totals.insert(c, 1)
                };
            }

            for (key, value) in totals.into_iter() {
               percs.insert(key, (value / length * 100) as f32);
            }

            for (key, value) in percs.into_iter() {
                score += (char_to_freq(key) - value).abs();
            }

            //println!("duh score: {}", score);

            scores.push(score);
        }

        let lowest_index = scores
                            .iter()
                            .enumerate()
                            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                            .map(|(index, _)| index);

        //println!("index: {}", lowest_index.unwrap());

        texts[lowest_index.unwrap()].to_string()
    }

    pub fn u8_to_ascii(chars: Vec<u8>) -> String {
        let mut buff = String::new();
        for c in chars {
            buff.push(c as char)
        }
        buff
    }

    fn char_to_freq(c: char) -> f32 {
        match c {
            'A' => 8.55,
            'B' => 1.60,
            'C' => 3.16,
            'D' => 3.87,
            'E' => 12.10,
            'F' => 2.18,
            'G' => 2.09,
            'H' => 4.96,
            'I' => 7.33,
            'J' => 0.22,
            'K' => 0.81,
            'L' => 4.21,
            'M' => 2.53,
            'N' => 7.17,
            'O' => 7.47,
            'P' => 2.07,
            'Q' => 0.10,
            'R' => 6.33,
            'S' => 6.73,
            'T' => 8.94,
            'U' => 2.68,
            'V' => 1.06,
            'W' => 1.83,
            'X' => 0.19,
            'Y' => 1.72,
            'Z' => 0.11,
            '\'' => 0.11,
            ' ' => 10.00,
            _ => -100.00 
        }
    }


    #[test]
    fn single_byte_xor_cipher() {
        let array = [ 
            (
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
                "COOKING MC'S LIKE A POUND OF BACON"
            )
        ];

        for item in array.iter() {
            assert_eq!(decrypt_hex_string(item.0.to_string()), item.1);
        }
    }
}

#[allow(dead_code)]
pub mod challenge_four {

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn decrypt_hex_strings() -> String {
        let file = File::open("src/data/section_1_challenge_4.txt").unwrap();
        let reader = BufReader::new(file);

        let mut possibles = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap(); 
            possibles.push(super::challenge_three::decrypt_hex_string(line));
        }

        super::challenge_three::get_best_score(possibles)
    }

    #[test]
    fn detect_single_character_xor() {
        assert_eq!(decrypt_hex_strings(), "NOW THAT THE PARTY IS JUMPING\n");
    }

}

#[allow(dead_code)]
pub mod challenge_five {

    fn encrypt_ascii(key: String, text: String) -> String {
        let mut buff = Vec::new();

        let text_hex = text.as_bytes();
        let key_hex = key.as_bytes();

        let rot = key_hex.len();

        for (i, byte) in text_hex.iter().enumerate() {
            buff.push(key_hex[i % rot] ^ byte);
        }
        
        super::challenge_two::u8_to_hex(buff)
    }

    #[test]
    fn implement_repeating_key_xor() {
        let array = [
            // Key, Input, Output
            (
                "ICE",
                "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
                "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
            )
        ];

        for item in array.iter() {
            assert_eq!(encrypt_ascii(item.0.to_string(), item.1.to_string()), item.2.to_string());
        }
    }

}
