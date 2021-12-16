use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Operator {
    pub version: u8,
    pub type_id: u8,
    pub values: Vec<Packet>,
}

#[derive(Debug, PartialEq)]
pub struct Literal {
    pub version: u8,
    pub type_id: u8,
    pub value: u8,
}

#[derive(Debug, PartialEq)]
enum Packet {
    Operator(Operator),
    Literal(Literal),
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;
  
    Ok((input, Color { red, green, blue }))
  }

pub fn main() {
    assert_eq!(
        parse("D2FE28"),
        Packet::Literal(Literal {
            version: 6,
            type_id: 4,
            value: 2021
        })
    );

    // assert_eq!(
    //     parse("38006F45291200"),
    //     Packet::Operator(Operator {
    //         version: 1,
    //         type_id: 6,
    //         values: vec![10, 20]
    //     })
    // );
}
