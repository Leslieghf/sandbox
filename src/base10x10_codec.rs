use num_bigint::BigUint;
use num_traits::Zero;

pub struct Base10x10Codec {
    max_digits: usize,
    power_sums: Vec<BigUint>,
    offsets: Vec<BigUint>,
}

impl Base10x10Codec {
    pub fn new(max_digits: usize) -> Result<Self, String> {
        if max_digits == 0 {
            return Err("Max number of base10x10 digits must be greater than 0!".to_string());
        }

        let mut power_sums = Vec::with_capacity(max_digits);
        let mut offsets = Vec::with_capacity(max_digits);

        let mut power_sum = BigUint::from(0u32);
        for i in 1..=max_digits {
            power_sum += BigUint::from(100u32).pow(i as u32);
            power_sums.push(power_sum.clone());

            offsets.push(BigUint::from(100u32).pow(i as u32));
        }

        Ok(Base10x10Codec {
            max_digits,
            power_sums,
            offsets,
        })
    }

    pub fn encode_to_base10x10(&self, mut input: BigUint) -> Result<Vec<(u8, u8)>, String> {
        let mut expected_pairs = 1u32;
        for sum in &self.power_sums {
            if input >= *sum {
                expected_pairs += 1u32;
            } else {
                break;
            }
        }
        if expected_pairs > self.max_digits as u32 {
            return Err(
                "Base10 input is too large for the specified max number of base10x10 digits!"
                    .to_string(),
            );
        }

        for offset in &self.offsets {
            if input >= *offset {
                input -= offset.clone();
            }
        }
        let mut input_digits = input.to_radix_le(10);
        input_digits.reverse();

        let mut input_digit_pairs = Vec::new();
        while !input_digits.is_empty() {
            if input_digits.len() == 1 {
                if let Some(second) = input_digits.pop() {
                    let first = 0u8;
                    input_digit_pairs.push((first, second));
                } else {
                    return Err("An unexpected error occured!".to_string());
                }
                break;
            } else {
                if let Some(second) = input_digits.pop() {
                    if let Some(first) = input_digits.pop() {
                        input_digit_pairs.push((first, second));
                        continue;
                    } else {
                        return Err("An unexpected error occured!".to_string());
                    }
                } else {
                    return Err("An unexpected error occured!".to_string());
                }
            }
        }

        while input_digit_pairs.len() < expected_pairs as usize {
            input_digit_pairs.push((0, 0));
        }

        input_digit_pairs.reverse();
        Ok(input_digit_pairs)
    }

    pub fn decode_from_base10x10(&self, input: Vec<(u8, u8)>) -> Result<BigUint, String> {
        if input.len() > self.max_digits {
            return Err("Base10x10 input has more pairs than allowed by the specified max number of base10x10 digits!".to_string());
        }

        let mut result = BigUint::zero();
        let mut num_pairs = 0;

        for (i, &(first, second)) in input.iter().rev().enumerate() {
            if first >= 10 || second >= 10 {
                return Err("Invalid digits in base10x10 input!".to_string());
            }
            let pair_value = BigUint::from((first as u32) * 10 + (second as u32));
            result += pair_value * BigUint::from(100u32).pow(i as u32);
            num_pairs += 1;
        }

        for i in 0..num_pairs - 1 {
            result += self.offsets[i].clone();
        }

        Ok(result)
    }
}
