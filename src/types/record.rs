use super::Serializable;

#[derive(Debug)]
pub struct DnsRecord {
    name: String,
    record_type: u16,
    class: u16,
    ttl: u32,
    data: Vec<u8>,
}

impl DnsRecord {
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

impl Serializable<Vec<u8>> for DnsRecord {
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let labels: Vec<&str> = self.name.split('.').collect();
        for label in labels {
            result.push(label.len() as u8);
            result.extend_from_slice(label.as_bytes());
        }
        result.push(0);
        result.extend_from_slice(&self.record_type.to_be_bytes());
        result.extend_from_slice(&self.class.to_be_bytes());
        result.extend_from_slice(&self.ttl.to_be_bytes());
        result.extend_from_slice(&(self.data.len() as u16).to_be_bytes());
        result.extend_from_slice(&self.data);
        result
    }
}

impl Serializable<Vec<u8>> for Vec<DnsRecord> {
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        for record in self {
            result.extend_from_slice(&record.serialize());
        }
        result
    }
}
