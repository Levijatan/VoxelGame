use bevy::{app::{Plugin, AppBuilder}, asset::AddAsset, ecs::ResMut, render::texture::TextureFormat, math::vec2, prelude::Assets, prelude::Handle, prelude::Texture};
use image::{GenericImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use noise::{Clamp, NoiseFn, OpenSimplex};

fn generate_grass_texture() -> Texture {
    let simplex = OpenSimplex::new();
    let noise = Clamp::new(&simplex).set_lower_bound(0.0).set_upper_bound(255.0);
    let mut tex: RgbaImage = ImageBuffer::new(16, 16);
    for x in 0..16 {
        for y in 0..16 {
            let n = noise.get([x as f64, y as f64]) as u8;
            let pixel = Rgba([n, n, n, n]);
            tex.put_pixel(x, y, pixel);
        }
    }
    Texture::new(vec2(16.0, 16.0), tex.into_raw(), TextureFormat::Rgba8Uint)
}