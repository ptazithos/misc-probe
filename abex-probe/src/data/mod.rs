use aoe2_probe::parse::Token;
use bevy::prelude::Component;

#[derive(Component)]
pub struct TriggerComponent {
    pub id: usize,
    pub token: Token,
}

#[derive(Component)]
pub struct TokenComponent;
