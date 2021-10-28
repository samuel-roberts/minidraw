use image::Rgba;

pub struct RendererConfig {
    pub wireframe: bool,
    pub clear_colour: Rgba<u8>,
}

impl RendererConfig {
    ///
    pub fn default() -> RendererConfig {
        RendererConfig {
            wireframe: false,
            clear_colour: Rgba([0, 0, 0, 255]),
        }
    }

    ///
    pub fn default_wireframe() -> RendererConfig {
        let mut config = RendererConfig::default();
        config.wireframe = true;
        config
    }
}
