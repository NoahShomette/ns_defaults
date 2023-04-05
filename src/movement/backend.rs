// main events
// MoveBegin
// MoveCalculated (Vec<TilePos>)
// MoveObject
// MoveComplete

use crate::movement::CurrentMovementInformation;
use bevy::ecs::system::SystemState;
use bevy::prelude::{Commands, Entity, EventReader, Mut, Query, World};
use bevy_ecs_tilemap::prelude::TilePos;
use bevy_ggf::movement::backend::MovementNodes;
use bevy_ggf::movement::{AvailableMove, MoveEvent, MovementSystem};
use bevy_ggf::object::ObjectId;
use std::collections::HashMap;

/// Handles all MoveBegin events. Uses the MovementSystem resource to calculate the move and update
/// the CurrentMoveInformation resource
pub(crate) fn handle_move_begin_events(mut world: &mut World) {
    let mut move_events_vec: Vec<MoveEvent> = vec![];
    let mut system_state: SystemState<EventReader<MoveEvent>> = SystemState::new(world);
    let mut move_events = system_state.get_mut(world);

    for event in move_events.iter() {
        if let MoveEvent::MoveBegin {
            object_moving,
            on_map,
        } = event
        {
            move_events_vec.push(MoveEvent::MoveBegin {
                object_moving: *object_moving,
                on_map: *on_map,
            });
        }
    }
    let mut moves: HashMap<Entity, MovementNodes> = HashMap::new();

    world.resource_scope(|world, movement_system: Mut<MovementSystem>| {
        for event in move_events_vec {
            if let MoveEvent::MoveBegin {
                object_moving,
                on_map,
            } = event
            {
                let mut system_state: SystemState<Query<(Entity, &ObjectId)>> =
                    SystemState::new(world);
                let mut object_query = system_state.get_mut(world);
                let Some((entity, _)) = object_query
                    .iter_mut()
                    .find(|(_, id)| id == &&object_moving) else {
                    continue;
                };

                let move_info = movement_system.movement_calculator.calculate_move(
                    &movement_system.tile_move_checks,
                    movement_system.map_type,
                    on_map,
                    entity,
                    world,
                );

                moves.insert(entity, move_info);
            }
        }
    });

    let mut system_state: SystemState<Commands> = SystemState::new(world);
    let mut commands = system_state.get_mut(world);

    for (entity, move_nodes) in moves.iter() {
        if !move_nodes.move_nodes.is_empty() {
            let mut moves: HashMap<TilePos, AvailableMove> = HashMap::new();

            for (tile_pos, move_node) in move_nodes.move_nodes.iter() {
                if move_node.valid_move {
                    moves.insert(*tile_pos, AvailableMove::from(*move_node));
                }
            }

            commands.entity(*entity).insert(CurrentMovementInformation {
                available_moves: moves,
            });
        }
    }

    system_state.apply(world);
}
