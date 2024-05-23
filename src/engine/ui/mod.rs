use wgpu;
use wgpu::util::StagingBelt;

pub mod text;

pub enum UIElement {
    Text(text::Text),
}

pub struct UIRenderer {
    text_renderer: text::TextRenderer,
    elements: Vec<UIElement>
}

impl UIRenderer {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        font_path: &str,
    ) -> Self {
        let text_renderer = text::TextRenderer::new(device, config, font_path);

        Self {
            text_renderer,
            elements: Vec::new()
        }
    }

    pub fn queue(&mut self, element: UIElement) {
        self.elements.push(element);
    }
    
    pub fn draw(&mut self, width: f32, height: f32) {
        for element in self.elements.iter_mut() {
            match element {
                UIElement::Text(t) => {
                    self.text_renderer.draw_text(&t, width, height);       
                }
            }
        }
    }

    pub fn render<'a>(
        &'a mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        texture_view: &wgpu::TextureView,
        staging_belt: &mut StagingBelt,
        width: u32,
        height: u32,
    ) {
        self.text_renderer.render(device, encoder, texture_view, staging_belt, width, height);
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }
}

