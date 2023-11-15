use std::os::raw::c_int;

struct Entry {
    checksum: i32,
    key: [u8; 10],
    offset: c_int,
    value: [u8; 4],
    expiry: * time::Instant,
}