pub trait Externalizable {
    fn to_external(&self) -> u128;
    fn from_external(handle: u128) -> Self;
}