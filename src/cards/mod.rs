use::bevy::prelude::*;

use crate::{components::{Card, CardSuit, CardState, Player, Enemy}, GameTextures, WinSize, CARD_SIZE, PLAYER_CARD_SCALE, ENEMY_CARD_SCALE, cursor::Cursor};

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(insert_selections)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_cards)
            .add_system(move_player_cards)
            .add_system(move_enemy_cards);
    }
}

impl Card {
    pub fn play(&mut self) {
        self.state = CardState::InTable;
    }
    pub fn discard(&mut self) {
        self.state = CardState::InPile;
    }
    pub fn bring_back(&mut self, i: i32) {
        self.state = CardState::InHand(i);
    }
}

#[derive(Resource)]
pub struct CardSelection {
    n: i32,
    pub i: Option<i32>,
}

impl CardSelection {
    fn select_card(&mut self, cursor: &Res<Cursor>, win_size: &Res<WinSize>) {
        let card_width = CARD_SIZE.0 * PLAYER_CARD_SCALE;
        let left = win_size.w / 2. - card_width * self.n as f32/ 2.;

        let i = (cursor.x - left) / card_width;
        let i = i.floor() as i32;
        
        self.i = if i < 0 || i >= self.n {
            Option::None
        } else {
            Option::Some(i)
        };

    }
}

fn insert_selections(mut commands: Commands) {
    let card_selection = CardSelection { n: 5, i: Option::None };
    commands.insert_resource(card_selection);
}

fn match_texture (card_suit: &CardSuit, game_textures: &Res<GameTextures>) -> Handle<Image> {
    match card_suit {
        CardSuit::Emperor => game_textures.emperor.clone(),
        CardSuit::Citizen => game_textures.citizen.clone(),
        CardSuit::Slave   => game_textures.slave.clone()
    }
}

fn spawn_cards(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
) {
    let enemy_cards =
        [CardSuit::Slave, CardSuit::Citizen, CardSuit::Citizen, CardSuit::Citizen, CardSuit::Citizen];

    let player_cards = [CardSuit::Emperor, CardSuit::Citizen, CardSuit::Citizen, CardSuit::Citizen, CardSuit::Citizen].iter().enumerate();
    for (i, suit) in player_cards {
        commands.spawn(SpriteBundle {
            texture: match_texture(&suit, &game_textures),
            transform: Transform {
                scale: Vec3::new(PLAYER_CARD_SCALE, PLAYER_CARD_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Card{ suit: suit.clone(), state: CardState::InHand(i as i32)})
        .insert(Player);
    }

    
    for i in 0..5 {
        commands.spawn(SpriteBundle {
            texture: match_texture(&enemy_cards[i], &game_textures),
            transform: Transform {
                scale: Vec3::new(ENEMY_CARD_SCALE, ENEMY_CARD_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Card{ suit: enemy_cards[i], state: CardState::InHand(i.try_into().unwrap()) })
        .insert(Enemy);
    }

}

fn move_player_cards(
    mut query: Query<(&Card, &mut Transform),With<Player>>,
    cursor: Res<Cursor>,
    win_size: Res<WinSize>,
    mut card_selection: ResMut<CardSelection>
) {

    { 

    let cards_at_play
        = query.iter_mut().filter(|(card, _)| match card.state { CardState::InHand(_) => true, _ => false });

    let cards_at_play: Vec<(&Card, Mut<Transform>)> = cards_at_play.collect();

    let n = cards_at_play.len();
    card_selection.n = n as i32;
    let n = n as f32;

    card_selection.select_card(&cursor, &win_size);

    let radius = 1000.;
    let bottom = -win_size.h/2. + 30. + CARD_SIZE.1*PLAYER_CARD_SCALE/2.;
    let delta_radians = 5. * 3.14159 / 180.;

    for (card, mut transform) in cards_at_play {
        let i = match card.state { CardState::InHand(i) => i, _ => 0 };
        let delta: f32 = delta_radians * ((n-1.)/2. - i as f32);

        // println!("Card {}", i);

        let scale = 
            match card_selection.i {
                Some(j) => (if i == j {1.2} else {1.}) * PLAYER_CARD_SCALE,
                None => PLAYER_CARD_SCALE
            };

        let d = CARD_SIZE.1 * (scale - PLAYER_CARD_SCALE);

        transform.translation = Vec3::new(0., bottom + d, 0.);
        transform.rotation = Quat::from_rotation_z(delta);
        transform.translate_around(
            Vec3::new(0., bottom - radius, 0.),
            Quat::from_rotation_z(delta));
        transform.scale = Vec3::new(scale, scale, 0.);

    }

    } // /Cards at play

    // Cards in table 
    {
        let card_at_table
            = query.iter_mut().filter(|(card, _)| match card.state { CardState::InTable => true, _ => false }).next(); 
        
        if let Some((_, mut transform)) = card_at_table {
            transform.rotation = Quat::from_rotation_z(0.);
            transform.scale = Vec3::new(PLAYER_CARD_SCALE, PLAYER_CARD_SCALE, 1.);
            transform.translation = Vec3::new(0., -10. -CARD_SIZE.1 * PLAYER_CARD_SCALE/2., 0.);
        }

    }
    
    // Cards in pile
    {
        for (_, mut transform) in query.iter_mut().filter(|(card, _)| match card.state { CardState::InPile => true, _ => false }) {
            transform.translation = Vec3::new(-100000000., 0., 0.);
        }
    }
}

fn move_enemy_cards(
    mut query: Query<(&Card, &mut Transform),With<Enemy>>,
    win_size: Res<WinSize>,
) {

    { 

    let cards_at_play
        = query.iter_mut().filter(|(card, _)| match card.state { CardState::InHand(_) => true, _ => false });

    let cards_at_play: Vec<(&Card, Mut<Transform>)> = cards_at_play.collect();

    let n = cards_at_play.len() as f32;

    let radius = -1000.;
    let top = win_size.h/2. - 30. - CARD_SIZE.1*ENEMY_CARD_SCALE/2.;
    let delta_radians = 5. * 3.14159 / 180.;

    for (card, mut transform) in cards_at_play {
        let i = match card.state { CardState::InHand(i) => i, _ => 0 };
        let delta: f32 = delta_radians * ((n-1.)/2. - i as f32);

        // println!("Card {}", i);

        transform.translation = Vec3::new(0., top, 0.);
        transform.rotation = Quat::from_rotation_z(delta);
        transform.translate_around(
            Vec3::new(0., top - radius, 0.),
            Quat::from_rotation_z(delta));

    }

    } // /Cards at play

    // Cards in table 
    {
        let card_at_table
            = query.iter_mut().filter(|(card, _)| match card.state { CardState::InTable => true, _ => false }).next(); 
        
        match card_at_table {
            None => (),
            Some((_, mut transform)) => {
                transform.rotation = Quat::from_rotation_z(0.);
                transform.translation = Vec3::new(0., 10. +CARD_SIZE.1 * ENEMY_CARD_SCALE/2., 0.);
            }
        }

    }

    // Cards in discard
    {
        for (_, mut transform) in query.iter_mut().filter(|(card, _)| match card.state { CardState::InPile => true, _ => false }) {
            transform.translation = Vec3::new(-100000000., 0., 0.);
        }
    }
}