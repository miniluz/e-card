use bevy::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_cursor_resource)
            .add_system(update_cursor);
    }
}

#[derive(Resource, Debug)]
pub struct Cursor {
    pub x: f32,
    pub y: f32
}

fn start_cursor_resource (mut commands: Commands) {
    let cursor = Cursor {x: 0.0, y: 0.0};
    commands.insert_resource(cursor);
}

fn update_cursor (mut cursor: ResMut<Cursor>, windows: Res<Windows>) {
    for window in windows.iter() {
        if let Some(vec) = window.cursor_position() {
            cursor.x = vec[0]; cursor.y = vec[1];
        }
    }
}


