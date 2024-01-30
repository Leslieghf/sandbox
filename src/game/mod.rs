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