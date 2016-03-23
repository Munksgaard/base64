static BASE64_CHARS: &'static[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                      abcdefghijklmnopqrstuvwxyz\
                                      0123456789+/";

pub fn encode(v: &[u8]) -> Vec<u8> {
    let len = v.len();

    let mut split: Vec<u8> = Vec::new();

    for i in 0..(len * 4 / 3 + ((len % 3 != 0) as usize)) {
        let index = i / 4 * 3;
        match i % 4 {
            0 => split.push(v[index] >> 2),
            1 =>
                if index+1 < len {
                    split.push(((v[index] & 3) << 4) + (v[index + 1] >> 4))
                } else {
                    split.push((v[index] & 3) << 4)
                },
            2 => {
                if index+2 < len {
                    split.push(((v[index + 1] & 15) << 2) + (v[index + 2] >> 6))
                } else {
                    split.push((v[index + 1] & 15) << 2)
                }},
            3 => {
                split.push(v[index + 2] & 63)},
            _ => panic!("Impossible!")
            }
    }

    let mut encoded: Vec<u8> = split.iter().map(|x| BASE64_CHARS[*x as usize]).collect();

    let elen = encoded.len();

    if elen % 4 != 0 {
        let len = encoded.len();
        encoded.resize(len + 4 - elen % 4, b'=');
    }

    encoded
}

pub fn decode(v: &[u8]) -> Vec<u8> {
    let trans: Vec<u8> = v.iter()
                 .take_while(|&x| { x != &('=' as u8) })
                 .map(|x| { BASE64_CHARS.iter().position(|c| c == x).unwrap() as u8 })
                 .collect();

    let mut res: Vec<u8> = Vec::new();

    for i in 0..trans.len() {
        match i % 4 {
            0 => res.push(trans[i] << 2),
            1 => { *res.last_mut().unwrap() = *res.last().unwrap() + (trans[i] >> 4);
                   res.push(trans[i] << 4) },
            2 => { *res.last_mut().unwrap() = *res.last().unwrap() + (trans[i] >> 2);
                   res.push(trans[i] << 6) },
            3 => *res.last_mut().unwrap() = *res.last().unwrap() + trans[i],
            _ => panic!("Impossible")
        }
    }

    res
}

#[cfg(test)]
mod test {
    use super::encode;
    use super::decode;

    #[test]
    fn encode_test() {
        let input: &[u8] = b"Man";
        let output: &[u8] = b"TWFu";

        assert_eq!(encode(input).as_slice(), output);
    }

    #[test]
    fn decode_test() {
        let input: &[u8] = b"TWFu";
        let output: &[u8] = b"Man";

        assert_eq!(output, decode(input).as_slice());
    }
}
