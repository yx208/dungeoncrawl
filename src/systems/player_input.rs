use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera
) {
    if let Some(key) = key {
        // 匹配对应的按键，计算移动的偏移量
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };

        // 如果偏移量不为 0，那么就移动玩家
        if delta.x != 0 || delta.y != 0 {
            // 获取所有的玩家
            let mut player_points = <&mut Point>::query().filter(component::<Player>());
            // 遍历所有的玩家的位置
            player_points.iter_mut(ecs).for_each(|pos| {
                // 计算新的位置
                let destination = *pos + delta;
                // 如果新的位置可以进入，那么就移动
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            });
        }
    }
}
