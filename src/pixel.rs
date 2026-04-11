use image::Luma;

pub trait PixelExt {
    fn to_luma_f32(&self) -> f32;
}

impl PixelExt for Luma<u8> {
    fn to_luma_f32(&self) -> f32 {
        (self.0[0] as f32) / 256.0
    }
}
