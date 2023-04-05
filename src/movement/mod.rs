pub mod backend;

use std::collections::HashMap;
use bevy::prelude::{Component, Entity, EventReader, Query};
use bevy_ecs_tilemap::prelude::TilePos;
use bevy_ggf::movement::AvailableMove;

//TODO: Update this to just a guaranteed ordered vec - ordered meaning the first element
/// Resource that holds a Hashmap of [`AvailableMove`] structs. These structs should represent verified
/// valid moves and are updated when the [`MoveEvent::MoveBegin`] event is processed.
#[derive(Clone, Eq, PartialEq, Default, Debug, Component)]
pub struct CurrentMovementInformation {
    pub available_moves: HashMap<TilePos, AvailableMove>,
}

impl CurrentMovementInformation {
    /// Returns true or false if CurrentMovementInformation contains a move at the assigned TilePos
    pub fn contains_move(&self, new_pos: &TilePos) -> bool {
        self.available_moves.contains_key(new_pos)
    }

    /// Clears the moves from the collection
    pub fn clear_moves(&mut self) {
        self.available_moves.clear();
    }
}

pub struct ClearObjectAvailableMoves {
    object: Entity,
}

fn clear_selected_object(
    mut clear_object_reader: EventReader<ClearObjectAvailableMoves>,
    mut current_movement_information: Query<&mut CurrentMovementInformation>,
) {
    for event in clear_object_reader.iter() {
        if let Ok(mut info) = current_movement_information.get_mut(event.object) {
            info.clear_moves();
        }
    }
}