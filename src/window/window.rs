use bevy::{
    prelude::*,
    render::{MainWorld, RenderApp},
};

use crate::update_camera_texture_size;

pub struct WindowPlugin;
impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        init_resources(&mut app.world);

        app.add_event::<ResizedWindowEvent>();
        app.add_systems(
            Update,
            (update_window_size_main, update_camera_texture_size),
        );

        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(ExtractSchedule, update_window_size_render);
    }
}

#[derive(Resource, Clone, Copy)]
pub struct WindowSize(pub Vec2);

#[derive(Resource, Clone, Copy)]
pub struct PreviousWindowSize(pub Vec2);

#[derive(Event)]
pub struct ResizedWindowEvent();

fn update_window_size_main(
    mut commands: Commands,
    windows: Query<&Window>,
    previous_size: Res<PreviousWindowSize>,
    mut ev_window_resized: EventWriter<ResizedWindowEvent>
) {
    let window = windows.single();
    let resolution = Vec2::new(
        window.resolution.width(),
        window.resolution.height(),
    );

    commands.insert_resource(WindowSize(resolution));

    if previous_size.0 != resolution {
        commands.insert_resource(PreviousWindowSize(resolution));

        ev_window_resized.send(ResizedWindowEvent());
    }
}

fn update_window_size_render(mut commands: Commands, world: Res<MainWorld>) {
    let resolution = world.resource::<WindowSize>();
    commands.insert_resource(*resolution);
}

fn init_resources(world: &mut World) {
    world.insert_resource(WindowSize(Vec2::new(1280.0, 720.0)));
    world.insert_resource(PreviousWindowSize(Vec2::new(1280.0, 720.0)));
}
