//! State that belongs to a Magic game.

use std::fmt;

/// The minimum number of players in a Magic game.
pub const MINIMUM_PLAYER_COUNT: usize = 2;

/// Stores the state of a Magic game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    player_count: usize,
}

impl GameState {
    /// Creates a game state for a game with `player_count` players.
    ///
    /// # Rules
    ///
    /// 100.1. These Magic rules apply to any Magic game with two or more
    /// players, including two-player games and multiplayer games.
    ///
    /// # Errors
    ///
    /// Returns [`PlayerCountError`] when fewer than two players are supplied.
    pub fn new(player_count: usize) -> Result<Self, PlayerCountError> {
        if player_count < MINIMUM_PLAYER_COUNT {
            return Err(PlayerCountError { player_count });
        }

        Ok(Self { player_count })
    }

    /// Returns the number of players in this game.
    #[must_use]
    pub const fn player_count(&self) -> usize {
        self.player_count
    }

    /// Returns whether this is a two-player game.
    ///
    /// # Rules
    ///
    /// 100.1a A two-player game is a game that begins with only two players.
    #[must_use]
    pub const fn is_two_player_game(&self) -> bool {
        self.player_count == 2
    }

    /// Returns whether this is a multiplayer game.
    ///
    /// # Rules
    ///
    /// 100.1b A multiplayer game is a game that begins with more than two
    /// players. See section 8, “Multiplayer Rules.”
    #[must_use]
    pub const fn is_multiplayer_game(&self) -> bool {
        self.player_count > 2
    }
}

/// Error returned when attempting to create a game with too few players.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerCountError {
    player_count: usize,
}

impl PlayerCountError {
    /// Returns the invalid player count that caused this error.
    #[must_use]
    pub const fn player_count(&self) -> usize {
        self.player_count
    }
}

impl fmt::Display for PlayerCountError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "a Magic game needs at least {MINIMUM_PLAYER_COUNT} players; received {}",
            self.player_count
        )
    }
}

impl std::error::Error for PlayerCountError {}

#[cfg(test)]
mod tests {
    use super::{GameState, MINIMUM_PLAYER_COUNT, PlayerCountError};

    #[test]
    fn stores_the_player_count_for_a_two_player_game() {
        let game_state = GameState::new(MINIMUM_PLAYER_COUNT).expect("two players are valid");

        assert_eq!(game_state.player_count(), 2);
        assert!(game_state.is_two_player_game());
        assert!(!game_state.is_multiplayer_game());
    }

    #[test]
    fn stores_the_player_count_for_a_multiplayer_game() {
        let game_state = GameState::new(4).expect("four players are valid");

        assert_eq!(game_state.player_count(), 4);
        assert!(!game_state.is_two_player_game());
        assert!(game_state.is_multiplayer_game());
    }

    #[test]
    fn rejects_games_with_fewer_than_two_players() {
        assert_eq!(GameState::new(1), Err(PlayerCountError { player_count: 1 }));
    }
}
