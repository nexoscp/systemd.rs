use bytebuffer::ByteBuffer;
use log::kv::{Error, Key, Value, VisitSource};
use crate::write;

pub(crate) struct KVBuffer<'l>  {
    internal: &'l mut ByteBuffer
}

impl <'l> KVBuffer<'l> {
    pub fn new(internal: &'l mut ByteBuffer) -> KVBuffer<'l> {
        KVBuffer { internal }
    }
}

impl VisitSource<'_> for KVBuffer<'_> {
    fn visit_pair(&mut self, key: Key<'_>, value: Value<'_>) -> Result<(), Error> {
        write(&mut self.internal, key.to_string().as_bytes(), value.to_string().as_bytes());
        Ok(())
    }
}