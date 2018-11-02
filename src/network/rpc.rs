pub trait Rpc {
    fn to_bytes(&self) -> &[u8];
    fn from_bytes(bytes: &[u8]);
}