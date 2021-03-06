use thiserror::Error;

pub mod huffman;
pub mod bit_pump;
pub mod byte_stream;
pub mod lookup_table;
pub mod ljpeg;

#[derive(Error, Debug)]
pub enum DecodingError {
    #[error("No marker found inside rest of buffer.")]
    ByteStreamNoMarkerFound,
    #[error("LJPEGDecompressor trying to decode {0}x{1} into {2}x{3} error.")]
    LJpegDecompressingError(usize, usize, usize, usize),
    #[error("LJpegDecompressor {0} component files is not supported.")]
    LJpegComponentFilesNotSupported(usize),
    #[error("LJpegDecompressor predictor {0} is not supported.")]
    LJpegPredictorNotSupported(usize),
    #[error("LJpegDecompressing error: {0}")]
    LJpegError(&'static str)
}