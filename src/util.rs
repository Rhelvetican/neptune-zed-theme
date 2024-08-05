use std::{fs::write, path::Path};

use serde::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer};

const INDENT: &[u8] = b"    ";

pub fn write_theme<P: AsRef<Path>, S: Serialize>(path: P, value: S) {
    let mut buf = Vec::new();
    let fmt = PrettyFormatter::with_indent(INDENT);
    let mut ser = Serializer::with_formatter(&mut buf, fmt);

    value.serialize(&mut ser).unwrap();
    write(path, buf).unwrap();
}
