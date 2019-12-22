use std::fmt;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

use ouey::mac::normalize_mac_str;

struct OuiRecord {
    mac_str: String,
    vendor: String,
}

struct OuiDb {
    records: Vec<OuiRecord>,
}

impl fmt::Display for OuiRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.mac_str, self.vendor)
    }
}

impl fmt::Display for OuiDb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let record_count = self.records.len();
        write!(f, "An OUI DB of {} records", record_count)
    }
}

fn main() -> Result<()> {
    let db_path = "/usr/lib/udev/hwdb.d/20-OUI.hwdb";
    // let db_path = "20-OUI.hwdb";
    let mut db_file = File::open(db_path)?;
    // db_file.read_to_string(&mut contents)?;
    let db = parse_db_file(&mut db_file).unwrap();

    let first_arg = args().nth(1).unwrap();
    let input_mac = normalize_mac_str(&first_arg);

    for r in &db.records {
        if input_mac.starts_with(&r.mac_str) {
            println!("{}\t{}", input_mac, r.vendor);
        }
    }

    Ok(())
}

fn parse_db_file(db_file: &std::fs::File) -> Result<OuiDb> {
    let mut records = Vec::new();
    let mut reader = BufReader::new(db_file);

    loop {
        match read_record(&mut reader) {
            Ok(r) => { records.push(r) }
            _ => { break }
        }
    };

    Ok(OuiDb {
        records,
    })
}

fn read_record(reader: &mut BufReader<&File>) -> Result<OuiRecord> {
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line)? {
            0 => {
                return Err(Error::new(ErrorKind::Other, "No more content to parse"));
            }
            1 => { // empty line
                line.clear();
                continue;
            }
            _ => {
                if line.starts_with("#") { // comment
                    line.clear();
                    continue;
                }

                let mut r = OuiRecord{
                    mac_str: String::new(),
                    vendor: String::new(),
                };

                parse_mac_line(&line, &mut r);

                let mut vendor_line = String::new();
                reader.read_line(&mut vendor_line)?;
                parse_vendor_line(&vendor_line, &mut r);
                vendor_line.clear();

                line.clear();

                return Ok(r);
            }
        }
    };
}

fn parse_mac_line(line: &String, r: &mut OuiRecord) {
    let chars = line.chars();
    let mac_prefix: String = chars.skip(4).take_while(|c| *c != '*').collect();
    r.mac_str = normalize_mac_str(&mac_prefix);
}

fn parse_vendor_line(line: &String, r: &mut OuiRecord) {
    let chars = line.chars();
    let len = line.len();
    let all_but_prefix = chars.skip(22);
    let vendor = all_but_prefix.take(len - 22 - 1).collect();
    r.vendor = vendor;
}
