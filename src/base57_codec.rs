use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};

pub struct Base57Codec {
    max_digits: usize,
}

impl Base57Codec {
    pub fn new(max_digits: usize) -> Result<Self, String> {
        if max_digits == 0 {
            return Err("Max number of base57 digits must be greater than 0!".to_string());
        }

        let mut powers_of_base = Vec::with_capacity(max_digits);
        let base = BigUint::from(57u32);
        let mut current_power = BigUint::one();
        for _ in 1..=max_digits {
            current_power *= &base;
            powers_of_base.push(current_power.clone());
        }

        Ok(Base57Codec { max_digits })
    }

    pub fn encode_to_base57(&self, mut input: BigUint) -> Result<String, String> {
        let charset = "abcdefghijklmnopqrstuvwxyz0123456789+,;_-'~`´@!$%&()[]{}=";
        let base = BigUint::from(57u32);

        let mut result = String::new();

        while input != BigUint::zero() {
            let rem = &input % &base;
            if let Some(character) = charset.chars().nth(
                rem.to_usize().ok_or(
                    "Base10 input is too large for the specified max number of base57 digits!"
                        .to_string(),
                )?,
            ) {
                result.push(character);
            }
            input = input / &base;
        }

        while result.chars().count() < self.max_digits {
            if let Some(character) = charset.chars().nth(0) {
                result.push(character);
            } else {
                return Err("An unexpected error occured!".to_string());
            }
        }

        if result.chars().count() > self.max_digits {
            return Err("An unexpected error occured!".to_string());
        }

        result = result.chars().rev().collect::<String>();

        Ok(result)
    }

    pub fn decode_from_base57(&self, input: &str) -> Result<BigUint, String> {
        if input.chars().count() > self.max_digits {
            return Err(
                "Base57 input is too large for the specified max number of base57 digits!"
                    .to_string(),
            );
        }

        let charset = "abcdefghijklmnopqrstuvwxyz0123456789+,;_-'~`´@!$%&()[]{}=";
        let base = BigUint::from(57u32);

        let mut result = BigUint::zero();
        let mut multiplier = BigUint::one();

        for char in input.chars().rev() {
            if let Some(position) = charset.chars().position(|c| c == char) {
                let value = BigUint::from(
                    position
                        .to_u32()
                        .ok_or("An unexpected error occured!".to_string())?,
                );
                result += value * &multiplier;
            } else {
                return Err(format!("Invalid digit '{}' in the base57 input!", char));
            }

            multiplier = multiplier * &base;
        }

        Ok(result)
    }
}
