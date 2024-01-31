use super::Serializable;

#[derive(Debug)]
pub struct DnsHeader {
    pub id: u16,
    pub qr: u8,
    pub opcode: u8,
    pub flags: u8,
    pub z: u8,
    pub rcode: u8,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl DnsHeader {
    pub fn new(
        id: u16,
        qr: u8,
        opcode: u8,
        flags: u8,
        z: u8,
        rcode: u8,
        qdcount: u16,
        ancount: u16,
        nscount: u16,
        arcount: u16,
    ) -> Self {
        Self {
            id,
            qr,
            opcode,
            flags,
            z,
            rcode,
            qdcount,
            ancount,
            nscount,
            arcount,
        }
    }
    pub fn builder(id: u16) -> DnsHeaderBuilder {
        DnsHeaderBuilder::new(id)
    }
}

impl Serializable<[u8; 12]> for DnsHeader {
    fn serialize(&self) -> [u8; 12] {
        let mut header = [0; 12];
        header[0] = (self.id >> 8) as u8;
        header[1] = self.id as u8;
        header[2] = self.qr << 7 | self.opcode << 3 | self.flags >> 1;
        header[3] = self.flags << 7 | self.z << 4 | self.rcode;
        header[4] = (self.qdcount >> 8) as u8;
        header[5] = self.qdcount as u8;
        header[6] = (self.ancount >> 8) as u8;
        header[7] = self.ancount as u8;
        header[8] = (self.nscount >> 8) as u8;
        header[9] = self.nscount as u8;
        header[10] = (self.arcount >> 8) as u8;
        header[11] = self.arcount as u8;
        header
    }
}

#[derive(Debug, Default)]
pub struct DnsHeaderBuilder {
    id: u16,
    qr: Option<u8>,
    opcode: Option<u8>,
    flags: Option<u8>,
    z: Option<u8>,
    rcode: Option<u8>,
    qdcount: Option<u16>,
    ancount: Option<u16>,
    nscount: Option<u16>,
    arcount: Option<u16>,
}

impl DnsHeaderBuilder {
    pub fn new(id: u16) -> Self {
        DnsHeaderBuilder {
            id,
            ..Default::default()
        }
    }

    pub fn qr(mut self, qr: u8) -> Self {
        self.qr = Some(qr);
        self
    }

    pub fn opcode(mut self, opcode: u8) -> Self {
        self.opcode = Some(opcode);
        self
    }

    pub fn flags(mut self, flags: u8) -> Self {
        self.flags = Some(flags);
        self
    }

    pub fn z(mut self, z: u8) -> Self {
        self.z = Some(z);
        self
    }

    pub fn rcode(mut self, rcode: u8) -> Self {
        self.rcode = Some(rcode);
        self
    }

    pub fn qdcount(mut self, qdcount: u16) -> Self {
        self.qdcount = Some(qdcount);
        self
    }

    pub fn ancount(mut self, ancount: u16) -> Self {
        self.ancount = Some(ancount);
        self
    }

    pub fn nscount(mut self, nscount: u16) -> Self {
        self.nscount = Some(nscount);
        self
    }

    pub fn arcount(mut self, arcount: u16) -> Self {
        self.arcount = Some(arcount);
        self
    }

    pub fn build(self) -> DnsHeader {
        DnsHeader::new(
            self.id,
            self.qr.unwrap_or(1),
            self.opcode.unwrap_or(0),
            self.flags.unwrap_or(0),
            self.z.unwrap_or(0),
            self.rcode.unwrap_or(0),
            self.qdcount.unwrap_or(0),
            self.ancount.unwrap_or(0),
            self.nscount.unwrap_or(0),
            self.arcount.unwrap_or(0),
        )
    }
}
