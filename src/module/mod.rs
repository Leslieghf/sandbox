use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use std::any::*;

pub type ModuleID = TypeId;

pub trait Module: Send + Sync + Debug + Clone + 'static {
    fn get_component_type_id(&self) -> TypeId;
}