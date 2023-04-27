use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::{components::{Card, Player, Enemy, CardState, CardSuit}, GameState, cards::CardSelection, GameStage, Turn};

pub struct LoopPlugin;

impl Plugin for LoopPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_timer)
            .add_system(game_loop);
    }
}

#[derive(Resource)]
struct GameTimer {
    timer: Timer,
}

fn add_timer( mut commands: Commands ) {
    let game_timer = GameTimer { timer: Timer::new(Duration::from_secs(1), TimerMode::Once ) };
    commands.insert_resource(game_timer);
}


fn game_loop(
    mut player_query: Query<&mut Card, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<&mut Card, (With<Enemy>, Without<Player>)>,
    mut game_state: ResMut<GameState>,
    card_selection: Res<CardSelection>,
    buttons: Res<Input<MouseButton>>,
    mut game_timer: ResMut<GameTimer>,
    time: Res<Time>,
) {
    let timer = &mut game_timer.timer;
    timer.tick(time.delta());

    match game_state.stage {
        GameStage::Check | GameStage::Set => {
            match game_state.turn {
                Turn::Player => player_turn(player_query, game_state, card_selection, buttons, game_timer),
                Turn::Enemy  => { if timer.just_finished() { enemy_turn( enemy_query, game_state, game_timer) } }
            };
        },
        GameStage::Open => {
            if !timer.just_finished() {
                return;
            }
            println!("Open!");

            let mut player_card =
                player_query.iter_mut().filter(|card| match card.state { CardState::InTable => true, _ => false }).next().unwrap();
            let mut enemy_card = 
                enemy_query.iter_mut().filter(|card| match card.state { CardState::InTable => true, _ => false }).next().unwrap();
            
            println!("{:?}, {:?}", player_card, enemy_card);
            
            match (player_card.suit, enemy_card.suit) {
                (CardSuit::Emperor, CardSuit::Citizen) | (CardSuit::Citizen, CardSuit::Emperor) |
                (CardSuit::Slave  , CardSuit::Citizen) | (CardSuit::Citizen, CardSuit::Slave  ) => {
                    println!("Emperor wins!");
                },
                (CardSuit::Emperor, CardSuit::Slave) | (CardSuit::Slave, CardSuit::Emperor) => {
                    println!("Slave wins!");
                },
                _ => {
                    println!("It is a draw!");
                }
            };
            player_card.discard();
            enemy_card.discard();
            game_timer.timer.reset();
            game_state.advance();
        }
    };

}

fn player_turn(
    mut query: Query<&mut Card, (With<Player>, Without<Enemy>)>,
    mut game_state: ResMut<GameState>,
    card_selection: Res<CardSelection>,
    buttons: Res<Input<MouseButton>>,
    mut game_timer: ResMut<GameTimer>
) {
    if buttons.just_pressed(MouseButton::Left) {
        println!("The left button was pressed. The selected card is {:?}", card_selection.i);
        if let Some(i) = card_selection.i {
            for mut card in query.iter_mut() {
                if let CardState::InHand(j) = card.state {
                    if j > i {
                        card.state = CardState::InHand(j-1);
                    } else if i == j {
                        card.play();
                    }
                }
            };
            game_timer.timer.reset();
            game_state.advance();
        }
    }
}

fn enemy_turn (
    mut query: Query<&mut Card, (With<Enemy>, Without<Player>)>,
    mut game_state: ResMut<GameState>,
    mut game_timer: ResMut<GameTimer>
) {
    println!("It is now the enemy's turn");

    let card_list: Vec<Mut<Card>>
        = query.iter_mut().filter(|card| match card.state { CardState::InHand(_) => true, _ => false }).collect();

    let n = card_list.len() as i32;

    let i = rand::thread_rng().gen_range(0..n);

    for mut card in card_list {
        if let CardState::InHand(j) = card.state {
            if j > i {
                card.state = CardState::InHand(j-1);
            } else if i == j {
                card.play();
            }
        }
    }

    game_state.advance();
    game_timer.timer.reset();
}

