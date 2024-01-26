pub type DnsHeader = [u8; 12];

#[derive(Debug)]
pub struct DnsHeaderStruct {
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

impl DnsHeaderStruct {
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
        DnsHeaderStruct {
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

impl From<DnsHeaderStruct> for DnsHeader {
    fn from(value: DnsHeaderStruct) -> Self {
        let mut header = [0; 12];
        header[0] = (value.id >> 8) as u8;
        header[1] = value.id as u8;
        header[2] = value.qr << 7 | value.opcode << 3 | value.flags >> 1;
        header[3] = value.flags << 7 | value.z << 4 | value.rcode;
        header[4] = (value.qdcount >> 8) as u8;
        header[5] = value.qdcount as u8;
        header[6] = (value.ancount >> 8) as u8;
        header[7] = value.ancount as u8;
        header[8] = (value.nscount >> 8) as u8;
        header[9] = value.nscount as u8;
        header[10] = (value.arcount >> 8) as u8;
        header[11] = value.arcount as u8;
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

    pub fn build(self) -> DnsHeaderStruct {
        DnsHeaderStruct::new(
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
