pub struct Block {
    segments: Vec<Segment>
}

pub struct Segment {
    segment_date: chrono::DateTime,
    segment_values: Vec<SegmentValue>, 
}

pub struct SegmentValue {
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