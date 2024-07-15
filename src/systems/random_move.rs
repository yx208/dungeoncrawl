use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    // 所有可移动的实体位置
    let mut movers = <(&mut Point, &MovingRandomly)>::query();
    for (pos, _) in movers.iter_mut(ecs) {
        let mut rng = RandomNumberGenerator::new();
        // 移动的距离
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        };
        // 实体的位置 + 随机移动的距离
        let destination = destination + *pos;
        if map.can_enter_tile(destination) {
            *pos = destination;
        }
    }
}
