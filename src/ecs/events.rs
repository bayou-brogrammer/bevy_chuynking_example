use crate::prelude::*;

#[derive(Debug)]
/// WantsToMove uses Point because Point supports negative values
/// which is important for walking off a map on the W/S edges
pub struct WantsToMove(pub Entity, pub Point);

#[derive(Debug)]
/// (Who is attacking, Who they are attacking)
pub struct WantsToAttack(pub Entity, pub Entity);

#[derive(Debug)]
pub struct WantsToPickupItem(pub Entity, pub Entity);

pub struct EventPlugin;
impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WantsToMove>()
            .add_event::<WantsToAttack>()
            .add_event::<WantsToPickupItem>();
    }
}
