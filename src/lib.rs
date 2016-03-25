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
        .take_while(|&x| { *x != b'=' })
        .filter_map(|x| BASE64_CHARS.iter().position(|c| c == x).map(|x| x as u8))
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

    if v.iter().last() == Some(&b'=') {
        res.pop();
    }

    res
}

#[cfg(test)]
mod test {
    extern crate rand;

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

    #[test]
    fn decode_padding_test1() {
        let input: &[u8] = b"Ma";

        let encoded: Vec<u8> = encode(input);

        let decoded: Vec<u8> = decode(&encoded);

        assert_eq!(input, decoded.as_slice());
    }

    #[test]
    fn decode_padding_test2() {
        let input: &[u8] = b"M";

        let encoded: Vec<u8> = encode(input);

        let decoded: Vec<u8> = decode(&encoded);

        assert_eq!(input, decoded.as_slice());
    }

    #[test]
    fn decode_padding_test3() {
        let input: &[u8] = b"Man";

        let encoded: Vec<u8> = encode(input);

        let decoded: Vec<u8> = decode(&encoded);

        assert_eq!(input, decoded.as_slice());
    }

    #[test]
    fn encode_and_decode_test() {
        let input: &[u8] = b"Hej med dig hvad saa? Din mor og saa videre.";

        let encoded: Vec<u8> = encode(input);

        let decoded: Vec<u8> = decode(&encoded);

        assert_eq!(input, decoded.as_slice());
    }

    #[test]
    fn random_test() {
        for _ in 0..20 {
            let n = rand::random::<u8>();

            let mut buf: Vec<u8> = Vec::new();

            for _ in 0..n {
                buf.push(rand::random());
            }

            let encoded = encode(&buf);

            assert_eq!(buf, decode(&encoded));
        }
    }

    #[test]
    fn asl() {
        let input: &[u8] = b"CRIwqt4+szDbqkNY+I0qbNXPg1XLaCM5etQ5Bt9DRFV/xIN2k8Go7jtArLIy";

        let expected: &[u8] = &[0x09, 0x12, 0x30, 0xaa, 0xde, 0x3e, 0xb3, 0x30,
                                0xdb, 0xaa, 0x43, 0x58, 0xf8, 0x8d, 0x2a, 0x6c,
                                0xd5, 0xcf, 0x83, 0x55, 0xcb, 0x68, 0x23, 0x39,
                                0x7a, 0xd4, 0x39, 0x06, 0xdf, 0x43, 0x44, 0x55,
                                0x7f, 0xc4, 0x83, 0x76, 0x93, 0xc1, 0xa8, 0xee,
                                0x3b, 0x40, 0xac, 0xb2, 0x32];

        let decoded = decode(&input);

        assert_eq!(decoded.as_slice(), expected);
    }
}
