// Modules


// Local imports


// Internal imports
use crate::math::*;
use crate::chunking::chunk::*;

// External imports
use num_bigint::BigUint;
use std::sync::{Arc, Mutex, RwLock};

// Static variables


// Constant variables


// Types


// Enums
pub enum Cluster {
    Registered {
        registration: Arc<RwLock<ClusterRegistration>>,
    },
    MetadataLoaded {
        registration: Arc<RwLock<ClusterRegistration>>,
        metadata: Arc<Mutex<ClusterMetadata>>,
    },
    DataLoaded {
        registration: Arc<RwLock<ClusterRegistration>>,
        metadata: Arc<Mutex<ClusterMetadata>>,
        data: Arc<Mutex<ClusterData>>,
    },
}

pub enum ClusterLoadState {
    Registered,
    MetadataLoaded,
    DataLoaded,
}

// Structs
#[derive(Clone, Debug)]
pub struct ClusterID {
    global_id_base10: BigUint,
    global_id_base57: String,
}

pub struct ClusterRegistration {
    id: ClusterID,
}

pub struct ClusterMetadata {
    placeholder_metadata: Option<i32>,
}

pub struct ClusterData {
    placeholder_data: Option<i32>,
}

pub struct ClusterManager {
    registered_clusters: Arc<Mutex<Vec<Cluster>>>,
}

// Implementations
impl ClusterID {
    pub fn from_chunk_id(chunk_id: ChunkID) -> ClusterID {
        let base10_id = chunk_id.get_global_id_base10() / BigUint::from(100u32);
        let base57_id = BASE57_CONVERTER
            .convert_to_base57(base10_id.clone())
            .unwrap();

        ClusterID {
            global_id_base10: base10_id,
            global_id_base57: base57_id,
        }
    }

    pub fn get_global_id_base10(&self) -> &BigUint {
        return &self.global_id_base10;
    }

    pub fn get_global_id_base57(&self) -> &String {
        return &self.global_id_base57;
    }
}

impl PartialEq for ClusterID {
    fn eq(&self, other: &Self) -> bool {
        self.global_id_base10 == other.global_id_base10
    }
}

impl ClusterRegistration {
    fn new(id: ClusterID) -> ClusterRegistration {
        ClusterRegistration { id: id }
    }

    fn get_id(&self) -> &ClusterID {
        return &self.id;
    }
}

impl ClusterMetadata {
    fn new() -> ClusterMetadata {
        ClusterMetadata {
            placeholder_metadata: None,
        }
    }

    fn get_placeholder_metadata(&self) -> Option<i32> {
        return self.placeholder_metadata;
    }

    fn set_placeholder_metadata(&mut self, placeholder_metadata: Option<i32>) {
        self.placeholder_metadata = placeholder_metadata;
    }
}

impl ClusterData {
    fn new() -> ClusterData {
        ClusterData {
            placeholder_data: None,
        }
    }

    fn get_placeholder_data(&self) -> Option<i32> {
        return self.placeholder_data;
    }

    fn set_placeholder_data(&mut self, placeholder_data: Option<i32>) {
        self.placeholder_data = placeholder_data;
    }
}

impl Cluster {
    fn new(id: ClusterID) -> Self {
        Cluster::Registered {
            registration: Arc::new(RwLock::new(ClusterRegistration::new(id))),
        }
    }

    fn load_metadata(&mut self, metadata: ClusterMetadata) -> Result<(), String> {
        match self {
            Cluster::Registered { .. } => {
                *self = Cluster::MetadataLoaded {
                    registration: self.get_registration().clone(),
                    metadata: Arc::new(Mutex::new(metadata)),
                };
                Ok(())
            },
            Cluster::MetadataLoaded { .. } => {
                Err("Cannot load metadata: Metadata is already loaded.".to_string())
            }
            Cluster::DataLoaded { .. } => {
                Err("Cannot load metadata: Both metadata and data are already loaded.".to_string())
            }
        }
    }

    fn load_data(&mut self, data: ClusterData) -> Result<(), String> {
        match self {
            Cluster::Registered { .. } => {
                Err("Cannot load data: Metadata must be loaded first.".to_string())
            }
            Cluster::MetadataLoaded { .. } => {
                *self = Cluster::DataLoaded {
                    registration: self.get_registration().clone(),
                    metadata: self.get_metadata().unwrap().clone(),
                    data: Arc::new(Mutex::new(data)),
                };
                Ok(())
            },
            Cluster::DataLoaded { .. } => {
                Err("Cannot load data: Data is already loaded.".to_string())
            }
        }
    }

    fn unload_metadata(&mut self) -> Result<(), String> {
        match self {
            Cluster::Registered { .. } => {
                Err("Cannot unload metadata: No metadata is loaded.".to_string())
            }
            Cluster::MetadataLoaded { .. } => {
                *self = Cluster::Registered {
                    registration: self.get_registration().clone(),
                };
                Ok(())
            }
            Cluster::DataLoaded { .. } => {
                Err("Cannot unload metadata: Data must be unloaded first.".to_string())
            }
        }
    }

    fn unload_data(&mut self) -> Result<(), String> {
        match self {
            Cluster::Registered { .. } => {
                Err("Cannot unload data: Neither metadata nor data are loaded.".to_string())
            }
            Cluster::MetadataLoaded { .. } => {
                Err("Cannot unload data: No data is loaded.".to_string())
            }
            Cluster::DataLoaded { .. } => {
                *self = Cluster::MetadataLoaded {
                    registration: self.get_registration().clone(),
                    metadata: self.get_metadata().unwrap().clone(),
                };
                Ok(())
            }
        }
    }

    fn get_registration(&self) -> &Arc<RwLock<ClusterRegistration>> {
        match self {
            Cluster::Registered { registration } => registration,
            Cluster::MetadataLoaded { registration, .. } => registration,
            Cluster::DataLoaded { registration, .. } => registration,
        }
    }

    fn get_metadata(&self) -> Result<&Arc<Mutex<ClusterMetadata>>, String> {
        match self {
            Cluster::Registered { .. } => Err("No metadata is loaded.".to_string()),
            Cluster::MetadataLoaded { metadata, .. } => Ok(metadata),
            Cluster::DataLoaded { metadata, .. } => Ok(metadata),
        }
    }

    fn get_data(&self) -> Result<&Arc<Mutex<ClusterData>>, String> {
        match self {
            Cluster::Registered { .. } => Err("No data is loaded.".to_string()),
            Cluster::MetadataLoaded { .. } => Err("No data is loaded.".to_string()),
            Cluster::DataLoaded { data, .. } => Ok(data),
        }
    }

    fn get_load_state(&self) -> ClusterLoadState {
        match self {
            Cluster::Registered { .. } => ClusterLoadState::Registered,
            Cluster::MetadataLoaded { .. } => ClusterLoadState::MetadataLoaded,
            Cluster::DataLoaded { .. } => ClusterLoadState::DataLoaded,
        }
    }
}

impl ClusterManager {
    pub fn new() -> ClusterManager {
        ClusterManager {
            registered_clusters: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

// Module Functions
