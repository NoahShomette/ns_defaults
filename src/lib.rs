pub mod movement;
pub mod camera;
pub mod selection;

/// The 2 overall states that a Bevy_GGF game can be in. If you think there should be more then submit
/// an issue on github and it can be discussed!
///
/// These two states are used for general logic and running the base game systems. Your game should always
/// be in one of these states.
///
/// ## Menu
///
/// Represents a menu outside of a game. Eg, the main menu, or an after game screen
///
/// ## `InGame`
///
/// Represents any time you are in a game and game logic should happen. Eg, starting a match, etc
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Menu,
    InGame,
}
