use super::{header::DnsHeader, question::DnsQuestion, record::DnsRecord, Serializable};

#[derive(Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub additionals: Vec<DnsRecord>,
}

impl DnsPacket {
    pub fn new(
        header: DnsHeader,
        questions: Vec<DnsQuestion>,
        answers: Vec<DnsRecord>,
        authorities: Vec<DnsRecord>,
        additionals: Vec<DnsRecord>,
    ) -> Self {
        Self {
            header,
            questions,
            answers,
            authorities,
            additionals,
        }
    }
}

impl Serializable<Vec<u8>> for DnsPacket {
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(&self.header.serialize());
        result.extend_from_slice(&self.questions.serialize());
        result.extend_from_slice(&self.answers.serialize());
        result.extend_from_slice(&self.authorities.serialize());
        result.extend_from_slice(&self.additionals.serialize());
        result
    }
}
