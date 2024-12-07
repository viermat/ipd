use std::{net::Ipv4Addr, str::FromStr};

fn dec_to_oct(mut dec: u64) -> u64 {
    let mut buffer = [0u8; 22];
    let mut index = 22;

    if dec == 0 {
        return 0;
    }

    while dec > 0 {
        index -= 1;
        buffer[index] = b'0' + (dec % 8) as u8;
        dec /= 8;
    }

    String::from_utf8(buffer[index..].to_vec())
        .unwrap()
        .parse::<u64>()
        .unwrap()
}

pub struct IPAddress {
    pub ip: String,
    bytes: [u8; 4],
}

impl IPAddress {
    pub fn is_valid(&self) -> bool {
        !self.bytes.eq(&[0; 4])
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub fn to_dec(&self) -> u64 {
        let mut ip_addr_dec: u64 = 0;

        for (i, &byte) in self.bytes.iter().enumerate() {
            ip_addr_dec += (byte as u64) << (8 * (3 - i));
        }

        ip_addr_dec
    }

    pub fn to_hex(&self, full: bool) -> String {
        if full {
            self.bytes
                .iter()
                .map(|byte| format!("{:#02x}", byte))
                .collect::<Vec<String>>()
                .join(".")
        } else {
            "0x".to_owned()
                + &self
                    .bytes
                    .iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect::<Vec<String>>()
                    .join("")
        }
    }

    pub fn to_oct(&self, full: bool) -> String {
        if full {
            let mut ip_addr_oct: [u64; 4] = [0; 4];

            for _dec in self.bytes.iter().enumerate() {
                ip_addr_oct[_dec.0] = dec_to_oct(*_dec.1 as u64);
            }

            let result: Vec<String> = ip_addr_oct
                .iter()
                .map(|n| format!("0{}", n.to_string()))
                .collect();

            result.join(".")
        } else {
            "0".to_owned() + &dec_to_oct(self.to_dec()).to_string()
        }
    }
}

impl TryFrom<u64> for IPAddress {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, String> {
        if value <= u32::MAX as u64 {
            let bytes = [
                ((value >> 24) & 0xff) as u8,
                ((value >> 16) & 0xff) as u8,
                ((value >> 8) & 0xff) as u8,
                (value & 0xff) as u8,
            ];

            let ip = Ipv4Addr::from(bytes).to_string();

            Ok(Self { ip, bytes })
        } else {
            Err(format!(
                "Invalid IP address: {}\nImpossible decimal representation",
                value
            ))
        }
    }
}

impl TryFrom<String> for IPAddress {
    type Error = String;

    fn try_from(value: String) -> Result<Self, String> {
        let ip_addr: Ipv4Addr =
            Ipv4Addr::from_str(&value).map_err(|_| format!("Invalid IP address: {}", value))?;

        let bytes: [u8; 4] = ip_addr.octets();
        let ip: String = ip_addr.to_string();

        Ok(Self { ip, bytes })
    }
}
