use std;
use kaitai_stream::KaitaiStream;

pub trait KaitaiStruct<'a> {
    fn from_file(path: &str) -> std::io::Result<Self> where Self : Sized {
        let mut f = std::fs::File::open(path)?;
        Self::new(f, None, None)
    }
    
    fn from_bytes(bytes: Vec<u8>) -> std::io::Result<Self> where Self : Sized {
        let mut b = std::io::Cursor::new(bytes);
        Self::new(b, None, None)
    }
    
    fn new(stream: KaitaiStream,
           parent: Option<&'a KaitaiStruct<'a>>,
           root: Option<&'a KaitaiStruct<'a>>)
           -> std::io::Result<Self>
        where Self : Sized;
    
    fn read(&mut self)
            -> std::io::Result<()> where Self : Sized;
}
