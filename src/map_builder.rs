use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut map_builder = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };

        map_builder.fill(TileType::Wall);
        map_builder.build_random_rooms(rng);
        map_builder.build_corridors(rng);
        map_builder.player_start = map_builder.rooms[0].center();

        map_builder
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    /// 生成房间
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        // 生成固定数量房间
        while self.rooms.len() < NUM_ROOMS {
            // 生成随机位置和大小的房间
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10)
            );

            // 判断房间是否与其他房间重叠
            let mut overlay = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlay = true;
                }
            }

            // 如果没有重叠，将房间填充到地图中
            // 如果有重叠，则进行下一轮循环生成新的房间
            if !overlay {
                // 迭代房间中的每一个点；如果点在地图范围内，则将该点设置为地板
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    /// 开凿垂直方向的两个隧道（把两点之间的路径变成地板）
    /// 垂直方向变动的只是 y 坐标，x 坐标不变，所以只需要以一个 x 坐标为基准，遍历 y 坐标，将地板填充到地图中
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{min, max};
        for y in min(y1, y2) ..= max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                 self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    /// 跟 apply_vertical_tunnel 类似，只是这次是水平方向的隧道
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{min, max};
        for x in min(x1, x2) ..= max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    /// 给所有的房间开凿走廊，连通房间
    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        // 克隆一份房间列表
        let mut rooms = self.rooms.clone();
        // 使用房间中点进行排序，减少连接两个相距较远的房间概率
        // 使得连接房间的走廊变短
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        // 因为需要获取前一个房间的中心点，而第零个房间没有前一个房间，所以从第一个房间开始
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            // 随机一种开凿方式，垂直还是水平
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}
