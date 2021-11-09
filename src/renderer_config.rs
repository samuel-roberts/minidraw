use image::Rgba;

pub struct RendererConfig {
    pub wireframe: bool,
    pub clear_colour: Rgba<u8>,
    pub field_of_view: f32,
}

impl RendererConfig {
    ///
    pub fn default() -> RendererConfig {
        RendererConfig {
            wireframe: false,
            clear_colour: Rgba([0, 0, 0, 255]),
            field_of_view: std::f32::consts::PI / 2.0,
        }
    }

    ///
    pub fn default_wireframe() -> RendererConfig {
        let mut config = RendererConfig::default();
        config.wireframe = true;
        config
    }
}
