use std;
use kaitai_stream::KaitaiStream;

pub trait KaitaiStruct<'a> {
    fn from_file(path: &str) -> std::io::Result<Self> where Self : Sized {
        let mut f = std::fs::File::open(path)?;
        Self::new(&mut f, &None, &None)
    }
    
    fn from_bytes(bytes: Vec<u8>) -> std::io::Result<Self> where Self : Sized {
        let mut b = std::io::Cursor::new(bytes);
        Self::new(&mut b, &None, &None)
    }
    
    fn new<S: KaitaiStream>(stream: &mut S,
                            parent: Option<&'a KaitaiStruct<'a>>,
                            root: Option<&'a KaitaiStruct<'a>>)
                            -> std::io::Result<Self>
        where Self : Sized;
    
    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S)
                             -> std::io::Result<()> where Self : Sized;
}
