use err::Error;
use lz4;
use std::{fs::File, io::prelude::*, path::Path};

#[inline]
pub fn compress_buf(src: &[u8], dst: &mut [u8]) -> Result<(), Error> {
    let mut out = lz4::EncoderBuilder::new().build(dst)?;
    out.write_all(src)?;

    out.finish().1.map_err(|e| e.into())
}

pub fn compress_file<P: AsRef<Path>>(src: P, dst: P) -> Result<(), Error> {
    let mut infile = File::open(src)?;
    let mut outfile = lz4::EncoderBuilder::new().build(File::create(dst)?)?;

    copy(&mut infile, &mut outfile)?;

    outfile.finish().1.map_err(|e| e.into())
}

#[inline]
fn copy<R: Read, W: Write>(src: &mut R, dst: &mut W) -> Result<(), Error> {
    let mut buf = [0u8; 8 * 1024];
    loop {
        let len = src.read(&mut buf)?;
        if len == 0 {
            break;
        }
        dst.write_all(&buf[0..len])?;
    }

    Ok(())
}
