use std::io::Cursor;

use image_webp::WebPDecoder;
use thiserror::Error;
use zune_jpeg::{JpegDecoder, zune_core::bytestream::ZCursor};

#[derive(Debug, Clone)]
enum ImageFormat {
    Jpeg,
    Png,
    Webp,
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Unknown or unsupported image format")]
    UnknownFormat,
    #[error("Image too large")]
    TooLarge,
    #[error("JPEG decode error: {0}")]
    Jpeg(#[from] zune_jpeg::errors::DecodeErrors),
    #[error("PNG decode error: {0:?}")]
    Png(#[from] minipng::Error),
    #[error("WebP decode error: {0}")]
    WebP(#[from] image_webp::DecodingError),
}

fn guess_format(data: &[u8]) -> Option<ImageFormat> {
    match data {
        [0xFF, 0xD8, 0xFF, ..] => Some(ImageFormat::Jpeg),
        [0x89, b'P', b'N', b'G', ..] => Some(ImageFormat::Png),
        [
            b'R',
            b'I',
            b'F',
            b'F',
            _,
            _,
            _,
            _,
            b'W',
            b'E',
            b'B',
            b'P',
            ..,
        ] => Some(ImageFormat::Webp),
        _ => None,
    }
}

fn decode_jpeg(data: &[u8]) -> Result<Vec<u8>, DecodeError> {
    let cur = ZCursor::new(data);
    let mut decoder = JpegDecoder::new(cur);
    let pixels_rgb = decoder.decode()?;
    let pixels = rgb_to_rgba(pixels_rgb);
    Ok(pixels)
}

fn decode_png(data: &[u8]) -> Result<Vec<u8>, DecodeError> {
    let header = minipng::decode_png_header(data)?;
    let mut buf = vec![0; header.required_bytes_rgba8bpc()];
    let mut image = minipng::decode_png(data, &mut buf)?;
    image.convert_to_rgba8bpc()?;
    Ok(buf)
}

fn decode_webp(data: &[u8]) -> Result<Vec<u8>, DecodeError> {
    let cur = Cursor::new(data);
    let mut decoder = WebPDecoder::new(cur)?;
    let mut pixels = vec![0u8; decoder.output_buffer_size().ok_or(DecodeError::TooLarge)?];
    decoder.read_image(&mut pixels)?;
    let pixels = if !decoder.has_alpha() {
        rgb_to_rgba(pixels)
    } else {
        pixels
    };
    Ok(pixels)
}

fn rgb_to_rgba(rgb: Vec<u8>) -> Vec<u8> {
    rgb.chunks(3)
        .flat_map(|p| [p[0], p[1], p[2], 255])
        .collect()
}

pub fn decode(data: &[u8]) -> Result<Vec<u8>, DecodeError> {
    match guess_format(data).ok_or(DecodeError::UnknownFormat)? {
        ImageFormat::Jpeg => decode_jpeg(data),
        ImageFormat::Png => decode_png(data),
        ImageFormat::Webp => decode_webp(data),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_format_jpeg() {
        assert!(matches!(
            guess_format(&[0xFF, 0xD8, 0xFF, 0x00]),
            Some(ImageFormat::Jpeg)
        ))
    }

    #[test]
    fn test_guess_format_png() {
        assert!(matches!(
            guess_format(&[0x89, b'P', b'N', b'G']),
            Some(ImageFormat::Png)
        ))
    }

    #[test]
    fn test_guess_format_webp() {
        assert!(matches!(
            guess_format(&[
                b'R', b'I', b'F', b'F', 0x00, 0x00, 0x00, 0x00, b'W', b'E', b'B', b'P'
            ]),
            Some(ImageFormat::Webp)
        ))
    }
}
