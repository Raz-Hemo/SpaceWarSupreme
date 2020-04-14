use crate::engine::prelude::*;
use std::collections::HashMap;
use glium::Display;

pub enum Texture {
    Regular(glium::texture::CompressedSrgbTexture2d),
    Cubemap(glium::texture::SrgbCubemap),
}

impl Texture {
    /// This is the "invalid texture" texture - a pink-yellow grid
    pub fn new_default(display: &Display) -> Texture {
        let mut imgbuf = image::ImageBuffer::new(2, 2);
        imgbuf.put_pixel(0, 0, image::Rgb([255, 0, 255]));
        imgbuf.put_pixel(1, 1, image::Rgb([255, 0, 255]));
        imgbuf.put_pixel(1, 0, image::Rgb([255, 255, 0]));
        imgbuf.put_pixel(0, 1, image::Rgb([255, 255, 0]));

        let tex = glium::texture::RawImage2d::from_raw_rgb(imgbuf.into_raw(), (2, 2));
        let tex = glium::texture::CompressedSrgbTexture2d::new(display, tex).expect(
            "Failed to create compressed SRGB texture"
        );
        Texture::Regular(tex)
    }

    pub fn from<P: AsRef<std::path::Path>>(filename: P, display: &Display)
    -> anyhow::Result<Texture> {
        use anyhow::Context;
        let tex = utils::load_image(filename)?.to_rgba();
        let dims = tex.dimensions();
        let tex = glium::texture::RawImage2d::from_raw_rgba(tex.into_raw(), dims);
        Ok(Texture::Regular(
            glium::texture::CompressedSrgbTexture2d::new(display, tex)
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

        let tex = glium::texture::SrgbCubemap::empty(display, size)
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

            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
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
    default_texture: Texture,
}

impl TexturesManager {
    pub fn new(display: &Display) -> TexturesManager {
        TexturesManager {
            textures: HashMap::new(),
            default_texture: Texture::new_default(display),
        }
    }

    pub fn get(&self, name: &str) -> &Texture {
        self.textures.get(name).unwrap_or(&self.default_texture)
    }

    pub fn try_load(&mut self, display: &Display, name: &str) -> anyhow::Result<()> {
        match self.textures.get(name) {
            Some(Texture::Cubemap(_)) => Ok(()),
            Some(_) => Err(anyhow!("{} is already loaded as another type", name)),
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