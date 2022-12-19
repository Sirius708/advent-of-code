use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter::once;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::map;
use nom::multi::{many1, separated_list0, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Packet(Vec<PacketData>);

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|item| format!("{item}"))
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum PacketData {
    Integer(u32),
    List(Vec<PacketData>),
}

impl Display for PacketData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketData::Integer(x) => write!(f, "{x}"),
            PacketData::List(list) => {
                write!(
                    f,
                    "[{}]",
                    list.iter()
                        .map(|item| format!("{item}"))
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }
        }
    }
}

fn main() {
    let input = util::get_input_string();
    let packets = parse_packets(&input).unwrap().1;

    let mut right_order_sum = 0;
    for (i, (packet_1, packet_2)) in packets.iter().enumerate() {
        if is_packet_data_in_order(&packet_1.0, &packet_2.0).unwrap() {
            right_order_sum += i + 1;
        }
    }
    println!("Right order sum: {}", right_order_sum);

    let divider_packet_1 = Packet(vec![PacketData::List(vec![PacketData::Integer(2)])]);
    let divider_packet_2 = Packet(vec![PacketData::List(vec![PacketData::Integer(6)])]);

    let mut packets = packets
        .into_iter()
        .flat_map(|(p1, p2)| [p1, p2])
        .chain(once(divider_packet_1.clone()))
        .chain(once(divider_packet_2.clone()))
        .collect::<Vec<_>>();
    packets.sort_unstable_by(|a, b| {
        if is_packet_data_in_order(&a.0, &b.0).unwrap() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let divider_1_index = packets
        .iter()
        .position(|packet| *packet == divider_packet_1)
        .unwrap()
        + 1;
    let divider_2_index = packets
        .iter()
        .position(|packet| *packet == divider_packet_2)
        .unwrap()
        + 1;
    println!("Decoder key: {}", divider_1_index * divider_2_index);
}

fn parse_packets(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    let (input, packets) = separated_list1(many1(newline), parse_packet_pair)(input)?;
    Ok((input, packets))
}

fn parse_packet_pair(input: &str) -> IResult<&str, (Packet, Packet)> {
    let (input, packet_pair) = separated_pair(parse_packet, newline, parse_packet)(input)?;
    Ok((input, packet_pair))
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    let (input, data) = parse_packet_data(input)?;
    Ok((input, Packet(data)))
}

fn parse_packet_data(input: &str) -> IResult<&str, Vec<PacketData>> {
    let (input, _) = tag("[")(input)?;
    let (input, data) = separated_list0(
        tag(","),
        alt((
            map(parse_packet_data, PacketData::List),
            map(digit1, |int: &str| {
                PacketData::Integer(int.parse().unwrap())
            }),
        )),
    )(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, data))
}

fn is_packet_data_in_order(data_1: &[PacketData], data_2: &[PacketData]) -> Option<bool> {
    if data_1.is_empty() {
        return if data_2.is_empty() { None } else { Some(true) };
    }
    for pair in data_1.iter().zip(data_2.iter()) {
        let result = match pair {
            (PacketData::Integer(a), PacketData::Integer(b)) => match a.cmp(b) {
                Ordering::Less => Some(true),
                Ordering::Equal => None,
                Ordering::Greater => Some(false),
            },
            (PacketData::List(list_1), PacketData::Integer(b)) => {
                is_packet_data_in_order(list_1, &[PacketData::Integer(*b)])
            }
            (PacketData::Integer(a), PacketData::List(list_2)) => {
                is_packet_data_in_order(&[PacketData::Integer(*a)], list_2)
            }
            (PacketData::List(list_1), PacketData::List(list_2)) => {
                is_packet_data_in_order(list_1, list_2)
            }
        };
        if result.is_some() {
            return result;
        }
    }
    match data_1.len().cmp(&data_2.len()) {
        Ordering::Less => Some(true),
        Ordering::Equal => None,
        Ordering::Greater => Some(false),
    }
}
