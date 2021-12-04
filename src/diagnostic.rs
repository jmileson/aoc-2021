use crate::prelude::*;
use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Default)]
struct BitCounts(i32, i32);

impl Add for BitCounts {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for BitCounts {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1);
    }
}

impl BitCounts {
    fn most_common(&self) -> char {
        if self.0 > self.1 {
            '0'
        } else {
            '1'
        }
    }

    fn least_common(&self) -> char {
        if self.0 <= self.1 {
            '0'
        } else {
            '1'
        }
    }
}

pub(crate) async fn eval(input: Vec<String>) -> Result<()> {
    let chars = input
        .iter()
        .map(|i| i.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let counts = count_bits(&chars)?;

    let gamma_ = gamma(&counts)?;
    let epsilon_ = epsilon(&counts)?;

    let power_consumption = gamma_ * epsilon_;
    
    let oxy_rating = oxygen_generator_rating(chars.clone())?;
    let co2_rating = co2_scrubber_rating(chars)?;
    
    let life_support_rating = oxy_rating * co2_rating;

    println!("power consumption: {:#?}", power_consumption);
    println!("life support rating: {:#?}", life_support_rating);

    Ok(())
}

fn count_bits(input: &Vec<Vec<char>>) -> Result<Vec<BitCounts>> {
    let num_bits = input[0].len();
    let mut counts = vec![BitCounts::default(); num_bits];
    for num in input.iter() {
        for (k, bit) in num.iter().enumerate() {
            let bc = match bit {
                '0' => Ok(BitCounts(1, 0)),
                '1' => Ok(BitCounts(0, 1)),
                _ => Err(anyhow!("unrecognized bit")),
            }?;
            counts[k] += bc;
        }
    }

    Ok(counts)
}

fn bits_to_i32(bits: &str) -> Result<i32> {
    i32::from_str_radix(&bits, 2).map_err(anyhow::Error::msg)
}
fn gamma(counts: &Vec<BitCounts>) -> Result<i32> {
    let bits: String = counts.iter().map(|c| c.most_common()).collect();
    bits_to_i32(&bits)
}

fn epsilon(counts: &Vec<BitCounts>) -> Result<i32> {
    let bits: String = counts.iter().map(|c| c.least_common()).collect();
    bits_to_i32(&bits)
}

fn oxygen_generator_rating(mut chars: Vec<Vec<char>>) -> Result<i32> {
    let mut i = 0;
    loop {
        if chars.len() == 1 {
            break;
        }
        let counts = count_bits(&chars)?;
        let bc = counts[i];
        let most_common = bc.most_common();
        chars.drain_filter(|cs| cs[i] != most_common);
        i += 1;
    }

    bits_to_i32(&chars[0].iter().collect::<String>())
}

fn co2_scrubber_rating(mut chars: Vec<Vec<char>>) -> Result<i32> {
    let mut i = 0;
    loop {
        if chars.len() == 1 {
            break;
        }
        let counts = count_bits(&chars)?;
        let bc = counts[i];
        let least_common = bc.least_common();
        chars.drain_filter(|cs| cs[i] != least_common);
        i += 1;
    }

    bits_to_i32(&chars[0].iter().collect::<String>())
}

#[cfg(test)]
mod test {

    use super::{co2_scrubber_rating, count_bits, epsilon, gamma, oxygen_generator_rating};

    fn inputs() -> Vec<Vec<char>> {
        vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ]
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect()
    }

    #[test]
    fn test_bit_counts() {
        let input = inputs();

        let counts = count_bits(&input);
        assert!(counts.is_ok());
    }

    #[test]
    fn test_gamma() {
        let input = inputs();
        let counts = count_bits(&input).unwrap();
        let gamma_ = gamma(&counts);
        assert!(gamma_.is_ok());
        assert_eq!(22, gamma_.unwrap());
    }

    #[test]
    fn test_epsilon() {
        let input = inputs();
        let counts = count_bits(&input).unwrap();
        let epsilon_ = epsilon(&counts);
        assert!(epsilon_.is_ok());
        assert_eq!(9, epsilon_.unwrap());
    }

    #[test]
    fn test_oxygen_generator() {
        let input = inputs();

        let oxy_rating = oxygen_generator_rating(input.clone());
        assert!(oxy_rating.is_ok());
        let oxy_rating = oxy_rating.unwrap();
        assert_eq!(23, oxy_rating);
    }
    #[test]
    fn test_co2_scrubber() {
        let input = inputs();

        let co2_rating = co2_scrubber_rating(input.clone());
        assert!(co2_rating.is_ok());
        let co2_rating = co2_rating.unwrap();
        assert_eq!(10, co2_rating);
    }
}
