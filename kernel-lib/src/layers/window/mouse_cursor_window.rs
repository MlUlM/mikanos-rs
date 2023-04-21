use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use core::any::Any;

use crate::error::KernelResult;
use crate::gop::console::CONSOLE_BACKGROUND_COLOR;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;
use crate::layers::window::WindowDrawable;

const CURSOR_WIDTH: usize = 15;

const CURSOR_HEIGHT: usize = 24;


const CURSOR_SHAPE: [&[u8; CURSOR_WIDTH]; CURSOR_HEIGHT] = [
    b"@              ",
    b"@@             ",
    b"@.@            ",
    b"@..@           ",
    b"@...@          ",
    b"@....@         ",
    b"@.....@        ",
    b"@......@       ",
    b"@.......@      ",
    b"@........@     ",
    b"@.........@    ",
    b"@..........@   ",
    b"@...........@  ",
    b"@............@ ",
    b"@......@@@@@@@@",
    b"@......@       ",
    b"@....@@.@      ",
    b"@...@ @.@      ",
    b"@..@   @.@     ",
    b"@.@    @.@     ",
    b"@@      @.@    ",
    b"@       @.@    ",
    b"         @.@   ",
    b"         @@@   ",
];


#[derive(Debug)]
pub struct MouseCursorDrawer {
    scale: Vector2D<usize>,
    color: PixelColor,
    widow_size: Size,
}


impl MouseCursorDrawer {
    pub fn new(scale: Vector2D<usize>) -> Self {
        let widow_size = Size::new(CURSOR_WIDTH * scale.x(), CURSOR_HEIGHT * scale.y());
        Self {
            scale,
            color: PixelColor::white(),
            widow_size,
        }
    }

    pub fn set_color(&mut self, color: PixelColor) {
        self.color = color
    }


    pub fn window_size(&self) -> Size {
        self.widow_size
    }

    unsafe fn write_row(
        &mut self,
        pos: Vector2D<usize>,
        writer: &mut dyn PixelWritable,
    ) -> KernelResult {
        for y in 0..CURSOR_HEIGHT {
            for _ in 0..self.scale.y() {
                self.write_line(pos, y, writer)?;
            }
        }

        Ok(())
    }

    unsafe fn write_line(
        &mut self,
        pos: Vector2D<usize>,
        y: usize,
        writer: &mut dyn PixelWritable,
    ) -> KernelResult {
        for x in 0..CURSOR_WIDTH {
            for _ in 0..self.scale.x() {
                writer.write(x + pos.x(), y + pos.y(), &cursor_color_at(x, y))?;
            }
        }

        Ok(())
    }
}


impl WindowDrawable for MouseCursorDrawer {
    fn draw(&mut self, pos: Vector2D<usize>, writer: &mut dyn PixelWritable) -> KernelResult {
        unsafe { self.write_row(pos, writer) }
    }


    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}


impl Default for MouseCursorDrawer {
    fn default() -> Self {
        Self::new(Vector2D::new(1, 1))
    }
}


fn cursor_color_at(x: usize, y: usize) -> PixelColor {
    let c = char::from(CURSOR_SHAPE[y][x]);
    if c == '@' {
        CONSOLE_BACKGROUND_COLOR
    } else if c == '.' {
        PixelColor::white()
    } else {
        PixelColor::black()
    }
}


#[cfg(test)]
mod tests {
    use common_lib::math::size::Size;
    use common_lib::math::vector::Vector2D;

    use crate::gop::pixel::mock_buffer_pixel_writer::MockBufferPixelWriter;
    use crate::layers::window::mouse_cursor_window::{
        cursor_color_at, MouseCursorDrawer, CURSOR_HEIGHT, CURSOR_WIDTH,
    };
    use crate::layers::window::WindowDrawable;

    #[test]
    fn it_no_scale() {
        let window = MouseCursorDrawer::default();
        assert_eq!(window.window_size(), Size::new(CURSOR_WIDTH, CURSOR_HEIGHT));
    }


    #[test]
    fn it_scale2() {
        let window = MouseCursorDrawer::new(Vector2D::new(2, 2));
        assert_eq!(
            window.window_size(),
            Size::new(CURSOR_WIDTH * 2, CURSOR_HEIGHT * 2)
        );
    }


    #[test]
    fn it_write_cursor_not_scale() {
        let mut window = MouseCursorDrawer::new(Vector2D::new(1, 1));
        let mut writer = MockBufferPixelWriter::new(100, 100);
        assert!(window
            .draw(&mut writer)
            .is_ok());
        for y in 0..CURSOR_HEIGHT {
            for x in 0..CURSOR_WIDTH {
                assert_eq!(writer.pixel_at(x, y), cursor_color_at(x, y));
            }
        }
    }
}
