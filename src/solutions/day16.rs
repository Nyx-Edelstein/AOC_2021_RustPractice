struct BitStream
{
    raw_data : String
}

impl BitStream
{
    fn from_hex(input: &str) -> Self
    {
        let raw_data = input.chars().fold(String::new(), |s, c| s + &format!("{:04b}", c.to_digit(16).unwrap()));
        BitStream { raw_data }
    }

    fn any(&self) -> bool
    {
        !self.raw_data.is_empty() && !self.raw_data.chars().all(|c| c == '0')
    }

    fn read(&mut self, num_bits: usize) -> String
    {
        let raw_data = self.raw_data.clone();
        self.raw_data = raw_data[num_bits..].to_owned();
        raw_data[..num_bits].to_owned()
    }

    fn read_as_bin(&mut self, num_bits: usize) -> u64
    {
        let raw_data = self.raw_data.clone();
        self.raw_data = raw_data[num_bits..].to_owned();
        u64::from_str_radix(&raw_data[..num_bits], 2).unwrap()
    }

    fn parse(&mut self) -> Vec<Packet>
    {
        let mut packets: Vec<Packet> = Vec::new();
        while self.any()
        {
            let next = self.parse_next_packet();
            packets.push(next);
        }
        packets
    }

    fn parse_next_packet(&mut self) -> Packet
    {
        let version = self.read_as_bin(3);
        let type_id = self.read_as_bin(3);
        let data = match type_id
        {
            4 => self.parse_literal_data(),
            _ => self.parse_operator_data()
        };
        Packet { version, type_id, data }
    }
    
    fn parse_literal_data(&mut self) -> PacketData
    {
        let mut value_str = String::new();
        loop
        {
            let continue_bit = self.read(1);
            value_str += &self.read(4);
            if continue_bit == "0" { break; }
        }
        let value = u64::from_str_radix(&value_str, 2).unwrap();
        PacketData::Literal(value)
    }

    fn parse_operator_data(&mut self) -> PacketData
    {
        let length_type_id = self.read_as_bin(1);
        let packets = match length_type_id
        {
            0 => self.parse_from_bit_length(),
            _ => self.parse_from_num_packets()
        };    
        PacketData::Subpackets(packets)
    }

    fn parse_from_bit_length(&mut self) -> Vec<Packet>
    {
        let num_bits = self.read_as_bin(15) as usize;
        let data_to_parse = self.read(num_bits);
        let mut substream = BitStream { raw_data: data_to_parse };
        substream.parse()
    }

    fn parse_from_num_packets(&mut self) -> Vec<Packet>
    {
        let num_packets = self.read_as_bin(11) as usize;
        let mut packets = Vec::new();
        while packets.len() < num_packets
        {
            let packet = self.parse_next_packet();
            packets.push(packet);
        }
        packets
    }

}

struct Packet
{
    version: u64,
    type_id: u64,
    data: PacketData
}

enum PacketData
{
    Literal(u64),
    Subpackets(Vec<Packet>)
}

impl Packet
{
    fn evaluate(&self) -> u64
    {
        let values = match &self.data
        {
            PacketData::Subpackets(s) => s.iter().map(|p| p.evaluate()).collect(),
            _ => Vec::new()
        };

        match self.type_id
        {
            4 => match self.data
            {
                PacketData::Literal(value) => value,
                _ => unreachable!("parse error")
            },
            0 => values.into_iter().sum(),
            1 => values.into_iter().product(),
            2 => values.into_iter().min().unwrap(),
            3 => values.into_iter().max().unwrap(),
            5 => (values[0] > values[1]) as u64,
            6 => (values[0] < values[1]) as u64,
            7 => (values[0] == values[1]) as u64,
            _ => unreachable!("invalid input")
        }
    }
}

fn parse(input: &str) -> Vec<Packet>
{
    let mut stream = BitStream::from_hex(input);
    stream.parse()
}

pub fn solution_a(input: &str) -> String
{
    let mut packets = parse(input);
    let mut version_sum = 0;
    while !packets.is_empty()
    {
        version_sum += packets.iter().map(|p| p.version).sum::<u64>();
        packets = packets.into_iter().filter_map(|p|
        {
            match p.data
            {
                PacketData::Subpackets(subpackets) => { Some(subpackets) }
                _ => None
            }
        }).flatten().collect();
    }
    version_sum.to_string()
}

pub fn solution_b(input: &str) -> String
{
    let packets = parse(input);
    packets[0].evaluate().to_string()
}