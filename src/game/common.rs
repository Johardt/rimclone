use bevy::prelude::*;

/// Add this component to any tile or entity that is collideable
#[derive(Component)]
pub struct Collideable;

#[derive(Component, Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}