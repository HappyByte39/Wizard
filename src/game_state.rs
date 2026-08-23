//! State that belongs to a Magic game.

use std::fmt;

/// The minimum number of players in a Magic game.
pub const MINIMUM_PLAYER_COUNT: usize = 2;

/// A player's deck of cards.
///
/// `Card` is generic until the simulator's card representation is implemented.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Deck<Card> {
    cards: Vec<Card>,
}

impl<Card> Deck<Card> {
    /// Creates an empty deck.
    #[must_use]
    pub const fn new() -> Self {
        Self { cards: Vec::new() }
    }

    /// Returns the cards in this deck.
    #[must_use]
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }
}

/// State that belongs to an individual player.
///
/// # Rules
///
/// 100.2. To play, each player needs their own deck of traditional Magic
/// cards, small items to represent any tokens and counters, and some way to
/// clearly track life totals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerState<Card> {
    life_total: i32,
    deck: Deck<Card>,
}

impl<Card> PlayerState<Card> {
    /// Creates a player state with the given life total and deck.
    #[must_use]
    pub const fn new(life_total: i32, deck: Deck<Card>) -> Self {
        Self { life_total, deck }
    }

    /// Returns this player's current life total.
    #[must_use]
    pub const fn life_total(&self) -> i32 {
        self.life_total
    }

    /// Returns this player's deck.
    #[must_use]
    pub const fn deck(&self) -> &Deck<Card> {
        &self.deck
    }
}

/// Stores the state of a Magic game.
///
/// Currently this state records the players in the game. More game-wide state
/// will be added here as the rules that require it are implemented.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState<Card> {
    players: Vec<PlayerState<Card>>,
}

impl<Card> GameState<Card> {
    /// Creates a game state containing `players`.
    ///
    /// # Rules
    ///
    /// 100.1. These Magic rules apply to any Magic game with two or more
    /// players, including two-player games and multiplayer games.
    ///
    /// # Errors
    ///
    /// Returns [`PlayerCountError`] when fewer than two players are supplied.
    pub fn new(players: Vec<PlayerState<Card>>) -> Result<Self, PlayerCountError> {
        if players.len() < MINIMUM_PLAYER_COUNT {
            return Err(PlayerCountError {
                player_count: players.len(),
            });
        }

        Ok(Self { players })
    }

    /// Returns the players in this game.
    #[must_use]
    pub fn players(&self) -> &[PlayerState<Card>] {
        &self.players
    }

    /// Returns the number of players in this game.
    #[must_use]
    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    /// Returns whether this is a two-player game.
    ///
    /// # Rules
    ///
    /// 100.1a A two-player game is a game that begins with only two players.
    #[must_use]
    pub fn is_two_player_game(&self) -> bool {
        self.player_count() == MINIMUM_PLAYER_COUNT
    }

    /// Returns whether this is a multiplayer game.
    ///
    /// # Rules
    ///
    /// 100.1b A multiplayer game is a game that begins with more than two
    /// players. See section 8, “Multiplayer Rules.”
    #[must_use]
    pub fn is_multiplayer_game(&self) -> bool {
        self.player_count() > MINIMUM_PLAYER_COUNT
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
    use super::{Deck, GameState, PlayerCountError, PlayerState};

    fn player(life_total: i32) -> PlayerState<()> {
        PlayerState::new(life_total, Deck::new())
    }

    #[test]
    fn stores_players_and_their_state_for_a_two_player_game() {
        let game_state =
            GameState::new(vec![player(20), player(15)]).expect("two players are valid");

        assert_eq!(game_state.player_count(), 2);
        assert_eq!(game_state.players().len(), game_state.player_count());
        assert_eq!(game_state.players()[0].life_total(), 20);
        assert!(game_state.players()[0].deck().cards().is_empty());
        assert!(game_state.is_two_player_game());
        assert!(!game_state.is_multiplayer_game());
    }

    #[test]
    fn stores_players_for_a_multiplayer_game() {
        let game_state = GameState::new(vec![player(20), player(20), player(20), player(20)])
            .expect("four players are valid");

        assert_eq!(game_state.player_count(), 4);
        assert_eq!(game_state.players().len(), game_state.player_count());
        assert!(!game_state.is_two_player_game());
        assert!(game_state.is_multiplayer_game());
    }

    #[test]
    fn rejects_games_with_fewer_than_two_players() {
        assert_eq!(
            GameState::new(vec![player(20)]),
            Err(PlayerCountError { player_count: 1 })
        );
    }
}
