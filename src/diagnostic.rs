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
    fn greatest(&self) -> char {
        if self.0 > self.1 {
            '0'
        } else {
            '1'
        }
    }

    fn least(&self) -> char {
        if self.0 < self.1 {
            '0'
        } else {
            '1'
        }
    }
}

pub(crate) async fn eval(input: Vec<String>) -> Result<()> {
    let counts = count_bits(&input)?;

    let gamma_ = gamma(&counts)?;
    let epsilon_ = epsilon(&counts)?;

    let power_consumption = gamma_ * epsilon_;

    println!("power consumption: {:#?}", power_consumption);

    Ok(())
}

fn count_bits(input: &Vec<String>) -> Result<Vec<BitCounts>> {
    let num_bits = input[0].len();
    let mut counts = vec![BitCounts::default(); num_bits];
    for num in input.iter() {
        for (k, bit) in num.chars().enumerate() {
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

fn gamma(counts: &Vec<BitCounts>) -> Result<i32> {
    let bits: String = counts.iter().map(|c| c.greatest()).collect();

    i32::from_str_radix(&bits, 2).map_err(anyhow::Error::msg)
}

fn epsilon(counts: &Vec<BitCounts>) -> Result<i32> {
    let bits: String = counts.iter().map(|c| c.least()).collect();
    i32::from_str_radix(&bits, 2).map_err(anyhow::Error::msg)
}

#[cfg(test)]
mod test {

    use super::{count_bits, epsilon, gamma};

    #[test]
    fn test_bit_counts() {
        let input = vec![
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
        ];

        let counts = count_bits(&input);
        assert!(counts.is_ok());
        let counts = counts.unwrap();

        let gamma_ = gamma(&counts);
        let epsilon_ = epsilon(&counts);
        assert!(gamma_.is_ok());
        assert!(epsilon_.is_ok());
        assert_eq!(22, gamma_.unwrap());
        assert_eq!(9, epsilon_.unwrap());
    }
}
