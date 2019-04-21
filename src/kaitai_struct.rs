use std;
use std::boxed::Box;
use std::rc::Weak;
use kaitai_stream::KaitaiStream;

pub trait KaitaiStruct {
    fn from_file(path: &str) -> std::io::Result<Self> where Self : Sized {
        let mut f = std::fs::File::open(path)?;
        Self::new(Box::new(f), None, None)
    }
    
    fn from_bytes(bytes: Vec<u8>) -> std::io::Result<Self> where Self : Sized {
        let mut b = std::io::Cursor::new(bytes);
        Self::new(Box::new(b), None, None)
    }
    
    fn new(stream: Box<KaitaiStream>,
           parent: Option<Weak<KaitaiStruct>>,
           root: Option<Weak<KaitaiStruct>>)
           -> std::io::Result<Self>
        where Self : Sized;
}
