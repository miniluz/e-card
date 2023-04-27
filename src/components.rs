use bevy::prelude::*;

// region:    --- Card Components

#[derive(Debug, Clone, Copy)]
pub enum CardSuit {
    Emperor,
    Citizen,
    Slave
}

#[derive(Debug, Clone, Copy)]
pub enum CardState {
    InHand(i32),
    InTable,
    InPile,
}

#[derive(Component, Debug)]
pub struct Card {
    pub suit: CardSuit,
    pub state: CardState
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

// endregion: --- Card Components