pub mod header;
pub mod question;
pub mod record;

pub trait Serializable<T: AsRef<[u8]>> {
    fn serialize(&self) -> T;
}
