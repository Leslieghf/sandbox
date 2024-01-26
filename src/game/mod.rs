use std::collections::HashMap;
use std::sync::*;
use std::fmt::Debug;
use std::any::*;
use lazy_static::lazy_static;
use log::*;
use serde::*;

pub type GameID = u64;

#[derive(Debug, Clone)]
pub struct GameInfo {
	game_id: GameID,
	name: String,
	installed_game_modules: HashMap<GameModuleID, GameModuleInfo>,
}

impl PartialEq for GameInfo {
	fn eq(&self, other: &Self) -> bool {
		self.game_id == other.game_id
	}

}

impl Eq for GameInfo {}

impl Default for GameInfo {
	fn default() -> Self {
		Self {
			game_id: 0,
			name: String::new(),
			installed_game_modules: HashMap::new(),
		}
	}

}

impl GameInfo {
	pub fn get_game_id(&self) -> GameID {
		self.game_id
	}

	pub fn get_name(&self) -> &str {
		&self.name
	}

	pub fn get_installed_game_modules(&self) -> &HashMap<GameModuleID, GameModuleInfo> {
		&self.installed_game_modules
	}

	pub fn is_compatible_with_config(&self, game_config: &GameConfig) -> bool {
		for (game_module_id, _) in self.installed_game_modules.iter() {
			if !game_config.installed_game_modules.contains_key(game_module_id) {
				return false;
			}
		}

		return true;
	}

	pub fn is_compatible_with_state(&self, game_state: &GameState) -> bool {
		for (game_module_id, _) in self.installed_game_modules.iter() {
			if !game_state.installed_game_modules.contains_key(game_module_id) {
				return false;
			}
		}

		return true;
	}
}

pub struct GameConfig {
	installed_game_modules: HashMap<GameModuleID, Box<dyn GameModuleConfig + Sync + Send>>,
}

impl Default for GameConfig {
	fn default() -> Self {
		Self {
			installed_game_modules: HashMap::new(),
		}
	}

}

impl GameConfig {
	pub fn is_game_module_installed(&self, game_module_id: GameModuleID) -> bool {
		self.installed_game_modules.contains_key(&game_module_id)
	}

	pub fn get_installed_game_module(&self, game_module_id: GameModuleID) -> Option<&Box<dyn GameModuleConfig + Sync + Send>> {
		self.installed_game_modules.get(&game_module_id)
	}

	pub fn get_installed_game_modules(&self) -> &HashMap<GameModuleID, Box<dyn GameModuleConfig + Sync + Send>> {
		&self.installed_game_modules
	}
}

pub struct GameState {
	installed_game_modules: HashMap<GameModuleID, Box<dyn GameModuleState + Sync + Send>>,
	has_been_joined: bool,
}

impl Default for GameState {
	fn default() -> Self {
		Self {
			installed_game_modules: HashMap::new(),
			has_been_joined: false,
		}
	}
}

impl GameState {
	pub fn is_game_module_installed(&self, game_module_id: GameModuleID) -> bool {
		self.installed_game_modules.contains_key(&game_module_id)
	}

	pub fn get_installed_game_module(&self, game_module_id: GameModuleID) -> Option<&Box<dyn GameModuleState + Sync + Send>> {
		self.installed_game_modules.get(&game_module_id)
	}

	pub fn get_installed_game_modules(&self) -> &HashMap<GameModuleID, Box<dyn GameModuleState + Sync + Send>> {
		&self.installed_game_modules
	}
}

pub type GameModuleTypeID = TypeId;
pub type GameModuleID = u64;

#[derive(Debug, Clone)]
pub struct GameModuleInfo {
    game_module_type_id: GameModuleTypeID,
    game_module_id: GameModuleID,
}

impl Default for GameModuleInfo {
	fn default() -> Self {
		Self {
			game_module_type_id: TypeId::of::<()>(),
			game_module_id: 0,
		}
	}
}

impl GameModuleInfo {
    pub fn get_game_module_type_id(&self) -> GameModuleTypeID {
        self.game_module_type_id
    }

    pub fn get_game_module_id(&self) -> GameModuleID {
        self.game_module_id
    }
}

pub trait GameModuleType {
	type ConfigType: 'static + GameModuleConfig + Sync + Send + Serialize + for<'de> Deserialize<'de>;
	type StateType: 'static + GameModuleState + Sync + Send + Serialize + for<'de> Deserialize<'de>;
}

pub trait GameModuleConfig {
	fn default() -> Box<dyn GameModuleConfig + Sync + Send> where Self: Sized;
	fn box_clone(&self) -> Box<dyn GameModuleConfig + Sync + Send>;
	fn new_default_state(&self) -> Box<dyn GameModuleState + Sync + Send>;
}

impl Clone for Box<dyn GameModuleConfig + Sync + Send> {
	fn clone(&self) -> Self {
		self.box_clone()
	}
}

pub trait GameModuleState {
	fn default() -> Box<dyn GameModuleState + Sync + Send> where Self: Sized;
	fn box_clone(&self) -> Box<dyn GameModuleState + Sync + Send>;
	fn new_default_config(&self) -> Box<dyn GameModuleConfig + Sync + Send>;
}

impl Clone for Box<dyn GameModuleState + Sync + Send> {
	fn clone(&self) -> Self {
		self.box_clone()
	}
}

#[derive(Debug, Clone)]
pub enum GameModuleLoadGameModuleConfigError {
	GameModuleConfigAlreadyLoaded,
}

#[derive(Debug, Clone)]
pub enum GameModuleLoadGameModuleStateError {
	GameModuleConfigStillUnloaded,
	GameModuleStateAlreadyLoaded,
}

#[derive(Debug, Clone)]
pub enum GameModuleUnloadGameModuleConfigError {
	GameModuleConfigAlreadyUnloaded,
	GameModuleStateStillLoaded,
}

#[derive(Debug, Clone)]
pub enum GameModuleUnloadGameModuleStateError {
	GameModuleStateAlreadyUnloaded,
}

pub enum GameModule {
    GameModuleInfoLoaded {
        game_module_info: GameModuleInfo,
    },
    GameModuleConfigLoaded {
        game_module_info: GameModuleInfo,
        game_module_config: Box<dyn GameModuleConfig + Sync + Send>,
    },
    GameModuleStateLoaded {
        game_module_info: GameModuleInfo,
        game_module_config: Box<dyn GameModuleConfig + Sync + Send>,
        game_module_state: Box<dyn GameModuleState + Sync + Send>,
    },
}

impl GameModule {
	fn new<T: 'static + GameModuleType>() -> GameModule {
		let game_module_manager = GAME_MODULE_MANAGER.clone();
		let mut game_module_manager = match game_module_manager.lock() {
			Ok(game_module_manager) => game_module_manager,
			Err(_) => panic!("Failed to lock game module manager!"),
		};

		GameModule::GameModuleInfoLoaded {
			game_module_info: GameModuleInfo {
				game_module_type_id: TypeId::of::<T>(),
				game_module_id: game_module_manager.get_unused_game_module_id(),
			},
		}
	}

	pub fn load_game_module_config<T: 'static + GameModuleType>(&mut self, game_module_config: Box<T::ConfigType>) -> Result<(), GameModuleLoadGameModuleConfigError> {
		match self {
			GameModule::GameModuleInfoLoaded { game_module_info } => {
				let game_module_info = std::mem::take(game_module_info);

				*self = GameModule::GameModuleConfigLoaded {
					game_module_info,
					game_module_config,
				};

				Ok(())
			},
			GameModule::GameModuleConfigLoaded { .. } => Err(GameModuleLoadGameModuleConfigError::GameModuleConfigAlreadyLoaded),
			GameModule::GameModuleStateLoaded { .. } => Err(GameModuleLoadGameModuleConfigError::GameModuleConfigAlreadyLoaded),
		}
	}

	pub fn load_game_module_state<T: 'static + GameModuleType>(&mut self, game_module_state: Box<T::StateType>) -> Result<(), GameModuleLoadGameModuleStateError> {
		match self {
			GameModule::GameModuleInfoLoaded { .. } => Err(GameModuleLoadGameModuleStateError::GameModuleConfigStillUnloaded),
			GameModule::GameModuleConfigLoaded { game_module_info, game_module_config } => {
				let game_module_info = std::mem::take(game_module_info);
				let game_module_config = std::mem::replace(game_module_config, T::ConfigType::default());

				*self = GameModule::GameModuleStateLoaded {
					game_module_info,
					game_module_config,
					game_module_state,
				};

				Ok(())
			},
			GameModule::GameModuleStateLoaded { .. } => Err(GameModuleLoadGameModuleStateError::GameModuleStateAlreadyLoaded),
		}
	}

	pub fn unload_game_module_config<T: 'static + GameModuleType>(&mut self) -> Result<(), GameModuleUnloadGameModuleConfigError> {
		match self {
			GameModule::GameModuleInfoLoaded { .. } => Err(GameModuleUnloadGameModuleConfigError::GameModuleConfigAlreadyUnloaded),
			GameModule::GameModuleConfigLoaded { game_module_info, .. } => {
				let game_module_info = std::mem::take(game_module_info);

				*self = GameModule::GameModuleInfoLoaded {
					game_module_info,
				};

				Ok(())
			},
			GameModule::GameModuleStateLoaded { .. } => Err(GameModuleUnloadGameModuleConfigError::GameModuleStateStillLoaded),
		}
	}

	pub fn unload_game_module_state<T: 'static + GameModuleType>(&mut self) -> Result<(), GameModuleUnloadGameModuleStateError> {
		match self {
			GameModule::GameModuleInfoLoaded { .. } => Err(GameModuleUnloadGameModuleStateError::GameModuleStateAlreadyUnloaded),
			GameModule::GameModuleConfigLoaded { .. } => Err(GameModuleUnloadGameModuleStateError::GameModuleStateAlreadyUnloaded),
			GameModule::GameModuleStateLoaded { game_module_info, game_module_config, .. } => {
				let game_module_info = std::mem::take(game_module_info);
				let game_module_config = std::mem::replace(game_module_config, T::ConfigType::default());

				*self = GameModule::GameModuleConfigLoaded {
					game_module_info,
					game_module_config,
				};

				Ok(())
			},
		}
	}

	pub fn get_game_module_info(&self) -> &GameModuleInfo {
		match self {
			GameModule::GameModuleInfoLoaded { game_module_info } => game_module_info,
			GameModule::GameModuleConfigLoaded { game_module_info, .. } => game_module_info,
			GameModule::GameModuleStateLoaded { game_module_info, .. } => game_module_info,
		}
	}

	pub fn get_game_module_config(&self) -> Option<&Box<dyn GameModuleConfig + Sync + Send>> {
		match self {
			GameModule::GameModuleInfoLoaded { .. } => None,
			GameModule::GameModuleConfigLoaded { game_module_config, .. } => Some(game_module_config),
			GameModule::GameModuleStateLoaded { game_module_config, .. } => Some(game_module_config),
		}
	}

	pub fn get_game_module_state(&self) -> Option<&Box<dyn GameModuleState + Sync + Send>> {
		match self {
			GameModule::GameModuleInfoLoaded { .. } => None,
			GameModule::GameModuleConfigLoaded { .. } => None,
			GameModule::GameModuleStateLoaded { game_module_state, .. } => Some(game_module_state),
		}
	}
}

#[derive(Debug, Clone)]
pub enum GameModuleHandleAccessError {
	Locked,
	Poisoned,
}

#[derive(Clone)]
pub struct GameModuleHandle {
	game_module: Arc<Mutex<GameModule>>
}

impl GameModuleHandle {
	fn new(game_module: GameModule) -> GameModuleHandle {
		GameModuleHandle {
			game_module: Arc::new(Mutex::new(game_module)),
		}
	}

	pub fn access(&mut self, wait_for_lock: bool) -> Result<MutexGuard<GameModule>, GameModuleHandleAccessError> {
		if wait_for_lock {
			match self.game_module.lock() {
				Ok(guard) => Ok(guard),
				Err(_) => Err(GameModuleHandleAccessError::Poisoned),
			}
		} else {
			match self.game_module.try_lock() {
				Ok(guard) => Ok(guard),
				Err(err) => {
					match err {
						TryLockError::WouldBlock => Err(GameModuleHandleAccessError::Locked),
						TryLockError::Poisoned(_) => Err(GameModuleHandleAccessError::Poisoned),
					}
				},
			}
		}
	}
}

lazy_static!(
	pub static ref GAME_MODULE_MANAGER: Arc<Mutex<GameModuleManager>> = Arc::new(Mutex::new(GameModuleManager::new()));
);

#[derive(Debug, Clone)]
pub enum GameModuleManagerRegisterGameModuleHandleError {
	GameModuleAlreadyRegistered,
	MutexLocked,
	MutexPoisoned,
}

#[derive(Debug, Clone)]
pub enum GameModuleManagerUnregisterGameModuleHandleError {
	GameModuleNotRegistered,
}

#[derive(Debug, Clone)]
pub enum GameModuleManagerGetGameModulesError {
	MutexLocked,
	MutexPoisoned,
}

#[derive(Debug, Clone)]
pub enum GameModuleManagerDeleteGameModuleError {
	GameModuleDoesNotExist,
}

pub struct GameModuleManager {
	registered_game_module_handles: HashMap<GameModuleID, GameModuleHandle>,
	new_game_module_id: GameModuleID,
	recycled_game_ids: Vec<GameModuleID>,
}

impl GameModuleManager {
	fn new() -> GameModuleManager {
		GameModuleManager {
			registered_game_module_handles: HashMap::new(),
			new_game_module_id: 1,
			recycled_game_ids: Vec::new(),
		}
	}

	pub(in crate) fn register_game_module_handle(&mut self, wait_for_lock: bool, game_module_handle: GameModuleHandle) -> Result<(), GameModuleManagerRegisterGameModuleHandleError> {
		let mut game_module = game_module_handle.clone();
		let game_module = match game_module.access(wait_for_lock) {
			Ok(game_module) => game_module,
			Err(err) => {
				match err {
					GameModuleHandleAccessError::Locked => return Err(GameModuleManagerRegisterGameModuleHandleError::GameModuleAlreadyRegistered),
					GameModuleHandleAccessError::Poisoned => return Err(GameModuleManagerRegisterGameModuleHandleError::GameModuleAlreadyRegistered),
				}
			},
		
		};

		let game_module_info = game_module.get_game_module_info();
		let game_module_id = game_module_info.get_game_module_id();

		if self.registered_game_module_handles.contains_key(&game_module_id) {
			return Err(GameModuleManagerRegisterGameModuleHandleError::GameModuleAlreadyRegistered);
		}

		self.registered_game_module_handles.insert(game_module_id, game_module_handle);

		Ok(())
	}

	pub(in crate) fn unregister_game_module_handle(&mut self, game_module_id: GameModuleID) -> Result<(), GameModuleManagerUnregisterGameModuleHandleError> {
		if let Some(_) = self.registered_game_module_handles.remove(&game_module_id) {
			self.recycle_used_game_module_id(game_module_id);
			Ok(())
		} else {
			Err(GameModuleManagerUnregisterGameModuleHandleError::GameModuleNotRegistered)
		}
	}
	
	fn get_unused_game_module_id(&mut self) -> GameModuleID {
		if let Some(game_module_id) = self.recycled_game_ids.pop() {
			game_module_id
		} else {
			let game_module_id = self.new_game_module_id;
			self.new_game_module_id += 1;
			game_module_id
		}
	}

	fn recycle_used_game_module_id(&mut self, game_module_id: GameModuleID) {
		self.recycled_game_ids.push(game_module_id);
	}

	pub fn get_game_modules(&self) -> Result<Vec<GameModuleInfo>, GameModuleManagerGetGameModulesError> {
		let game_module_handles = self.registered_game_module_handles.values().into_iter();
		let mut game_module_infos: Vec<GameModuleInfo> = Vec::new();

		for game_module_handle in game_module_handles {
			let mut game_module = game_module_handle.clone();

			let game_module = match game_module.access(true) {
				Ok(game_module) => game_module,
				Err(_) => return Err(GameModuleManagerGetGameModulesError::MutexLocked),
			};

			game_module_infos.push(game_module.get_game_module_info().clone());
		}

		Ok(game_module_infos)
	}

	pub fn get_game_module_handle(&self, game_module_id: GameModuleID) -> Option<GameModuleHandle> {
		self.registered_game_module_handles.get(&game_module_id).cloned()
	}

	pub fn create_game_module<T: 'static + GameModuleType>(&mut self) -> GameModuleHandle {
		let game_module = GameModule::new::<T>();

		let game_module_handle = GameModuleHandle::new(game_module);

		match self.register_game_module_handle(true, game_module_handle.clone()) {
			Ok(_) => game_module_handle,
			Err(err) => panic!("Failed to register game module handle: {:?}", err),
		}
	}

	pub fn delete_game_module(&mut self, game_module_id: GameModuleID) -> Result<(), GameModuleManagerDeleteGameModuleError> {
		match self.unregister_game_module_handle(game_module_id) {
			Ok(_) => Ok(()),
			Err(_) => Err(GameModuleManagerDeleteGameModuleError::GameModuleDoesNotExist),
		}
	}
}

#[derive(Debug, Clone)]
pub enum GameLoadGameConfigError {
	IncompatibleConfig,
	AlreadyLoaded,
}

#[derive(Debug, Clone)]
pub enum GameLoadGameStateError {
	IncompatibleState,
	GameConfigNotLoaded,
	AlreadyLoaded,
}

#[derive(Debug, Clone)]
pub enum GameUnloadGameConfigError {
	AlreadyUnloaded,
	GameStateNotUnloaded,
}

#[derive(Debug, Clone)]
pub enum GameUnloadGameStateError {
	AlreadyUnloaded,
}

#[derive(Debug, Clone)]
pub enum GameInstallGameModuleInfoError {
	GameConfigAlreadyLoaded,
	GameStateAlreadyLoaded,
	GameModuleAlreadyInstalled,
}

#[derive(Debug, Clone)]
pub enum GameInstallGameModuleConfigError {
	GameConfigNotLoaded,
	GameStateAlreadyLoaded,
	GameModuleConfigAlreadyInstalled,
	GameModuleConfigNotCompatible,
}

#[derive(Debug, Clone)]
pub enum GameInstallGameModuleStateError {
	GameConfigNotLoaded,
	GameStateNotLoaded,
	GameModuleStateAlreadyInstalled,
	GameModuleStateNotCompatible,
}

#[derive(Debug, Clone)]
pub enum GameUninstallGameModuleInfoError {
	GameConfigStillLoaded,
	GameStateStillLoaded,
	GameModuleInfoNotInstalled,
}

#[derive(Debug, Clone)]
pub enum GameUninstallGameModuleConfigError {
	GameConfigNotLoaded,
	GameStateStillLoaded,
	GameModuleConfigNotInstalled,
}

#[derive(Debug, Clone)]
pub enum GameUninstallGameModuleStateError {
	GameConfigNotLoaded,
	GameStateNotLoaded,
	GameModuleStateNotInstalled,
}

pub enum Game {
	GameInfoLoaded {
		game_info: GameInfo,
	},
	GameConfigLoaded {
		game_info: GameInfo,
		game_config: GameConfig,
	},
	GameStateLoaded {
		game_info: GameInfo,
		game_config: GameConfig,
		game_state: GameState,
	}
}

impl Game {
	fn new(game_info: GameInfo) -> Self {
		Self::GameInfoLoaded {
			game_info,
		}
	}

	pub fn load_game_config(&mut self, game_config: GameConfig) -> Result<(), GameLoadGameConfigError> {
		if !self.get_game_info().is_compatible_with_config(&game_config) {
			return Err(GameLoadGameConfigError::IncompatibleConfig);
		}
		
		match self {
			Game::GameInfoLoaded { game_info } => {
				let game_info = std::mem::take(game_info);
				
				*self = Game::GameConfigLoaded {
					game_info,
					game_config,
				};

				Ok(())
			},
			Game::GameConfigLoaded { .. } => Err(GameLoadGameConfigError::AlreadyLoaded),
			Game::GameStateLoaded { .. } => Err(GameLoadGameConfigError::AlreadyLoaded),
		}
	}

	pub fn load_game_state(&mut self, game_state: GameState) -> Result<(), GameLoadGameStateError> {
		if !self.get_game_info().is_compatible_with_state(&game_state) {
			return Err(GameLoadGameStateError::IncompatibleState);
		}

		match self {
			Game::GameInfoLoaded { .. } => Err(GameLoadGameStateError::GameConfigNotLoaded),
			Game::GameConfigLoaded { game_info, game_config } => {
				let game_info = std::mem::take(game_info);
				let game_config = std::mem::take(game_config);
				
				*self = Game::GameStateLoaded {
					game_info,
					game_config,
					game_state,
				};

				Ok(())
			},
			Game::GameStateLoaded { .. } => Err(GameLoadGameStateError::AlreadyLoaded),
		}
	}

	pub fn unload_game_config(&mut self) -> Result<(), GameUnloadGameConfigError> {
		match self {
			Game::GameInfoLoaded { .. } => Err(GameUnloadGameConfigError::AlreadyUnloaded),
			Game::GameConfigLoaded { game_info, .. } => {
				let game_info = std::mem::take(game_info);
				
				*self = Game::GameInfoLoaded {
					game_info,
				};

				Ok(())
			},
			Game::GameStateLoaded { .. } => Err(GameUnloadGameConfigError::GameStateNotUnloaded),
		}
	}

	pub fn unload_game_state(&mut self) -> Result<(), GameUnloadGameStateError> {
		match self {
			Game::GameInfoLoaded { .. } => Err(GameUnloadGameStateError::AlreadyUnloaded),
			Game::GameConfigLoaded { .. } => Err(GameUnloadGameStateError::AlreadyUnloaded),
			Game::GameStateLoaded { game_info, game_config, .. } => {
				let game_info = std::mem::take(game_info);
				let game_config = std::mem::take(game_config);
				
				*self = Game::GameConfigLoaded {
					game_info,
					game_config,
				};

				Ok(())
			},
		}
	}

	pub fn get_game_info(&self) -> GameInfo {
		match self {
			Game::GameInfoLoaded { game_info } => game_info.clone(),
			Game::GameConfigLoaded { game_info, .. } => game_info.clone(),
			Game::GameStateLoaded { game_info, .. } => game_info.clone(),
		}
	}
	
	pub fn get_game_config(&self) -> Option<&GameConfig> {
		match self {
			Game::GameInfoLoaded { .. } => None,
			Game::GameConfigLoaded { game_config, .. } => Some(game_config),
			Game::GameStateLoaded { game_config, .. } => Some(game_config),
		}
	}
	
	pub fn get_game_state(&self) -> Option<&GameState> {
		match self {
			Game::GameInfoLoaded { .. } => None,
			Game::GameConfigLoaded { .. } => None,
			Game::GameStateLoaded { game_state, .. } => Some(game_state),
		}
	}

	pub fn install_game_module_info(&mut self, game_module: &GameModule) -> Result<(), GameInstallGameModuleInfoError> {
		match self {
			Game::GameInfoLoaded { game_info } => {
				let game_module_info = game_module.get_game_module_info();
				let game_module_id = game_module_info.get_game_module_id();

				if game_info.installed_game_modules.contains_key(&game_module_id) {
					return Err(GameInstallGameModuleInfoError::GameModuleAlreadyInstalled);
				}

				game_info.installed_game_modules.insert(game_module_id, game_module_info.clone());

				Ok(())
			},
			Game::GameConfigLoaded { .. } => Err(GameInstallGameModuleInfoError::GameConfigAlreadyLoaded),
			Game::GameStateLoaded { .. } => Err(GameInstallGameModuleInfoError::GameStateAlreadyLoaded),
		}
	}

	pub fn install_game_module_config(&mut self, game_module: &GameModule) -> Result<(), GameInstallGameModuleConfigError> {
		match self {
			Game::GameInfoLoaded { .. } => Err(GameInstallGameModuleConfigError::GameConfigNotLoaded),
			Game::GameConfigLoaded { game_info, game_config } => {
				let game_module_info = game_module.get_game_module_info();
				let game_module_id = game_module_info.get_game_module_id();

				if !game_info.installed_game_modules.contains_key(&game_module_id) {
					return Err(GameInstallGameModuleConfigError::GameModuleConfigNotCompatible);
				}

				if game_config.installed_game_modules.contains_key(&game_module_id) {
					return Err(GameInstallGameModuleConfigError::GameModuleConfigAlreadyInstalled);
				}

				let game_module_config: Box<dyn GameModuleConfig + Sync + Send> = game_module.get_game_module_config().unwrap().clone();

				game_config.installed_game_modules.insert(game_module_id, game_module_config);

				Ok(())
			},
			Game::GameStateLoaded { .. } => Err(GameInstallGameModuleConfigError::GameStateAlreadyLoaded),
		}
	}

	pub fn install_game_module_state(&mut self, game_module: &GameModule) -> Result<(), GameInstallGameModuleStateError> {
		match self {
			Game::GameInfoLoaded { .. } => Err(GameInstallGameModuleStateError::GameConfigNotLoaded),
			Game::GameConfigLoaded { .. } => Err(GameInstallGameModuleStateError::GameStateNotLoaded),
			Game::GameStateLoaded { game_info, game_config, game_state } => {
				let game_module_info = game_module.get_game_module_info();
				let game_module_id = game_module_info.get_game_module_id();

				if !game_info.installed_game_modules.contains_key(&game_module_id) {
					return Err(GameInstallGameModuleStateError::GameModuleStateNotCompatible);
				}

				if !game_config.installed_game_modules.contains_key(&game_module_id) {
					return Err(GameInstallGameModuleStateError::GameModuleStateNotCompatible);
				}

				if game_state.installed_game_modules.contains_key(&game_module_id) {
					return Err(GameInstallGameModuleStateError::GameModuleStateAlreadyInstalled);
				}

				let game_module_state: Box<dyn GameModuleState + Sync + Send> = game_module.get_game_module_state().unwrap().clone();

				game_state.installed_game_modules.insert(game_module_id, game_module_state);

				Ok(())
			},
		}
	}

	pub fn uninstall_game_module_info(&mut self, game_module_id: GameModuleID) -> Result<(), GameUninstallGameModuleInfoError> {
		match self {
			Game::GameInfoLoaded { game_info } => {
				if !game_info.installed_game_modules.contains_key(&game_module_id) {
					return Err(GameUninstallGameModuleInfoError::GameModuleInfoNotInstalled);
				}

				game_info.installed_game_modules.remove(&game_module_id);

				Ok(())
			},
			Game::GameConfigLoaded { .. } => Err(GameUninstallGameModuleInfoError::GameConfigStillLoaded),
			Game::GameStateLoaded { .. } => Err(GameUninstallGameModuleInfoError::GameStateStillLoaded),
		}
	}

	pub fn uninstall_game_module_config(&mut self, game_module_id: GameModuleID) -> Result<(), GameUninstallGameModuleConfigError> {
		match self {
			Game::GameInfoLoaded { .. } => Err(GameUninstallGameModuleConfigError::GameConfigNotLoaded),
			Game::GameConfigLoaded { game_config, .. } => {
				if !game_config.installed_game_modules.contains_key(&game_module_id) {
					return Err(GameUninstallGameModuleConfigError::GameModuleConfigNotInstalled);
				}

				game_config.installed_game_modules.remove(&game_module_id);

				Ok(())
			},
			Game::GameStateLoaded { .. } => Err(GameUninstallGameModuleConfigError::GameStateStillLoaded),
		}
	}

	pub fn uninstall_game_module_state(&mut self, game_module_id: GameModuleID) -> Result<(), GameUninstallGameModuleStateError> {
		match self {
			Game::GameInfoLoaded { .. } => Err(GameUninstallGameModuleStateError::GameConfigNotLoaded),
			Game::GameConfigLoaded { .. } => Err(GameUninstallGameModuleStateError::GameStateNotLoaded),
			Game::GameStateLoaded { game_state, .. } => {
				if !game_state.installed_game_modules.contains_key(&game_module_id) {
					return Err(GameUninstallGameModuleStateError::GameModuleStateNotInstalled);
				}

				game_state.installed_game_modules.remove(&game_module_id);

				Ok(())
			},
		}
	}
}

#[derive(Debug, Clone)]
pub enum GameHandleAccessError {
	Locked,
	Poisoned,
}

#[derive(Clone)]
pub struct GameHandle {
	game: Arc<Mutex<Game>>
}

impl GameHandle {
	fn new(game: Game) -> GameHandle {
		GameHandle {
			game: Arc::new(Mutex::new(game)),
		}
	}

	pub fn access(&mut self, wait_for_lock: bool) -> Result<MutexGuard<Game>, GameHandleAccessError> {
		if wait_for_lock {
			match self.game.lock() {
				Ok(guard) => Ok(guard),
				Err(_) => Err(GameHandleAccessError::Poisoned),
			}
		} else {
			match self.game.try_lock() {
				Ok(guard) => Ok(guard),
				Err(err) => {
					match err {
						TryLockError::WouldBlock => Err(GameHandleAccessError::Locked),
						TryLockError::Poisoned(_) => Err(GameHandleAccessError::Poisoned),
					}
				},
			}
		}
	}
}

#[derive(Debug, Clone)]
pub enum GameManagerRegisterGameHandleError {
	GameAlreadyRegistered,
	MutexLocked,
	MutexPoisoned,
	AllGameIDsInUse,
}

pub enum GameManagerUnregisterGameHandleError {
	GameNotRegistered,
}

#[derive(Debug, Clone)]
pub enum GameManagerGetGamesError {
	MutexLocked,
	MutexPoisoned,
}

#[derive(Debug, Clone)]
pub enum GameManagerDeleteGameError {
	GameDoesNotExist,
}

#[derive(Debug, Clone)]
pub enum GameManagerJoinGameError {
	GameDoesNotExist,
	GameAlreadyJoined,
	GameConfigNotLoaded,
	GameStateNotLoaded,
	MutexLocked,
	MutexPoisoned,
}

#[derive(Debug, Clone)]
pub enum GameManagerLeaveGameError {
	GameNotJoined,
	MutexLocked,
	MutexPoisoned,
}

lazy_static!(
	pub static ref GAME_MANAGER: Arc<Mutex<GameManager>> = Arc::new(Mutex::new(GameManager::new()));
);

pub struct GameManager {
	registered_game_handles: HashMap<GameID, GameHandle>,
	new_game_id: GameID,
	recycled_game_ids: Vec<GameID>,
	currently_joined_game: Option<GameHandle>,
}

impl GameManager {
	fn new() -> Self {
		Self {
			registered_game_handles: HashMap::new(),
			new_game_id: 1,
			recycled_game_ids: Vec::new(),
			currently_joined_game: None,
		}
	}

	pub(in crate) fn register_game_handle(&mut self, wait_for_lock: bool, game_handle: GameHandle) -> Result<(), GameManagerRegisterGameHandleError> {
		let mut game = game_handle.clone();

		let game = match game.access(wait_for_lock) {
			Ok(game) => game,
			Err(err) => {
				match err {
					GameHandleAccessError::Locked => return Err(GameManagerRegisterGameHandleError::MutexLocked),
					GameHandleAccessError::Poisoned => return Err(GameManagerRegisterGameHandleError::MutexPoisoned),
				}
			},
		
		};

		let game_info = game.get_game_info();

		let game_id = game_info.get_game_id();

		if self.registered_game_handles.contains_key(&game_id) {
			return Err(GameManagerRegisterGameHandleError::GameAlreadyRegistered);
		}

		self.registered_game_handles.insert(game_id, game_handle);

		Ok(())
	}

	pub(in crate) fn unregister_game_handle(&mut self, game_id: GameID) -> Result<(), GameManagerUnregisterGameHandleError> {
		if let Some(_) = self.registered_game_handles.remove(&game_id) {
			self.recycle_used_game_id(game_id);
			Ok(())
		} else {
			Err(GameManagerUnregisterGameHandleError::GameNotRegistered)
		}
	}

	fn get_unused_game_id(&mut self) -> GameID {
		if let Some(game_id) = self.recycled_game_ids.pop() {
			game_id
		} else {
			let game_id = self.new_game_id;
			self.new_game_id += 1;
			game_id
		}
	}

	fn recycle_used_game_id(&mut self, game_id: GameID) {
		self.recycled_game_ids.push(game_id);
	}

	pub fn get_currently_joined_game(&self) -> Option<GameHandle> {
		self.currently_joined_game.clone()
	}

	pub fn get_games(&self, wait_for_lock: bool) -> Result<Vec<GameInfo>, GameManagerGetGamesError> {
		let game_handles = self.registered_game_handles.values().into_iter();
		let mut game_infos: Vec<GameInfo> = Vec::new();

		for game_handle in game_handles {
			let mut game = game_handle.clone();

			let game = match game.access(wait_for_lock) {
				Ok(game) => game,
				Err(err) => {
					match err {
						GameHandleAccessError::Locked => return Err(GameManagerGetGamesError::MutexLocked),
						GameHandleAccessError::Poisoned => return Err(GameManagerGetGamesError::MutexPoisoned),
					}
				},
			
			};

			game_infos.push(game.get_game_info());
		}

		Ok(game_infos)
	}
	
	pub fn get_game_handle(&self, game_id: GameID) -> Option<GameHandle> {
		self.registered_game_handles.get(&game_id).cloned()
	}

	pub fn create_game(&mut self, game_name: String) -> GameHandle {
		let game_id = self.get_unused_game_id();

		if self.registered_game_handles.contains_key(&game_id) {
			panic!("All {} possible game IDs are in use!", u64::MAX)
		} else {
			let game_info = GameInfo {
				game_id,
				name: game_name,
				installed_game_modules: HashMap::new(),
			};

			let game = Game::new(game_info);

			let game_handle = GameHandle::new(game);

			self.registered_game_handles.insert(game_id, game_handle.clone());

			game_handle
		}
	}

	pub fn delete_game(&mut self, game_id: GameID) -> Result<(), GameManagerDeleteGameError> {
		if let Some(_) = self.registered_game_handles.remove(&game_id) {
			self.recycle_used_game_id(game_id);
			Ok(())
		} else {
			Err(GameManagerDeleteGameError::GameDoesNotExist)
		}
	}

	pub fn join_game(&mut self, wait_for_lock: bool, game_id: GameID) -> Result<(), GameManagerJoinGameError> {
		let game_handle = match self.registered_game_handles.get(&game_id) {
			Some(game_handle) => game_handle.clone(),
			None => return Err(GameManagerJoinGameError::GameDoesNotExist),
		};

		if self.currently_joined_game.is_some() {
			return Err(GameManagerJoinGameError::GameAlreadyJoined);
		}

		let mut game = game_handle.clone();

		let mut game = match game.access(wait_for_lock) {
			Ok(game) => game,
			Err(err) => {
				match err {
					GameHandleAccessError::Locked => return Err(GameManagerJoinGameError::MutexLocked),
					GameHandleAccessError::Poisoned => return Err(GameManagerJoinGameError::MutexPoisoned),
				}
			},
		
		};

		let game_state = match *game {
			Game::GameInfoLoaded { .. } => return Err(GameManagerJoinGameError::GameConfigNotLoaded),
			Game::GameConfigLoaded { .. } => return Err(GameManagerJoinGameError::GameStateNotLoaded),
			Game::GameStateLoaded { ref mut game_state, .. } => game_state,
		};

		game_state.has_been_joined = true;

		self.currently_joined_game = Some(game_handle);

		Ok(())
	}

	pub fn leave_game(&mut self, wait_for_lock: bool) -> Result<(), GameManagerLeaveGameError> {
		let mut currently_joined_game_handle = match self.currently_joined_game {
			Some(ref mut currently_joined_game_handle) => currently_joined_game_handle.clone(),
			None => return Err(GameManagerLeaveGameError::GameNotJoined),
		};

		let mut currently_joined_game = match currently_joined_game_handle.access(wait_for_lock) {
			Ok(currently_joined_game) => currently_joined_game,
			Err(err) => {
				match err {
					GameHandleAccessError::Locked => return Err(GameManagerLeaveGameError::MutexLocked),
					GameHandleAccessError::Poisoned => return Err(GameManagerLeaveGameError::MutexPoisoned),
				}
			},
		
		};

		let game_state = match *currently_joined_game {
			Game::GameInfoLoaded { .. } => unreachable!(),
			Game::GameConfigLoaded { .. } => unreachable!(),
			Game::GameStateLoaded { ref mut game_state, .. } => game_state,
		};

		game_state.has_been_joined = false;

		self.currently_joined_game = None;

		Ok(())
	}
}