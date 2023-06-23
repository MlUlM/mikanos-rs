use alloc::string::String;

use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transformable2D;
use kernel_lib::gop::pixel::pixel_color::PixelColor;
use kernel_lib::layers::cursor::cursor_colors::CursorColors;
use pci::class_driver::mouse::MouseButton;
use pci::class_driver::mouse::subscribable::MouseSubscribable;

use crate::layers::{LAYERS, MOUSE_LAYER_KEY};

#[derive(Debug, Clone)]
pub struct MouseSubscriber;


impl MouseSubscriber {
    pub fn new() -> Self {
        Self {}
    }
}


impl MouseSubscribable for MouseSubscriber {
    fn subscribe(
        &self,
        prev_cursor: Vector2D<usize>,
        current_cursor: Vector2D<usize>,
        prev_button: Option<MouseButton>,
        button: Option<MouseButton>,
    ) -> anyhow::Result<()> {
        update_cursor_layer(current_cursor, button)?;
        update_draggable_layer(prev_cursor, current_cursor, prev_button, button)?;

        Ok(())
    }
}


fn update_draggable_layer(
    prev_cursor: Vector2D<usize>,
    current_cursor: Vector2D<usize>,
    prev_button: Option<MouseButton>,
    button: Option<MouseButton>,
) -> anyhow::Result<()> {
    let prev_drag = prev_button.is_some_and(|b| matches!(b, MouseButton::Button1));
    let current_drag = button.is_some_and(|b| matches!(b, MouseButton::Button1));

    if prev_drag && current_drag {
        let relative = current_cursor.relative(prev_cursor);

        if let Some(window_key) =
            draggable_layer_key(&prev_cursor).or(draggable_layer_key(&current_cursor))
        {
            let mut layers = LAYERS.lock();

            layers
                .bring_to_front(window_key.as_str())
                .map_err(|e| anyhow::anyhow!("{e:?}"))?;

            layers
                .bring_to_front(MOUSE_LAYER_KEY)
                .map_err(|e| anyhow::anyhow!("{e:?}"))?;

            layers
                .update_layer(window_key.as_str(), |layer| {
                    layer
                        .move_to_relative(relative)
                        .unwrap_or(());
                })
                .map_err(|e| anyhow::anyhow!("{e:?}"))?;
        }
    }
    Ok(())
}


fn draggable_layer_key(current_cursor: &Vector2D<usize>) -> Option<String> {
    LAYERS
        .lock()
        .find_window_layer_by_pos(current_cursor)
        .map(String::from)
}


fn update_cursor_layer(
    current_cursor: Vector2D<usize>,
    button: Option<MouseButton>,
) -> anyhow::Result<()> {
    LAYERS
        .try_lock()
        .ok_or(anyhow::anyhow!("Failed Lock Layers"))?
        .update_layer(MOUSE_LAYER_KEY, |layer| {
            let color: PixelColor = cursor_color(button);
            if let Ok(cursor) = layer.require_cursor() {
                cursor.move_to(current_cursor);

                cursor.set_color(CursorColors::default().change_foreground(color))
            }
        })
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}


fn cursor_color(button: Option<MouseButton>) -> PixelColor {
    button
        .map(|b| match b {
            MouseButton::Button1 => PixelColor::yellow(),
            MouseButton::Button2 => PixelColor::new(0x13, 0xA9, 0xDB),
            MouseButton::Button3 => PixelColor::new(0x35, 0xFA, 0x66),
            _ => PixelColor::white(),
        })
        .unwrap_or(PixelColor::white())
}
