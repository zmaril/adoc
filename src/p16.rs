#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    version: i32,
    value: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operator {
    version: i32,
    operator: i32,
    values: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PacketTypes {
    Value(Value),
    Operator(Operator),
}

// 0 = 0000
// 1 = 0001
// 2 = 0010
// 3 = 0011
// 4 = 0100
// 5 = 0101
// 6 = 0110
// 7 = 0111
// 8 = 1000
// 9 = 1001
// A = 1010
// B = 1011
// C = 1100
// D = 1101
// E = 1110
// F = 1111

fn char_to_bits(c: char) -> [u8; 4] {
    match c {
        '0' => [0, 0, 0, 0],
        '1' => [0, 0, 0, 1],
        '2' => [0, 0, 1, 0],
        '3' => [0, 0, 1, 1],
        '4' => [0, 1, 0, 0],
        '5' => [0, 1, 0, 1],
        '6' => [0, 1, 1, 0],
        '7' => [0, 1, 1, 1],
        '8' => [1, 0, 0, 0],
        '9' => [1, 0, 0, 1],
        'A' => [1, 0, 1, 0],
        'B' => [1, 0, 1, 1],
        'C' => [1, 1, 0, 0],
        'D' => [1, 1, 0, 1],
        'E' => [1, 1, 1, 0],
        'F' => [1, 1, 1, 1],
        _ => unreachable!(),
    }
}

fn str_to_bits(s: &str) -> Vec<u8> {
    s.chars().map(char_to_bits).fold(vec![], |mut acc, a| {
        acc.append(&mut a.to_vec());
        acc
    })
}

fn bits_to_int(bs: Vec<u8>) -> i32 {
    let mut int = 0;
    let base: i32 = 2;
    for (i, b) in bs.iter().rev().enumerate() {
        int += base.pow(i as u32) * (*b as i32);
    }
    return int;
}

fn parse_literal(mut bits: Vec<u8>) -> (Vec<u8>, i32) {
    let mut value: Vec<u8> = vec![];
    loop {
        let next: Vec<u8> = bits.drain(0..5).collect();
        value.append(&mut next[1..5].to_vec());
        if next[0] == 0 {
            break;
        }
    }
    let length_so_far = 3 + 3 + 5 * value.len();
    let to_drain = 4 - (length_so_far % 4);
    let drained: Vec<u8> = bits.drain(0..=to_drain).collect();
    assert!(drained.iter().all(|x| { *x == 0 }));
    return (bits, bits_to_int(value));
}

fn parse_values(mut bits: Vec<u8>) -> (Vec<u8>, Vec<i32>) {
    let length_id: Vec<u8> = bits.drain(0..1).collect();
    if length_id[0] == 0 {
        let length: Vec<u8> = bits.drain(0..15).collect();
        let l = bits_to_int(length) as usize;
        let packets: Vec<u8> = bits.drain(0..=l).collect();
        return (bits, vec![10, 20]);
    } else if length_id[0] == 1 {
        let length: Vec<u8> = bits.drain(0..11).collect();
        let l = 3* (bits_to_int(length) as usize);
        let packets: Vec<u8> = bits.drain(0..=l).collect();
        return (bits, vec![10, 2]);
    }
    else {
        unimplemented!();
    }
}

// return bits
fn parse_packet(mut bits: Vec<u8>) -> (Vec<u8>, PacketTypes) {
    let version_bits = bits.drain(0..3).collect();
    let version = bits_to_int(version_bits);

    let type_bits = bits.drain(0..3).collect();
    let ttype = bits_to_int(type_bits);

    let result = match ttype {
        4 => {
            let (new_bits, value) = parse_literal(bits.clone());
            bits = new_bits;
            PacketTypes::Value(Value {
                version: version,
                value: value,
            })
        }
        _ => {
            let (new_bits, values) = parse_values(bits.clone());
            bits = new_bits;
            PacketTypes::Operator(Operator {
                version: version,
                operator: ttype,
                values: values,
            })
        }
    };
    return (bits, result);
}

pub fn parse(s: &str) -> Vec<PacketTypes> {
    dbg!(s);
    let mut bits: Vec<u8> = str_to_bits(s);
    let mut packets: Vec<PacketTypes> = vec![];
    while bits.len() != 0 && !bits.iter().all(|x| { *x == 0}){
        println!("bits: {:?}", &bits);
        let (new_bits, packet) = parse_packet(bits.clone());
        packets.push(packet);
        bits = new_bits;
    }
    return packets;
}

pub fn main() {
    assert_eq!(
        parse("D2FE28")[0],
        PacketTypes::Value(Value {
            version: 6,
            value: 2021
        })
    );

    assert_eq!(
        parse("38006F45291200")[0],
        PacketTypes::Operator(Operator {
            version: 1,
            operator: 6,
            values: vec![10, 20]
        })
    );

    assert_eq!(
        parse("D2FE28D2FE28"),
        [
            PacketTypes::Value(Value {
                version: 6,
                value: 2021
            }),
            PacketTypes::Value(Value {
                version: 6,
                value: 2021
            }),
        ]
    );

    assert_eq!(
        parse("EE00D40C823060"),
        [
            PacketTypes::Operator(Operator {
                version: 1,
                operator: 6,
                values: vec![10, 20]
            }),
            PacketTypes::Operator(Operator {
                version: 1,
                operator: 6,
                values: vec![10, 20]
            })
        ]
    );
}
