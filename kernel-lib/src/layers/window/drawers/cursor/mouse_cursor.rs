use core::any::Any;

use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::Transform2D;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::writer::pixel_writable::PixelWritable;
use crate::layers::window::drawers::cursor::cursor_buffer::CursorBuffer;
use crate::layers::window::WindowDrawable;

#[derive(Debug, Clone)]
pub struct MouseCursorDrawer {
    cursor_buff: CursorBuffer,
    color: PixelColor,
    border_color: PixelColor,
}


impl MouseCursorDrawer {
    pub fn new(scale: Vector2D<usize>, color: PixelColor, border_color: PixelColor) -> Self {
        Self {
            cursor_buff: CursorBuffer::new(scale),
            color,
            border_color,
        }
    }

    pub fn cursor_size(&self) -> Size {
        self.cursor_buff.size()
    }


    pub fn set_color(&mut self, color: PixelColor) {
        self.color = color
    }


    pub fn set_border_color(&mut self, border_color: PixelColor) {
        self.border_color = border_color;
    }
}


impl WindowDrawable for MouseCursorDrawer {
    fn draw_in_area(
        &mut self,
        window_transform: &Transform2D,
        draw_rect: &Rectangle<usize>,
        writer: &mut dyn PixelWritable,
    ) -> KernelResult {
        for pixel in self
            .cursor_buff
            .cursor_pixels_checked(window_transform, self.color, self.border_color)?
            .filter(|p| draw_rect.with_in_pos(&p.pos()))
        {
            if let Some(color) = pixel.color() {
                unsafe { writer.write(pixel.pos().x(), pixel.pos().y(), &color)? };
            }
        }

        Ok(())
    }


    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}


impl Default for MouseCursorDrawer {
    fn default() -> Self {
        Self::new(Vector2D::unit(), PixelColor::white(), PixelColor::black())
    }
}


#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use common_lib::math::size::Size;
    use common_lib::math::vector::Vector2D;
    use common_lib::transform::builder::Transform2DBuilder;

    use crate::gop::pixel::pixel_color::PixelColor;
    use crate::gop::pixel::writer::mock_buffer_pixel_writer::MockBufferPixelWriter;
    use crate::layers::window::drawers::cursor::mouse_cursor::MouseCursorDrawer;
    use crate::layers::window::drawers::WindowDrawable;

    #[test]
    fn it_write_cursor_not_scale() {
        let cursor_color = PixelColor::blue();
        let border_color = PixelColor::yellow();
        let mut drawer = MouseCursorDrawer::new(Vector2D::unit(), cursor_color, border_color);
        let mut writer = MockBufferPixelWriter::new(
            drawer.cursor_size().width() * 4,
            drawer.cursor_size().height() * 4,
        );

        let transform = Transform2DBuilder::new()
            .size(Size::new(
                drawer.cursor_size().width() * 4,
                drawer.cursor_size().height() * 4,
            ))
            .build();

        assert!(drawer
            .draw(&transform, &mut writer)
            .is_ok());

        let pixels = drawer
            .cursor_buff
            .cursor_pixels_checked(&transform, cursor_color, border_color)
            .unwrap();

        let points: Vec<Vector2D<usize>> = drawer
            .cursor_size()
            .points()
            .collect();

        points
            .into_iter()
            .zip(pixels)
            .for_each(|(point, cursor_pixel)| {
                let actual = writer.pixel_at(point.x(), point.y());

                let expect = cursor_pixel
                    .color()
                    .unwrap_or(PixelColor::black());

                assert_eq!(actual, expect);
            });
    }
}
