use glyph_brush::{ab_glyph::FontArc, FontId, GlyphPositioner, Section, Text as GlyphText};
use glyph_brush_layout::{ab_glyph::*, Layout, SectionGeometry, SectionText};
use std::fs::File;
use std::io::Read;
use wgpu::util::StagingBelt;
use wgpu_glyph::{GlyphBrush, GlyphBrushBuilder};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, Debug)]
struct TextVertex {
    pos: [f32; 2],
    color: [f32; 4],
}

#[derive(Clone, Copy)]
pub enum TextOrigin {
    Center,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Text {
    pub content: String,
    pub position: cgmath::Point2<f32>,
    pub color: [f32; 4],
    pub origin: TextOrigin,
}

pub struct TextRenderer {
    brush: GlyphBrush<()>,
    font: FontArc,
}

impl TextRenderer {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        font_path: &str,
    ) -> Self {
        // Ensure the font file is a TTF file
        if font_path.split('.').last() != Some("ttf") {
            panic!("Font is not a TTF file");
        }

        let mut file = File::open(font_path).expect("Failed to open font file");
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)
            .expect("Failed to read font file");

        let font = FontArc::try_from_vec(buffer.clone()).expect("Failed to parse font file");

        let brush = GlyphBrushBuilder::using_font(font.clone()).build(device, config.format);

        Self { brush, font }
    }
    fn measure_text(&self, content: &str, scale: f32) -> (f32, f32) {
        let font = self.font.clone();
        let font_size = PxScale::from(scale);

        let glyphs = Layout::default().calculate_glyphs(
            &[&font],
            &SectionGeometry {
                screen_position: (0.0, 0.0),
                bounds: (f32::INFINITY, f32::INFINITY),
            },
            &[SectionText {
                text: content,
                scale: font_size,
                font_id: FontId(0),
            }],
        );

        let width = glyphs
            .iter()
            .map(|g| g.glyph.position.x + g.glyph.scale.x)
            .fold(0.0f32, |a, b| a.max(b));
        let height = glyphs
            .iter()
            .map(|g| g.glyph.position.y + g.glyph.scale.y)
            .fold(0.0f32, |a, b| a.max(b));

        (width, height)
    }

    pub fn draw_text(&mut self, text: &Text, width: f32, height: f32) {
        let scale = 24.0; // You can adjust the scale as needed

        let (text_width, text_height) = self.measure_text(&text.content, scale);

        let screen_position = match text.origin {
            TextOrigin::Left => (
                text.position.x + width / 2.0,
                text.position.y + height / 2.0,
            ),
            TextOrigin::Right => (
                text.position.x + width / 2.0 - text_width,
                text.position.y + height / 2.0,
            ),
            TextOrigin::Center => (
                text.position.x + width / 2.0 - text_width / 2.0,
                text.position.y + height / 2.0 - text_height / 2.0,
            ),
        };

        self.brush.queue(Section {
            screen_position,
            bounds: (width, height),
            text: vec![GlyphText::new(&text.content)
                .with_color([text.color[0], text.color[1], text.color[2], text.color[3]])
                .with_scale(scale)],
            ..Section::default()
        });
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
        self.brush
            .draw_queued(
                device,
                staging_belt,
                encoder,
                texture_view,
                // queue,
                width,
                height,
            )
            .expect("Draw queued");
    }
}
