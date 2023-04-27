use bevy::prelude::*;

mod components;

mod cursor;
use cursor::*;

mod cards;
use cards::*;

mod game_loop;
use game_loop::*;

// region:    --- Textures

const EMPEROR_SPRITE: &str = "emperor.png";
const CITIZEN_SPRITE: &str = "citizen.png";
const SLAVE_SPRITE: &str = "slave.png";
const CARD_SIZE: (f32, f32) = (29., 44.);
const ENEMY_CARD_SCALE: f32 = 3.0;
const PLAYER_CARD_SCALE: f32 = 3.0;

#[derive(Resource)]
pub struct GameTextures {
    emperor: Handle<Image>,
    slave: Handle<Image>,
    citizen: Handle<Image>
}

// endregion: --- Textures

#[derive(Resource)]
pub struct WinSize{
    pub w: f32,
    pub h: f32,
}

#[derive(Clone, Debug)]
enum Turn {
    Player,
    Enemy
}

#[derive(Debug, Clone)]
enum GameStage {
    Check,
    Set,
    Open
}

#[derive(Resource, Debug, Clone)]
pub struct GameState {
    stage: GameStage,
    turn: Turn
}


impl GameState {
    fn advance(&mut self) {
        match self.stage.clone() {
            GameStage::Check => {
                self.stage = GameStage::Set;
                self.turn = match self.turn { Turn::Player => Turn::Enemy, Turn::Enemy => Turn::Player};
            },
            GameStage::Set  => { self.stage = GameStage::Open ; },
            GameStage::Open => { self.stage = GameStage::Check; }
        };
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                    title: "Emperor".to_string(),
                    width: 598.0,
                    height: 676.0,
                    ..default()
                    },
                    ..default()})
                .set(ImagePlugin::default_nearest()))
        .add_startup_system(setup_system)
        .add_plugin(CursorPlugin)
        .add_plugin(CardPlugin)
        .add_plugin(LoopPlugin)
        .run();
}

fn setup_system (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // add winsize
    let win_size = WinSize{w: win_w, h: win_h};
    commands.insert_resource(win_size);

    // add textures
    let game_textures = GameTextures{
        emperor: asset_server.load(EMPEROR_SPRITE),
        slave: asset_server.load(SLAVE_SPRITE),
        citizen: asset_server.load(CITIZEN_SPRITE)
    };
    commands.insert_resource(game_textures);

    // add GameState
    let game_state = GameState{
        stage: GameStage::Check,
        turn: Turn::Player
    };
    commands.insert_resource(game_state);
}

