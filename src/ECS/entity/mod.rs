use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use std::any::*;
use bevy::ecs::entity::Entity as BevyEntity;

#[derive(Send + Sync + Serialize + Deserialize<'static>, Default + Debug + Clone)]
pub struct Entity {
    bevy_entity: BevyEntity,
    components: Vec<Box<dyn Component>>,
}

impl Entity {
    pub fn new() -> Entity {
        Self {
            bevy_entity: BevyEntity::new(),
        }
    }
    
    pub fn get_entity_id(&self) -> BevyEntity {
        self.entity_id
    }

    pub fn register_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }

    pub fn unregister_component(&mut self, component: Box<dyn Component>) {
        self.components.remove(component);
    }
}