use aoc_2021::input;
use itertools::Itertools;

fn main() {
    let input = input!("16");

    let binary = input
        .chars()
        .map(to_binary)
        .flat_map(|x| x.chars())
        .map(|x| x == '1')
        .collect_vec();

    dbg!(&binary);

    let mut reader = PacketReader::new(&binary);
    let packet = reader.read_packet();
    dbg!(&packet);
    dbg!(packet.sum_versions());
    dbg!(packet.evaluate());
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

#[derive(Debug)]
enum PacketBody {
    Literal(u64),
    Operator(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u64,
    type_id: u64,
    body: PacketBody,
}

impl Packet {
    fn sum_versions(&self) -> u64 {
        let mut sum = self.version;
        if let PacketBody::Operator(x) = &self.body {
            for p in x {
                sum += p.sum_versions();
            }
        }
        sum
    }

    fn evaluate(&self) -> u64 {
        match &self.body {
            PacketBody::Literal(x) => *x,
            PacketBody::Operator(x) => match self.type_id {
                0 => x.iter().map(|x| x.evaluate()).sum(),
                1 => x.iter().map(|x| x.evaluate()).product(),
                2 => x.iter().map(|x| x.evaluate()).min().unwrap(),
                3 => x.iter().map(|x| x.evaluate()).max().unwrap(),
                5 => {
                    let (a, b) = x.iter().map(|x| x.evaluate()).collect_tuple().unwrap();
                    if a > b {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    let (a, b) = x.iter().map(|x| x.evaluate()).collect_tuple().unwrap();
                    if a < b {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    let (a, b) = x.iter().map(|x| x.evaluate()).collect_tuple().unwrap();
                    if a == b {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

struct PacketReader<'a> {
    data: &'a [bool],
    pos: usize,
}

impl<'a> PacketReader<'a> {
    fn new(data: &'a [bool]) -> Self {
        Self { data, pos: 0 }
    }

    fn read_n(&mut self, n: usize) -> &'a [bool] {
        let data = &self.data[self.pos..(self.pos + n)];
        self.pos += n;
        data
    }

    fn read_number(&mut self, n: usize) -> u64 {
        bits_to_num(self.read_n(n))
    }

    fn read_header(&mut self) -> (u64, u64) {
        let version = self.read_number(3);
        let type_id = self.read_number(3);
        (version, type_id)
    }

    fn read_literal_part(&mut self) -> (bool, [bool; 4]) {
        let should_continue = self.read_n(1)[0];
        let value = self.read_n(4);
        (should_continue, value.try_into().unwrap())
    }

    fn read_literal_body(&mut self) -> u64 {
        let mut go = true;
        let mut val = 0;
        while go {
            let (should_continue, value) = self.read_literal_part();
            val *= 16;
            val += bits_to_num(&value);
            go = should_continue;
        }
        val
    }

    fn read_packet(&mut self) -> Packet {
        let (version, type_id) = self.read_header();
        let body = match type_id {
            4 => PacketBody::Literal(self.read_literal_body()),
            _ => {
                let length_type_id = self.read_n(1)[0];
                let num = match length_type_id {
                    false => self.read_number(15),
                    true => self.read_number(11),
                };
                let data = match length_type_id {
                    false => self.read_packets_until_len(num as _),
                    true => self.read_n_packets(num as _),
                };
                PacketBody::Operator(data)
            }
        };
        Packet {
            version,
            type_id,
            body,
        }
    }

    fn read_n_packets(&mut self, n: usize) -> Vec<Packet> {
        let mut result = Vec::new();
        for _ in 0..n {
            result.push(self.read_packet());
        }
        result
    }

    fn read_packets_until_len(&mut self, len: usize) -> Vec<Packet> {
        let target = self.pos + len;
        let mut result = Vec::new();
        while self.pos < target {
            result.push(self.read_packet());
        }
        result
    }
}

fn bits_to_num(bits: &[bool]) -> u64 {
    bits.iter()
        .copied()
        .rev()
        .enumerate()
        .map(|(i, bit)| if bit { (2u64).pow(i as _) } else { 0 })
        .sum()
}
