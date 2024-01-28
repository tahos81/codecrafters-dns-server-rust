pub type DnsQuestion = Vec<u8>;

pub struct DnsQuestionStruct {
    name: String,
    record_type: u16,
    class: u16,
}

impl DnsQuestionStruct {
    pub fn new(name: String, record_type: u16, class: u16) -> Self {
        Self {
            name,
            record_type,
            class,
        }
    }
}

impl From<DnsQuestionStruct> for DnsQuestion {
    fn from(question: DnsQuestionStruct) -> Self {
        let mut result = Vec::new();
        let labels: Vec<&str> = question.name.split('.').collect();
        for label in labels {
            result.push(label.len() as u8);
            result.extend_from_slice(label.as_bytes());
        }
        result.push(0);
        result.extend_from_slice(&question.record_type.to_be_bytes());
        result.extend_from_slice(&question.class.to_be_bytes());
        result
    }
}
