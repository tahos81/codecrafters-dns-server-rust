use super::Serializable;

#[derive(Debug, Clone)]
pub struct DnsQuestion {
    pub name: String,
    pub record_type: u16,
    pub class: u16,
}

impl DnsQuestion {
    pub fn new(name: String, record_type: u16, class: u16) -> Self {
        Self {
            name,
            record_type,
            class,
        }
    }
}

impl Serializable<Vec<u8>> for DnsQuestion {
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
        result
    }
}

impl Serializable<Vec<u8>> for Vec<DnsQuestion> {
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        for question in self {
            result.extend_from_slice(&question.serialize());
        }
        result
    }
}
