pub type DnsRecord = Vec<u8>;

pub struct DnsRecordStruct {
    name: String,
    record_type: u16,
    class: u16,
    ttl: u32,
    data: Vec<u8>,
}

impl DnsRecordStruct {
    pub fn new(name: String, record_type: u16, class: u16, ttl: u32, data: Vec<u8>) -> Self {
        Self {
            name,
            record_type,
            class,
            ttl,
            data,
        }
    }
}

impl From<DnsRecordStruct> for DnsRecord {
    fn from(record: DnsRecordStruct) -> Self {
        let mut result = Vec::new();
        let labels: Vec<&str> = record.name.split('.').collect();
        for label in labels {
            result.push(label.len() as u8);
            result.extend_from_slice(label.as_bytes());
        }
        result.push(0);
        result.extend_from_slice(&record.record_type.to_be_bytes());
        result.extend_from_slice(&record.class.to_be_bytes());
        result.extend_from_slice(&record.ttl.to_be_bytes());
        result.extend_from_slice(&(record.data.len() as u16).to_be_bytes());
        result.extend_from_slice(&record.data);
        result
    }
}
