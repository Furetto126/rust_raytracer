mod compute_shader {
    pub mod lib {
        pub mod buffers_interface;
        pub mod buffers_setup;
        mod buffers_update;
    }
    pub mod compute_buffers;
}
mod camera {
    pub mod camera_update;
}
mod window {
    pub mod window;
    pub mod window_shader;
}
mod scene {
    pub mod scene;
    pub mod materials {
        pub mod material;
    }
    pub mod spheres {
        pub mod sphere;
    }
}

use bevy::{prelude::*, window::WindowPlugin};

use camera::camera_update::*;
use compute_shader::{compute_buffers::*, lib::buffers_interface::*};
use scene::scene::ScenePlugin;
use window::{window::*, window_shader::*};

fn main() {
    //better_panic::install();

    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK)).add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // uncomment for unthrottled FPS
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }),
        ComputeBuffersPlugin,
        ComputeBuffersUpdatePlugin,
        WindowPlugin,
        CameraPlugin,
        ScenePlugin,
    ));

    app.run();
}
