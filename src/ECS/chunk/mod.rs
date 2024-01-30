use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use std::any::*;

pub type ChunkID = u64;

pub enum ChunkRegisterEntityError {
    EntityAlreadyRegistered,
}

pub enum ChunkUnregisterEntityError {
    EntityNotRegistered,
}

pub enum ChunkGetEntityError {
    EntityNotRegistered,
}

#[derive(Send + Sync + Serialize + Deserialize<'static>, Default + Debug + Clone)]
pub struct Chunk {
    chunk_id: ChunkID,
    entities: Vec<Entity>
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            chunk_id: bevy::ecs::entity::Entity::new(),
        }
    }

    pub fn get_chunk_id(&self) -> ChunkID {
        self.chunk_id
    }

    pub fn register_entity(&mut self, entity: Entity) -> Result<(), ()> {
        self.entities.push(entity);
    }

    pub fn unregister_entity(&mut self, entity: Entity) -> Result<(), ()> {
        self.entities.remove(entity);
    }

    pub fn get_entity(&self, entity_id: EntityID) -> Option<&Entity> {
        for entity in &self.entities {
            if entity.get_entity_id() == entity_id {
                return Some(entity);
            }
        }

        None
    }

    pub fn get_entity_mut(&mut self, entity_id: EntityID) -> Option<&mut Entity> {
        for entity in &mut self.entities {
            if entity.get_entity_id() == entity_id {
                return Some(entity);
            }
        }

        None
    }
}