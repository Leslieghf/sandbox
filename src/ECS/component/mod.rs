use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use std::any::*;

pub type ComponentID = u64;

pub trait Component: Send + Sync + Serialize + Deserialize<'static> + Default + Debug + Clone + 'static {
    fn get_component_type_id(&self) -> TypeId;
    fn get_component_id(&self) -> ComponentID;
}