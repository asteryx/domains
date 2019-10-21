use base64;
use byteorder::{BigEndian, ByteOrder};
use hmac::Hmac;
use pbkdf2::pbkdf2;
use rand::{rngs::OsRng, RngCore};
use sha2::Sha256;
use std::fmt::Display;
use subtle::ConstantTimeEq;

#[derive(Debug)]
pub enum HasherError {
    PasswordEmpty,
    SaltNotGenerated,
    InvalidFormat,
    HashMismatch,
}

impl Display for HasherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HasherError")
    }
}

pub struct PBKDF2PasswordHasher {
    pub iterations: u32,
}

impl PBKDF2PasswordHasher {
    pub fn new() -> PBKDF2PasswordHasher {
        PBKDF2PasswordHasher::default()
    }

    pub fn encode(&self, raw_password: &str) -> Result<String, HasherError> {
        let algorithm = "pbkdf2_sha256".to_string();

        if raw_password == "".to_string() {
            return Err(HasherError::PasswordEmpty);
        };

        let mut rng = OsRng {};

        // 128-bit salt
        let mut salt = [0u8; 16];
        match rng.try_fill_bytes(&mut salt) {
            Ok(_) => (),
            Err(_) => return Err(HasherError::SaltNotGenerated),
        };

        // 256-bit derived key
        let mut derived_key = [0u8; 32];

        pbkdf2::<Hmac<Sha256>>(
            raw_password.as_bytes(),
            &salt,
            self.iterations as usize,
            &mut derived_key,
        );

        let mut result = "$rpbkdf2$0$".to_string();
        let mut tmp = [0u8; 4];
        BigEndian::write_u32(&mut tmp, self.iterations);
        result.push_str(&base64::encode(&tmp));
        result.push('$');
        result.push_str(&base64::encode(&salt));
        result.push('$');
        result.push_str(&base64::encode(&derived_key));
        result.push('$');
        Ok(result)
    }

    pub fn verify(&self, raw_password: &str, hashed_value: &str) -> Result<bool, HasherError> {
        let mut iter = hashed_value.split('$');

        // Check that there are no characters before the first "$"
        if iter.next() != Some("") {
            return Err(HasherError::InvalidFormat);
        };

        // Check the name
        if iter.next() != Some("rpbkdf2") {
            return Err(HasherError::InvalidFormat);
        };

        // Parse format - currenlty only version 0 is supported
        match iter.next() {
            Some(fstr) => match fstr {
                "0" => {}
                _ => return Err(HasherError::InvalidFormat),
            },
            None => return Err(HasherError::InvalidFormat),
        }

        // Parse the iteration count
        let c = match iter.next() {
            Some(pstr) => match base64::decode(pstr) {
                Ok(pvec) => {
                    if pvec.len() != 4 {
                        return Err(HasherError::InvalidFormat);
                    }
                    BigEndian::read_u32(&pvec[..])
                }
                Err(_) => return Err(HasherError::InvalidFormat),
            },
            None => return Err(HasherError::InvalidFormat),
        };

        // Salt
        let salt = match iter.next() {
            Some(sstr) => match base64::decode(sstr) {
                Ok(salt) => salt,
                Err(_) => return Err(HasherError::InvalidFormat),
            },
            None => return Err(HasherError::InvalidFormat),
        };

        // Hashed value
        let hash = match iter.next() {
            Some(hstr) => match base64::decode(hstr) {
                Ok(hash) => hash,
                Err(_) => return Err(HasherError::InvalidFormat),
            },
            None => return Err(HasherError::InvalidFormat),
        };

        // Make sure that the input ends with a "$"
        if iter.next() != Some("") {
            Err(HasherError::InvalidFormat)?;
        }

        // Make sure there is no trailing data after the final "$"
        if iter.next() != None {
            Err(HasherError::InvalidFormat)?;
        }

        let mut output = vec![0u8; hash.len()];
        pbkdf2::<Hmac<Sha256>>(raw_password.as_bytes(), &salt, c as usize, &mut output);

        // Be careful here - its important that the comparison be done using a fixed
        // time equality check. Otherwise an adversary that can measure how long
        // this step takes can learn about the hashed value which would allow them
        // to mount an offline brute force attack against the hashed password.
        if output.ct_eq(&hash).unwrap_u8() == 1 {
            Ok(true)
        } else {
            Err(HasherError::HashMismatch)
        }
    }
}
impl Default for PBKDF2PasswordHasher {
    fn default() -> PBKDF2PasswordHasher {
        PBKDF2PasswordHasher { iterations: 12_032 }
    }
}
