use anyhow::{anyhow, Result};

pub struct Decoder<'a> {
    buf: &'a [u8],
    pos: usize,
    len: usize,
}

impl<'a> Decoder<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self {
            buf,
            pos: 0,
            len: buf.len() * 8,
        }
    }
    fn read(&mut self, bits: u8) -> Result<u64> {
        let start = self.pos;
        let end = self.pos + bits as usize;
        if end > self.len {
            return Err(anyhow!(
                "not enough bits left -- position: {} length: {} needed: {}",
                self.pos,
                self.len,
                bits
            ));
        }

        let mut value = 0;
        for i in start..end {
            let byte_index = (i / 8) as usize;
            let byte = self.buf[byte_index];
            let shift = 7 - (i % 8);
            let bit = (byte >> shift) as u64 & 1;
            value = (value << 1) | bit;
        }

        self.pos = end;
        Ok(value)
    }
    pub fn skip(&mut self, count: usize) -> Self {
        assert!(count <= self.len - self.pos);
        self.pos += count;
        Self {
            buf: self.buf,
            pos: self.pos - count,
            len: self.pos,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.len
    }
}

pub fn part_one(d: &mut Decoder) -> Result<u64> {
    let mut version = d.read(3)?;
    let type_id = d.read(3)?;

    match type_id {
        4 => {
            let _ = read_num(d)?;
            Ok(version)
        }
        _ => match d.read(1)? {
            0 => {
                let len = d.read(15)?;
                let mut rest = d.skip(len as usize);
                while !rest.is_empty() {
                    version += part_one(&mut rest)?;
                }
                Ok(version)
            }
            1 => {
                let num = d.read(11)?;
                version += (0..num).flat_map(|_| part_one(d)).sum::<u64>();
                Ok(version)
            }
            _ => Err(anyhow!("invalid operator length type id")),
        },
    }
}

fn read_num(d: &mut Decoder) -> Result<Vec<u64>> {
    let mut num = 0;
    loop {
        let next = d.read(1)? == 1;
        let part = d.read(4)?;
        num = (num << 4) | part;
        if !next {
            break;
        }
    }
    Ok(vec![num])
}

pub fn part_two(d: &mut Decoder) -> Result<Vec<u64>> {
    let _version = d.read(3)?;
    let type_id = d.read(3)?;

    match type_id {
        4 => read_num(d),
        _ => {
            let mut nums = vec![];
            match d.read(1)? {
                0 => {
                    let len = d.read(15)?;
                    let mut rest = d.skip(len as usize);
                    while !rest.is_empty() {
                        nums.extend_from_slice(&part_two(&mut rest)?);
                    }
                }
                1 => {
                    let num = d.read(11)?;
                    for _ in 0..num {
                        nums.extend_from_slice(&part_two(d)?);
                    }
                }
                _ => return Err(anyhow!("invalid operator length type id")),
            };

            match type_id {
                0 => Ok(vec![nums.iter().sum()]),
                1 => Ok(vec![nums.iter().product()]),
                2 => Ok(vec![*nums.iter().min().unwrap()]),
                3 => Ok(vec![*nums.iter().max().unwrap()]),
                5 => Ok(vec![(nums[0] > nums[1]) as u64]),
                6 => Ok(vec![(nums[0] < nums[1]) as u64]),
                7 => Ok(vec![(nums[0] == nums[1]) as u64]),
                _ => Err(anyhow!("unknown type id")),
            }
        }
    }
}

fn input(s: &str) -> Vec<u8> {
    let s = s.trim();
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day16_1_test() {
        let input = input("8A004A801A8002F478");
        let mut d = Decoder::new(&input);
        assert_eq!(16, part_one(&mut d).unwrap());
    }
    #[test]
    fn day16_1_test_2() {
        let input = input("620080001611562C8802118E34");
        let mut d = Decoder::new(&input);
        assert_eq!(12, part_one(&mut d).unwrap());
    }
    #[test]
    fn day16_1_test_3() {
        let input = input("C0015000016115A2E0802F182340");
        let mut d = Decoder::new(&input);
        assert_eq!(23, part_one(&mut d).unwrap());
    }
    #[test]
    fn day16_1_test_4() {
        let input = input("A0016C880162017C3686B18A3D4780");
        let mut d = Decoder::new(&input);
        assert_eq!(31, part_one(&mut d).unwrap());
    }
    #[test]
    fn day16_2_test() {
        let input = input("C200B40A82");
        let mut d = Decoder::new(&input);
        assert_eq!(3, part_two(&mut d).unwrap()[0]);
    }
    #[test]
    fn day16_1() {
        dbg!(part_one(&mut Decoder::new(&input(include_str!("../data/day16.txt")))).unwrap());
    }
    #[test]
    fn day16_2() {
        dbg!(part_two(&mut Decoder::new(&input(include_str!("../data/day16.txt")))).unwrap()[0]);
    }
}
