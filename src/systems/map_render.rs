use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    let offset = Point::new(camera.left_x, camera.top_y);
    // 注意这里，遍历的是相机的范围，而不是整个地图
    for y in camera.top_y ..= camera.bottom_y {
        for x in camera.left_x .. camera.right_x {
            let pt = Point::new(x, y);
            if map.in_bounds(pt) {
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Wall => to_cp437('#'),
                    TileType::Floor => to_cp437('.'),
                };
                draw_batch.set(pt - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}
