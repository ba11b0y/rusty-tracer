use iced::widget::{canvas, column, slider, text, Text};
use iced::{Color, Pixels, Point, Rectangle, Renderer, Theme};
pub fn main() -> iced::Result {
    iced::program("A Rusty Ray Tracer!",
    PictureRenderer::update,
    PictureRenderer::view)
    .run()
}



struct PictureRenderer{
    point_rendering: PointRenderer,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    HeightChanged(u32),
    WidthChanged(u32),
}


struct PointRenderer{
    cache: canvas::Cache,
    width: u32,
    height: u32,
}

impl<Message> canvas::Program<Message> for PointRenderer{
    type State = ();

    fn draw(
            &self,
            _state: &Self::State,
            renderer: &Renderer,
            _theme: &Theme,
            bounds: Rectangle,
            _cursor: iced::mouse::Cursor,
        ) -> Vec<canvas::Geometry> {
            let geom = self.cache.draw(renderer,
                bounds.size(), |frame| {

                    for j in 0..self.height as u32{
                        for i in 0..self.width as u32{
                            let r = i as f32/(self.width-1) as f32;
                            let g = j as f32/(self.height-1) as f32;
                            let b = 0.0;
                
                            let ir = (255.999*r) as u32;
                            let ig = (255.999*g) as u32;
                            frame.fill_text(canvas::Text {
                                position: Point::new(i as f32, j as f32),
                                color: Color::from_rgb8(ir as u8, ig as u8, b as u8),
                                size: Pixels(12.0),
                                font: iced::Font::MONOSPACE,
                                content: ".".into(),
                                horizontal_alignment: iced::alignment::Horizontal::Center,
                                vertical_alignment: iced::alignment::Vertical::Center,
                                line_height: text::LineHeight::Relative(1.0),
                                shaping: iced::widget::text::Shaping::Basic,
                            });
                        }
                    }

            });

            vec![geom]
        }

}

impl Default for PointRenderer{
    fn default() -> Self{
        PointRenderer{
            cache: canvas::Cache::default(),
            width: 256,
            height: 256,
        }
    }
}

impl Default for PictureRenderer{
    fn default() -> Self{
        PictureRenderer{
            point_rendering: PointRenderer::default(),
        }
    }
}


impl PictureRenderer{
    fn view(&self) -> iced::Element<Message> {
        // TODO: implement progress bar
        // let progress_bar = progress_bar(0.0..=100.0,
        //      self.point_rendering.height as f32);
        
        column![
            canvas(&self.point_rendering)
            .width(256)
            .height(256),
            Text::new(format!("Image size: {}x{}", self.point_rendering.width, self.point_rendering.height)),
            Text::new("Slide to adjust image width"),
            slider(100..=256, self.point_rendering.width, Message::WidthChanged),
            Text::new("Slide to adjust image height"),
            slider(100..=256, self.point_rendering.height, Message::HeightChanged),
        ].into()
        
    }

    fn update(&mut self, message: Message){
        match message {
            Message::WidthChanged(width) => {
                self.point_rendering.width = width;
            }
            Message::HeightChanged(height) => {
                self.point_rendering.height = height;
            }
        }
        self.redraw();
    }

    fn redraw(&mut self) {
        self.point_rendering.cache.clear();
    }
}