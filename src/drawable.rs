use crate::renderer::Renderer;

pub trait Drawable {
    ///
    fn draw(&self, renderer: &mut Renderer);

    ///
    fn draw_wireframe(&self, renderer: &mut Renderer);
}
