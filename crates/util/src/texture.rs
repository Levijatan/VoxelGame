use bevy::{prelude::Texture, render::texture::Extent3d, render::texture::TextureFormat};
use image::{ImageBuffer, Rgba, RgbaImage};
use noise::{Clamp, NoiseFn, OpenSimplex};

pub fn generate_grass_texture() -> Texture {
    let simplex = OpenSimplex::new();
    let noise = Clamp::new(&simplex)
        .set_lower_bound(0.0)
        .set_upper_bound(255.0);
    let mut tex: RgbaImage = ImageBuffer::new(16, 16);
    for x in 0..16 {
        for y in 0..16 {
            let n = noise.get([x as f64, y as f64]) as u8;
            let pixel = Rgba([n, n, n, 255]);
            tex.put_pixel(x, y, pixel);
        }
    }
    Texture::new_fill(
        Extent3d::new(16, 16, 1),
        bevy::render::texture::TextureDimension::D2,
        &tex.into_raw(),
        TextureFormat::Rgba8Uint,
    )
}


