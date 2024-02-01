pub mod header;
pub mod packet;
pub mod question;
pub mod record;

pub trait Serializable<T: AsRef<[u8]>> {
    fn serialize(&self) -> T;
}
