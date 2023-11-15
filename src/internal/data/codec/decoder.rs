use std::io::Read;

const KEY_SIZE: i8 = 4;
const VALUE_SIZE: i8 = 8;
const CHECKSUM_SIZE: i8 = 4;
const TTL_SIZE: i8 = 8;

struct Decoder {
    r: std::io::Read,
    max_key_size: u32,
    max_value_size: u32,
}

fn NewDecoder(r: Box<std::io::Read>, max_key_size: u32, max_value_size: u64) -> * Decoder {
    return &Decoder{
        r=r,
    }
}