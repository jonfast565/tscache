use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc, DateTime};

use crate::utilities::*;

/*** 
 * Block is the basic unit of sotrage for the system
 * Contains metadata and the segment lists
 * Parsed in TLV format
 * Metdata is fixed length, filename length is max 8000 bytes (1000 chars).
 * Each dates are 8 bytes long.
 * ***/
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct BlockMetadata {
     filename: String,
     end_datetime: chrono::DateTime<Utc>,
     start_datetime: chrono::DateTime<Utc>,
 }
 
 impl BlockMetadata {
     pub fn serialize(&self) -> Vec<u8> {
         let mut data: Vec<u8> = vec![];
         let filename_bytes = self.filename.as_bytes();
         for i in 0..999 {
            data.write_u8(filename_bytes[i]).unwrap();
         }
         data.write_i64::<LittleEndian>(self.end_datetime.timestamp()).unwrap();
         data.write_i64::<LittleEndian>(self.start_datetime.timestamp()).unwrap();
 
         data
     }
 
     pub fn deserialize(data: &[u8]) -> BlockMetadata {
         let filename = String::from_utf8(data[0..999].to_vec()).unwrap();
         let end_datetime = dateutils::convert_i64_to_utc_datetime(LittleEndian::read_i64(&data[1000..1008]));
         let start_datetime = dateutils::convert_i64_to_utc_datetime(LittleEndian::read_i64(&data[1008..1016]));
 
         BlockMetadata {
             filename,
             end_datetime,
             start_datetime,
         }
     }
 }
 

pub struct Block {
    metadata: BlockMetadata,
    segments: Vec<Segment>,
    resident: bool,
}

pub struct Segment {
    segment_date: chrono::DateTime<Utc>,
    segment_values: Vec<SegmentValue>, 
}

pub struct SegmentValue {
    segment_name: String,
    segment_type: SegmentType,
    segment_value: Vec<u8>
}

pub enum SegmentType {
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    Float,
    Double,
    DateTime,
}