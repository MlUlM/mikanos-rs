use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transform2D;
use kernel_lib::error::KernelResult;
use kernel_lib::layers::count::CountLayer;
use kernel_lib::layers::layer_key::LayerKey;
use kernel_lib::layers::window::WindowLayer;

use crate::layers::WINDOW_COUNT;

pub(crate) fn time_count_window(
    config: FrameBufferConfig,
    title: &str,
    pos: Vector2D<usize>,
    layer_key: &str,
) -> KernelResult<LayerKey> {
    let transform = Transform2D::new(pos, Size::new(160, 52));

    let window = WindowLayer::new(config, transform.clone(), title)
        .new_layer(count_layer(config, &transform, layer_key)?)?
        .into_enum()
        .into_layer_key(WINDOW_COUNT);

    Ok(window)
}


fn count_layer(
    config: FrameBufferConfig,
    window_transform: &Transform2D,
    key: &str,
) -> KernelResult<LayerKey> {
    let size = window_transform.size() - Size::new(20, 0);
    let pos = Vector2D::new(
        window_transform
            .size()
            .width()
            / 2
            - 32,
        0,
    );

    let count = CountLayer::new(
        config,
        Transform2D::new(pos, size.unwrap_or(window_transform.size())),
    )?;

    Ok(count
        .into_enum()
        .into_layer_key(key))
}
