use std::ffi::c_int;

trait Datafile {
    fn FileID(&self) -> i32;
    fn Name(&self) -> str;
    fn Read(&self) -> (internal::Entry, i64, std::error);
    fn Close(&self) -> std::error;
    fn Sync(&self) -> std::error;
    fn Size(&self) -> i64;
    fn ReadAt(index: i64, size: i64) -> (internal::Entry, std::error);
    fn Write(internal::Entry) -> (i64, i64, i64);
}

struct datafile {
    id: i32,
    r: std::fs::File,
    ra: * mmap.ReaderAt,
    w: std::fs::File,
    offset: i64,
    dec: * codec.Decoder,
    enc: * codec.Decoder,
    max_key_size: u32,
    max_value_size: u64,
}