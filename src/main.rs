mod map;
mod map_builder;
mod camera;
mod components;
mod spawner;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}

use prelude::*;

struct State {
    ecs: World,
    resource: Resources,
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resource = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        resource.insert(map_builder.map);
        resource.insert(Camera::new(map_builder.player_start));
        spawn_player(&mut ecs, map_builder.player_start);

        Self {
            ecs,
            resource,
            systems: build_scheduler()
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        // 把当前 tick 的键盘事件插入到资源中，使得所有系统可以访问
        self.resource.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resource);
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(60.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("material.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "material.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "material.png")
        .build()?;

    main_loop(context, State::new())
}
