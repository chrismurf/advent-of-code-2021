use std::fs::File;
use bitvec::prelude::*;
use std::io::{self, prelude::*, BufReader};

const NBITS_VERSION: usize = 3;
const NBITS_TYPE_ID: usize = 3;
const NBITS_LITERAL_CHUNK: usize = 4;
const NBITS_LENGTH_BITS: usize = 15;
const NBITS_LENGTH_PACKETS: usize = 11;

const PACKET_TYPE_ID_SUM: u8 = 0;
const PACKET_TYPE_ID_PRODUCT: u8 = 1;
const PACKET_TYPE_ID_MIN: u8 = 2;
const PACKET_TYPE_ID_MAX: u8 = 3;
const PACKET_TYPE_ID_LITERAL: u8 = 4;
const PACKET_TYPE_ID_GT: u8 = 5;
const PACKET_TYPE_ID_LT: u8 = 6;
const PACKET_TYPE_ID_EQ: u8 = 7;

fn bytevec_from_hexstring(hex: &str) -> Vec<u8> {
    (0..hex.len()).step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i+2], 16).unwrap())
        .collect()
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    data: PacketData
}

#[derive(Debug, PartialEq)]
enum PacketData {
    Sum(Vec<Box<Packet>>),
    Product(Vec<Box<Packet>>),
    Min(Vec<Box<Packet>>),
    Max(Vec<Box<Packet>>),
    Literal(u64),
    GreaterThan(Vec<Box<Packet>>),
    LessThan(Vec<Box<Packet>>),
    Equal(Vec<Box<Packet>>),
}

impl Packet {
    fn from_hexstring(hex: &str) -> Self {
        let mut offset: usize = 0;
        Packet::from_bytes(&bytevec_from_hexstring(hex), &mut offset)
    }

    fn from_bytes(bytes: &Vec<u8>, offset: &mut usize) -> Self {
        use PacketData::*;
        let bits = bytes.view_bits::<Msb0>();
        let version: u8 = bits[*offset..*offset+NBITS_VERSION].load_be();
        *offset += NBITS_VERSION;
        let type_id: u8 = bits[*offset..*offset+NBITS_TYPE_ID].load_be();
        *offset += NBITS_TYPE_ID;

        match type_id {
            PACKET_TYPE_ID_LITERAL => {
                let mut value: u64 = 0;
                let mut more_chunks_exist: bool = true;
                while more_chunks_exist {
                    // First bit says whether there are more chunks
                    more_chunks_exist = bits[*offset];
                    *offset += 1;
                    // Now read the chunk, and append to the value
                    let chunk = bits[*offset..*offset+NBITS_LITERAL_CHUNK].load_be::<u8>();
                    *offset += NBITS_LITERAL_CHUNK;
                    value <<= NBITS_LITERAL_CHUNK;
                    value += chunk as u64;
                }
                return Packet { version, data: Literal(value) }
            },
            type_id => { // other operator type
                let mut sub_packets = Vec::new();
                let length_is_packets: bool = bits[*offset];
                *offset += 1;
                if length_is_packets {
                    let packets_left: usize = bits[*offset..*offset+NBITS_LENGTH_PACKETS].load_be();
                    *offset += NBITS_LENGTH_PACKETS;
                    for _ in 0..packets_left {
                        sub_packets.push(Box::new(Self::from_bytes(bytes, offset)));
                    }
                } else { // length_is_bits
                    let length: usize = bits[*offset..*offset+NBITS_LENGTH_BITS].load_be();
                    *offset += NBITS_LENGTH_BITS;
                    let offset_end_subpackets: usize = *offset + length;
                    while *offset < offset_end_subpackets {
                        sub_packets.push(Box::new(Self::from_bytes(bytes, offset)));
                    }
                }

                match type_id {
                    PACKET_TYPE_ID_SUM => Packet { version, data: Sum(sub_packets) },
                    PACKET_TYPE_ID_PRODUCT => Packet { version, data: Product(sub_packets) },
                    PACKET_TYPE_ID_MIN => Packet { version, data: Min(sub_packets) },
                    PACKET_TYPE_ID_MAX => Packet { version, data: Max(sub_packets) },
                    PACKET_TYPE_ID_GT => Packet { version, data: GreaterThan(sub_packets) },
                    PACKET_TYPE_ID_LT => Packet { version, data: LessThan(sub_packets) },
                    PACKET_TYPE_ID_EQ => Packet { version, data: Equal(sub_packets) },
                    t => { unreachable!("Packet type of {} found somehow???", t); }
                }
            }
        }
    }

    fn sum_versions(&self) -> u64 {
        use PacketData::*;
        let mut total: u64 = self.version as u64;
    
        match &self.data {
            Literal(_) => { },
            Sum(packets) |
            Product(packets) |
            Min(packets) |
            Max(packets) |
            GreaterThan(packets) |
            LessThan(packets) |
            Equal(packets) => {
                total += packets.iter()
                    .map(|p| p.sum_versions() )
                    .sum::<u64>()
            },
        }
        total
    }    

    fn evaluate(&self) -> u64 {
        use PacketData::*;
        let mut total: u64 = 0;
    
        match &self.data {
            Literal(value) => {
                total = *value;
            },
            Sum(packets) => {
                total += packets.iter().map(|p| p.evaluate()).sum::<u64>();
            },
            Product(packets) => {
                total += packets.iter().map(|p| p.evaluate()).product::<u64>();
            },
            Min(packets) => {
                total += packets.iter().map(|p| p.evaluate()).min().unwrap();
            },
            Max(packets) => {
                total += packets.iter().map(|p| p.evaluate()).max().unwrap();
            },
            GreaterThan(packets) => {
                if packets[0].evaluate() > packets[1].evaluate() {
                    total += 1;
                }
            },
            LessThan(packets) => {
                if packets[0].evaluate() < packets[1].evaluate() {
                    total += 1;
                }
            },
            Equal(packets) => {
                if packets[0].evaluate() == packets[1].evaluate() {
                    total += 1;
                }
            },
        }
        total
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let hexline = reader.lines().next().unwrap().unwrap();
    let packet = Packet::from_hexstring(hexline.trim());

    println!("Sum of versions is {}", packet.sum_versions());
    assert_eq!(packet.sum_versions(), 943); // Correct result

    println!("Result of computation is {}", packet.evaluate());
    assert_eq!(packet.evaluate(), 167737115857);
    Ok(())
}

#[test]
fn test_packets() {
    use PacketData::*;

    let packet = Packet::from_hexstring("D2FE28");
    assert_eq!(packet.version, 6);
    assert_eq!(packet.data, Literal(2021));

    let packet = Packet::from_hexstring("38006F45291200");
    assert_eq!(packet.version, 1);
    let subpackets = vec![
       Box::new(Packet { version: 6, data: Literal(10) }),
       Box::new(Packet { version: 2, data: Literal(20) }),
    ];
    assert_eq!(packet.data, LessThan(subpackets));

    let packet = Packet::from_hexstring("EE00D40C823060");
    assert_eq!(packet.version, 7);
    let subpackets = vec![
       Box::new(Packet { version: 2, data: Literal(1) }),
       Box::new(Packet { version: 4, data: Literal(2) }),
       Box::new(Packet { version: 1, data: Literal(3) }),
    ];
    assert_eq!(packet.data, Max(subpackets));
}

#[test]
fn test_sum_versions() {
    assert_eq!(Packet::from_hexstring("8A004A801A8002F478").sum_versions(), 16);
    assert_eq!(Packet::from_hexstring("620080001611562C8802118E34").sum_versions(), 12);
    assert_eq!(Packet::from_hexstring("C0015000016115A2E0802F182340").sum_versions(), 23);
    assert_eq!(Packet::from_hexstring("A0016C880162017C3686B18A3D4780").sum_versions(), 31);
}
