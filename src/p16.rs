use std::fmt;
use colored::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    version: i64,
    value: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operator {
    version: i64,
    operator: i64,
    values: Vec<PacketTypes>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PacketTypes {
    Literal(Literal),
    Operator(Operator),
}

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

type Bits = Vec<u8>;

fn bits_to_int(bs: Bits) -> i64 {
    let mut int = 0;
    let base: i64 = 2;
    for (i, b) in bs.iter().rev().enumerate() {
        int += base.pow(i as u32) * (*b as i64);
    }
    return int;
}

fn parse_literal(mut bits: Bits) -> (Bits, i64) {
    let mut value: Bits = vec![];
    loop {
        let next: Bits = bits.drain(0..5).collect();
        println!("Next part of literal: {:?}", next.clone());
        value.append(&mut next[1..5].to_vec());
        if next[0] == 0 {
            break;
        }
    }
    let result = bits_to_int(value);
    println!("Finished parsing as literal result: {}", result);
    return (bits, result);
}

fn parse_values(mut bits: Bits) -> (Bits, Vec<PacketTypes>) {
    let length_id: Bits = bits.drain(0..1).collect();
    if length_id[0] == 0 {
        println!("Parsing packet as bytes");
        let length: Bits = bits.drain(0..15).collect();
        let l = bits_to_int(length.clone()) as usize;
        println!("Taking {} bytes: {:?}", l, length);
        let mut drained: Bits = bits.drain(0..l).collect();
        let mut packets: Vec<PacketTypes> = vec![];
        while drained.len() != 0 && !drained.iter().all(|x| *x == 0) {
            let (new_bits, sub_packet) = parse_packet(drained.clone());
            packets.push(sub_packet);
            drained = new_bits;
        }
        println!("Finished parsing as byte\n");
        return (bits, packets);
    } else if length_id[0] == 1 {
        println!("Parsing as sub_packets");
        let length: Bits = bits.drain(0..11).collect();
        let number_packets = bits_to_int(length.clone()) as usize;
        println!("Parsing as {} sub_packets, {:?}", number_packets, length);
        let mut packets: Vec<PacketTypes> = vec![];
        for _ in 0..number_packets {
            let (new_bits, sub_packet) = parse_packet(bits);
            packets.push(sub_packet);
            bits = new_bits;
        }
        println!("Finished parsing as sub_packet\n");
        return (bits, packets);
    } else {
        unimplemented!();
    }
}

fn parse_packet(mut bits: Bits) -> (Bits, PacketTypes) {
    println!("Parsing into a packet: {:?}", bits);
    let version_bits: Bits = bits.drain(0..3).collect();
    let version = bits_to_int(version_bits.clone());
    println!("{} w/ bits: {} {:?}", "Version".green(), version, version_bits);

    let type_bits: Bits = bits.drain(0..3).collect();
    let ttype = bits_to_int(type_bits.clone());
    println!("Type w/ bits: {} {:?}", ttype, type_bits);

    let result = match ttype {
        4 => {
            println!("Parsing as a literal: {:?}", bits.clone());
            let (new_bits, value) = parse_literal(bits.clone());
            bits = new_bits;
            PacketTypes::Literal(Literal {
                version: version,
                value: value,
            })
        }
        _ => {
            println!("Parsing as a operator: {:?}", bits.clone());
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

pub fn parse(s: &str) -> PacketTypes {
    dbg!(s);
    let bits: Bits = str_to_bits(s);
    println!("Starting top parse of bits: {:?}", &bits);
    let (new_bits, packet) = parse_packet(bits.clone());
    assert!(new_bits.iter().all(|x| { *x == 0 }));
    println!("Done! \n");
    return packet;
}

pub fn sum(p: PacketTypes) -> i64 {
    match p {
        PacketTypes::Literal(Literal{version, value: _}) => version,
        PacketTypes::Operator(Operator{version, operator: _, values}) => {
            version + values.iter().fold(0, |acc, v| {
                acc + sum(v.clone())
            })
        }
    }
}


// Packets with type ID 0 are sum packets - their value is the sum of the values of their sub-packets. If they only have a single sub-packet, their value is the value of the sub-packet.
// Packets with type ID 1 are product packets - their value is the result of multiplying together the values of their sub-packets. If they only have a single sub-packet, their value is the value of the sub-packet.
// Packets with type ID 2 are minimum packets - their value is the minimum of the values of their sub-packets.
// Packets with type ID 3 are maximum packets - their value is the maximum of the values of their sub-packets.
// Packets with type ID 5 are greater than packets - their value is 1 if the value of the first sub-packet is greater than the value of the second sub-packet; otherwise, their value is 0. These packets always have exactly two sub-packets.
// Packets with type ID 6 are less than packets - their value is 1 if the value of the first sub-packet is less than the value of the second sub-packet; otherwise, their value is 0. These packets always have exactly two sub-packets.
// Packets with type ID 7 are equal to packets - their value is 1 if the value of the first sub-packet is equal to the value of the second sub-packet; otherwise, their value is 0. These packets always have exactly two sub-packets.

pub fn eval(p: PacketTypes) -> i64 {
    if let PacketTypes::Literal(Literal{value, version}) = p{
        return value;
    }
    else if let PacketTypes::Operator(Operator{values, version, operator}) =p {
        let vs: Vec<i64> = values.iter().map(|x| {eval(x.clone())}).collect();
        match operator {
            0 => vs.iter().fold(0, |acc, x| {acc+x}),
            1 => vs.iter().fold(1, |acc, x| {acc*x}),
            2 => vs.iter().fold(1000000000, |acc, x| {std::cmp::min(acc,*x)}),
            3 => vs.iter().fold(0, |acc, x| {std::cmp::max(acc,*x)}),
            5 => {
                if vs[0] > vs[1] {
                    1
                }
                else {
                    0
                }
            },
            6 => {
                if vs[0] < vs[1] {
                    1
                }
                else {
                    0
                }
            },
            7 => {
                if vs[0] == vs[1] {
                    1
                }
                else {
                    0
                }
            }
            _ => 0
        }
    }
    else {
        unreachable!();
    }
}
pub fn evals(s: &str) -> i64 {
    let p: PacketTypes = parse(s);
    return eval(p);
}
pub fn main() {
    let d = parse("D2FE28");
    assert_eq!(
        d,
        PacketTypes::Literal(Literal {
            version: 6,
            value: 2021
        })
    );
    assert_eq!(sum(d), 6);

    let t = parse("38006F45291200");
    assert_eq!(
        t,
        PacketTypes::Operator(Operator {
            version: 1,
            operator: 6,
            values: vec![
                PacketTypes::Literal(Literal {
                    version: 6,
                    value: 10
                }),
                PacketTypes::Literal(Literal {
                    version: 2,
                    value: 20
                })
            ]
        })
    );
    assert_eq!(sum(t),9);

    let e = parse("EE00D40C823060");
    assert_eq!(
        e,
        PacketTypes::Operator(Operator {
            version: 7,
            operator: 3,
            values: vec![
                PacketTypes::Literal(Literal {
                    version: 2,
                    value: 1
                }),
                PacketTypes::Literal(Literal {
                    version: 4,
                    value: 2
                }),
                PacketTypes::Literal(Literal {
                    version: 1,
                    value: 3
                })
            ]
        })
    );
    assert_eq!(sum(e),14);

    assert_eq!(16, sum(parse("8A004A801A8002F478")));
    assert_eq!(12, sum(parse("620080001611562C8802118E34")));
    assert_eq!(23, sum(parse("C0015000016115A2E0802F182340")));
    assert_eq!(31, sum(parse("A0016C880162017C3686B18A3D4780")));
    let p = "C20D7900A012FB9DA43BA00B080310CE3643A0004362BC1B856E0144D234F43590698FF31D249F87B8BF1AD402389D29BA6ED6DCDEE59E6515880258E0040A7136712672454401A84CE65023D004E6A35E914BF744E4026BF006AA0008742985717440188AD0CE334D7700A4012D4D3AE002532F2349469100708010E8AD1020A10021B0623144A20042E18C5D88E6009CF42D972B004A633A6398CE9848039893F0650048D231EFE71E09CB4B4D4A00643E200816507A48D244A2659880C3F602E2080ADA700340099D0023AC400C30038C00C50025C00C6015AD004B95002C400A10038C00A30039C0086002B256294E0124FC47A0FC88ACE953802F2936C965D3005AC01792A2A4AC69C8C8CA49625B92B1D980553EE5287B3C9338D13C74402770803D06216C2A100760944D8200008545C8FB1EC80185945D9868913097CAB90010D382CA00E4739EDF7A2935FEB68802525D1794299199E100647253CE53A8017C9CF6B8573AB24008148804BB8100AA760088803F04E244480004323BC5C88F29C96318A2EA00829319856AD328C5394F599E7612789BC1DB000B90A480371993EA0090A4E35D45F24E35D45E8402E9D87FFE0D9C97ED2AF6C0D281F2CAF22F60014CC9F7B71098DFD025A3059200C8F801F094AB74D72FD870DE616A2E9802F800FACACA68B270A7F01F2B8A6FD6035004E054B1310064F28F1C00F9CFC775E87CF52ADC600AE003E32965D98A52969AF48F9E0C0179C8FE25D40149CC46C4F2FB97BF5A62ECE6008D0066A200D4538D911C401A87304E0B4E321005033A77800AB4EC1227609508A5F188691E3047830053401600043E2044E8AE0008443F84F1CE6B3F133005300101924B924899D1C0804B3B61D9AB479387651209AA7F3BC4A77DA6C519B9F2D75100017E1AB803F257895CBE3E2F3FDE014ABC";
    assert_eq!(960, sum(parse(p)));

    assert_eq!(3, evals("C200B40A82"));
    assert_eq!(54, evals("04005AC33890"));
    assert_eq!(7, evals("880086C3E88112"));
    assert_eq!(9, evals("CE00C43D881120"));
    assert_eq!(1, evals("D8005AC2A8F0"));
    assert_eq!(0, evals("F600BC2D8F"));
    assert_eq!(0, evals("9C005AC2F8F0"));
    assert_eq!(1, evals("9C0141080250320F1802104A08"));

    assert_eq!(12301926782560, evals(p));
}
