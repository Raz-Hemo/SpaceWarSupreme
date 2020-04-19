use crate::engine::prelude::*;
use std::collections::HashMap;
use glium::{Display, texture::{CompressedTexture2d, CompressedSrgbTexture2d, RawImage2d, SrgbCubemap}};

pub enum Texture {
    Albedo(CompressedSrgbTexture2d),
    RoughnessMetal(CompressedTexture2d),
    NormalMap(CompressedTexture2d),
    AmbientOcclusion(CompressedTexture2d),
    Cubemap(SrgbCubemap),
}

impl Texture {
    /// This is the "invalid texture" texture - a pink-yellow grid
    pub fn new_default_albedo(display: &Display) -> CompressedSrgbTexture2d {
        let mut imgbuf = image::ImageBuffer::new(2, 2);
        imgbuf.put_pixel(0, 0, image::Rgb([255, 0, 255]));
        imgbuf.put_pixel(1, 1, image::Rgb([255, 0, 255]));
        imgbuf.put_pixel(1, 0, image::Rgb([255, 255, 0]));
        imgbuf.put_pixel(0, 1, image::Rgb([255, 255, 0]));

        let tex = RawImage2d::from_raw_rgb(imgbuf.into_raw(), (2, 2));
        CompressedSrgbTexture2d::new(display, tex).expect(
            "Failed to create default albedo texture"
        )
    }

    /// This is the default roughness/metallic map (rough and non metallic)
    pub fn new_default_rough_metal(display: &Display) -> CompressedTexture2d {
        let mut imgbuf = image::ImageBuffer::new(1, 1);
        imgbuf.put_pixel(0, 0, image::Rgb([0, 255, 0]));

        let tex = RawImage2d::from_raw_rgb(imgbuf.into_raw(), (1, 1));
        CompressedTexture2d::new(display, tex).expect(
            "Failed to create default roughness/metal texture"
        )
    }

    /// This is the default normal map (all +Z, toward surface normal)
    pub fn new_default_normal(display: &Display) -> CompressedTexture2d {
        let mut imgbuf = image::ImageBuffer::new(1, 1);
        imgbuf.put_pixel(0, 0, image::Rgb([128, 128, 255]));

        let tex = RawImage2d::from_raw_rgb(imgbuf.into_raw(), (1, 1));
        CompressedTexture2d::new(display, tex).expect(
            "Failed to create default normal map"
        )
    }

    /// This is the default occlusion map (all ones - fully lit)
    pub fn new_default_occ(display: &Display) -> CompressedTexture2d {
        let mut imgbuf = image::ImageBuffer::new(1, 1);
        imgbuf.put_pixel(0, 0, image::Rgb([255, 255, 255]));

        let tex = RawImage2d::from_raw_rgb(imgbuf.into_raw(), (1, 1));
        CompressedTexture2d::new(display, tex).expect(
            "Failed to create default AO map"
        )
    }


    pub fn from<P: AsRef<std::path::Path>>(filename: P, display: &Display)
    -> anyhow::Result<Texture> {
        use anyhow::Context;
        let tex = utils::load_image(filename)?.to_rgba();
        let dims = tex.dimensions();
        let tex = RawImage2d::from_raw_rgba(tex.into_raw(), dims);
        Ok(Texture::Albedo(
            CompressedSrgbTexture2d::new(display, tex)
            .context("Failed to load texture")?
        ))
    }

    pub fn cubemap<P: AsRef<std::path::Path>>(filename: P, display: &Display)
    -> anyhow::Result<Texture> {
        use glium::texture::CubeLayer;
        use glium::Surface;
        use image::GenericImageView;
        use anyhow::Context;

        // Validate this is a square image
        let size = utils::load_image(utils::extend_filename(filename.as_ref(), "_up"))?.dimensions();
        if size.0 != size.1 {
            return Err(anyhow!("Cubemap images are not square"));
        }
        let size = size.0;

        let tex = SrgbCubemap::empty(display, size)
        .context("Failed to create cubemap")?;

        let layers = [
            (CubeLayer::NegativeX, "_right"),
            (CubeLayer::NegativeY, "_up"),
            (CubeLayer::NegativeZ, "_back"),
            (CubeLayer::PositiveX, "_left"),
            (CubeLayer::PositiveY, "_down"),
            (CubeLayer::PositiveZ, "_front")
        ];
        for (layer, suffix) in layers.iter() {
            let framebuffer = glium::framebuffer::SimpleFrameBuffer::new(
                display,
                tex.main_level().image(*layer)
            ).context("Failed to create cubemap fb")?;

            // validate size
            let image = utils::load_image(utils::extend_filename(filename.as_ref(), suffix))?;
            if image.dimensions().0 != image.dimensions().1 {
                return Err(anyhow!("Cubemap images are not square"));
            }
            if image.dimensions().0 != size {
                return Err(anyhow!("Cubemap images are not the same size"));
            }

            let image = RawImage2d::from_raw_rgba_reversed(
                &image.to_rgba().into_raw(),
                (size, size)
            );
            let image = glium::Texture2d::new(display, image)
            .context("Failed to create cubemap texture")?;
            
            image.as_surface().blit_whole_color_to(
                &framebuffer,
                &glium::BlitTarget {
                    left: 0,
                    bottom: 0,
                    width: size as i32,
                    height: size as i32,
                },
                glium::uniforms::MagnifySamplerFilter::Linear
            );
        }

        Ok(Texture::Cubemap(tex))
    }
}

pub struct TexturesManager {
    textures: HashMap<String, Texture>,
    default_albedo: CompressedSrgbTexture2d,
    default_rough_metal: CompressedTexture2d,
    default_normal: CompressedTexture2d,
    default_occ: CompressedTexture2d,
}

impl TexturesManager {
    pub fn new(display: &Display) -> TexturesManager {
        TexturesManager {
            textures: HashMap::new(),
            default_albedo: Texture::new_default_albedo(display),
            default_rough_metal: Texture::new_default_rough_metal(display),
            default_normal: Texture::new_default_normal(display),
            default_occ: Texture::new_default_occ(display)
        }
    }

    pub fn get(&self, name: &str) -> Option<&Texture> {
        self.textures.get(name)
    }

    pub fn get_default_albedo(&self) -> &CompressedSrgbTexture2d {
        &self.default_albedo
    }
    pub fn get_default_rough_metal(&self) -> &CompressedTexture2d {
        &self.default_rough_metal
    }
    pub fn get_default_normal(&self) -> &CompressedTexture2d {
        &self.default_normal
    }
    pub fn get_default_occ(&self) -> &CompressedTexture2d {
        &self.default_occ
    }

    pub fn try_load(&mut self, display: &Display, name: &str) -> anyhow::Result<()> {
        match self.textures.get(name) {
            Some(_) => Ok(()),
            None => match Texture::from(name, display) {
                Ok(t) => {self.textures.insert(String::from(name), t); Ok(())},
                Err(e) => Err(e)
            }
        }
    }

    pub fn try_load_cubemap(&mut self, display: &Display, name: &str) -> anyhow::Result<()> {
        match self.textures.get(name) {
            Some(Texture::Cubemap(_)) => Ok(()),
            Some(_) => Err(anyhow!("{} is already loaded as a non-cubemap", name)),
            None => match Texture::cubemap(name, display) {
                Ok(cm) => {self.textures.insert(String::from(name), cm); Ok(())},
                Err(e) => Err(e)
            }
        }
    }
}