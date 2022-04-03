use std::io::Read;
use std::io::Write;

pub trait Channel: Read + Write {}
