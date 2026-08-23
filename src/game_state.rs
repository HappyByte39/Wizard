//! State that belongs to a Magic game.

use std::{collections::HashMap, fmt};

/// The minimum number of players in a Magic game.
pub const MINIMUM_PLAYER_COUNT: usize = 2;

/// The minimum number of cards in a Constructed deck.
pub const MINIMUM_CONSTRUCTED_DECK_SIZE: usize = 60;

/// The minimum number of cards in a Limited deck.
pub const MINIMUM_LIMITED_DECK_SIZE: usize = 40;

/// The starting life total for a normal Magic game.
///
/// # Rules
///
/// 103.4. Each player begins the game with a starting life total of 20. Some
/// variant games have different starting life totals.
pub const DEFAULT_STARTING_LIFE_TOTAL: i32 = 20;

/// A Magic card's currently modeled information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    english_name: String,
    rules_purpose_name: String,
}

impl Card {
    /// Creates a card whose rules-purpose name is its English name.
    #[must_use]
    pub fn new(english_name: impl Into<String>) -> Self {
        let english_name = english_name.into();

        Self {
            rules_purpose_name: english_name.clone(),
            english_name,
        }
    }

    /// Creates a card with a distinct rules-purpose name.
    ///
    /// Use this for cards with interchangeable names. See rule 201.3b.
    #[must_use]
    pub fn with_rules_purpose_name(
        english_name: impl Into<String>,
        rules_purpose_name: impl Into<String>,
    ) -> Self {
        Self {
            english_name: english_name.into(),
            rules_purpose_name: rules_purpose_name.into(),
        }
    }

    /// Returns the card's English name.
    #[must_use]
    pub fn english_name(&self) -> &str {
        &self.english_name
    }

    /// Returns the name used when rules compare this card's name.
    #[must_use]
    pub fn rules_purpose_name(&self) -> &str {
        &self.rules_purpose_name
    }

    /// Returns whether this card is a basic land.
    ///
    /// This temporary implementation identifies basic lands by their exact
    /// English name. It must be replaced by a check of the card's basic
    /// supertype once card type information is modeled.
    #[must_use]
    pub fn is_basic_land(&self) -> bool {
        matches!(
            self.english_name.as_str(),
            "Plains"
                | "Island"
                | "Swamp"
                | "Mountain"
                | "Forest"
                | "Wastes"
                | "Snow-Covered Plains"
                | "Snow-Covered Island"
                | "Snow-Covered Swamp"
                | "Snow-Covered Mountain"
                | "Snow-Covered Forest"
                | "Snow-Covered Wastes"
        )
    }
}

/// A player's deck of cards.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Creates a deck containing `cards`.
    #[must_use]
    pub fn new(cards: Vec<Card>) -> Self {
        Self { cards }
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
pub struct PlayerState {
    life_total: i32,
    deck: Deck,
}

impl PlayerState {
    /// Creates a player state with the given life total and deck.
    #[must_use]
    pub const fn new(life_total: i32, deck: Deck) -> Self {
        Self { life_total, deck }
    }

    /// Returns this player's current life total.
    #[must_use]
    pub const fn life_total(&self) -> i32 {
        self.life_total
    }

    /// Returns this player's deck.
    #[must_use]
    pub const fn deck(&self) -> &Deck {
        &self.deck
    }
}

/// Stores the state of a Magic game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    players: Vec<PlayerState>,
}

impl GameState {
    /// Creates a Constructed game using `decks` for its players.
    ///
    /// This is the default game constructor until other ways of playing are
    /// implemented.
    ///
    /// # Rules
    ///
    /// 100.1. These Magic rules apply to any Magic game with two or more
    /// players, including two-player games and multiplayer games.
    ///
    /// # Errors
    ///
    /// Returns [`GameStateError`] if the player count or any Constructed deck
    /// is invalid.
    pub fn new(decks: Vec<Deck>) -> Result<Self, GameStateError> {
        Self::new_constructed(decks)
    }

    /// Creates a Constructed game using `decks` for its players.
    ///
    /// # Rules
    ///
    /// 100.2a In constructed play (a way of playing in which each player
    /// creates their own deck ahead of time), each deck has a minimum deck size
    /// of 60 cards. A constructed deck may contain any number of basic land
    /// cards and no more than four of any card with a particular English name
    /// other than basic land cards. For the purposes of deck construction,
    /// cards with interchangeable names have the same English name (see rule
    /// 201.3).
    ///
    /// # Errors
    ///
    /// Returns [`GameStateError`] if the player count or any deck is invalid.
    pub fn new_constructed(decks: Vec<Deck>) -> Result<Self, GameStateError> {
        if decks.len() < MINIMUM_PLAYER_COUNT {
            return Err(PlayerCountError {
                player_count: decks.len(),
            }
            .into());
        }

        for (player_index, deck) in decks.iter().enumerate() {
            validate_constructed_deck(deck, player_index)?;
        }

        let players = decks
            .into_iter()
            .map(|deck| PlayerState::new(DEFAULT_STARTING_LIFE_TOTAL, deck))
            .collect();

        Ok(Self { players })
    }

    /// Creates a Limited game using `decks` for its players.
    ///
    /// # Rules
    ///
    /// 100.2b In limited play (a way of playing in which each player gets the
    /// same quantity of unopened Magic product such as booster packs and
    /// creates their own deck using only this product and basic land cards),
    /// each deck has a minimum deck size of 40 cards. A limited deck may
    /// contain as many duplicates of a card as are included with the product.
    ///
    /// # Errors
    ///
    /// Returns [`GameStateError`] if the player count or any deck is invalid.
    pub fn new_limited(decks: Vec<Deck>) -> Result<Self, GameStateError> {
        if decks.len() < MINIMUM_PLAYER_COUNT {
            return Err(PlayerCountError {
                player_count: decks.len(),
            }
            .into());
        }

        for (player_index, deck) in decks.iter().enumerate() {
            validate_limited_deck(deck, player_index)?;
        }

        let players = decks
            .into_iter()
            .map(|deck| PlayerState::new(DEFAULT_STARTING_LIFE_TOTAL, deck))
            .collect();

        Ok(Self { players })
    }

    /// Returns the players in this game.
    #[must_use]
    pub fn players(&self) -> &[PlayerState] {
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

fn validate_constructed_deck(deck: &Deck, player_index: usize) -> Result<(), GameStateError> {
    if deck.cards.len() < MINIMUM_CONSTRUCTED_DECK_SIZE {
        return Err(GameStateError::DeckTooSmall {
            player_index,
            deck_size: deck.cards.len(),
        });
    }

    let mut copies_by_name = HashMap::new();
    for card in &deck.cards {
        if card.is_basic_land() {
            continue;
        }

        let copies = copies_by_name
            .entry(card.rules_purpose_name())
            .and_modify(|copies| *copies += 1)
            .or_insert(1);

        if *copies > 4 {
            return Err(GameStateError::TooManyCardCopies {
                player_index,
                rules_purpose_name: card.rules_purpose_name().to_owned(),
                copies: *copies,
            });
        }
    }

    Ok(())
}

fn validate_limited_deck(deck: &Deck, player_index: usize) -> Result<(), GameStateError> {
    if deck.cards.len() < MINIMUM_LIMITED_DECK_SIZE {
        return Err(GameStateError::LimitedDeckTooSmall {
            player_index,
            deck_size: deck.cards.len(),
        });
    }

    Ok(())
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

/// Error returned when creating a game with invalid starting data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameStateError {
    /// The game has fewer than two players.
    PlayerCount(PlayerCountError),
    /// A Constructed deck has fewer than 60 cards.
    DeckTooSmall {
        /// Zero-based index of the player who owns the deck.
        player_index: usize,
        /// Number of cards in the deck.
        deck_size: usize,
    },
    /// A Limited deck has fewer than 40 cards.
    LimitedDeckTooSmall {
        /// Zero-based index of the player who owns the deck.
        player_index: usize,
        /// Number of cards in the deck.
        deck_size: usize,
    },
    /// A nonbasic card has more than four copies in a Constructed deck.
    TooManyCardCopies {
        /// Zero-based index of the player who owns the deck.
        player_index: usize,
        /// Name used to compare copies for rules purposes.
        rules_purpose_name: String,
        /// Number of copies found in the deck.
        copies: usize,
    },
}

impl From<PlayerCountError> for GameStateError {
    fn from(error: PlayerCountError) -> Self {
        Self::PlayerCount(error)
    }
}

impl fmt::Display for GameStateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PlayerCount(error) => error.fmt(formatter),
            Self::DeckTooSmall {
                player_index,
                deck_size,
            } => write!(
                formatter,
                "player {player_index}'s Constructed deck has {deck_size} cards; it needs at least {MINIMUM_CONSTRUCTED_DECK_SIZE}"
            ),
            Self::LimitedDeckTooSmall {
                player_index,
                deck_size,
            } => write!(
                formatter,
                "player {player_index}'s Limited deck has {deck_size} cards; it needs at least {MINIMUM_LIMITED_DECK_SIZE}"
            ),
            Self::TooManyCardCopies {
                player_index,
                rules_purpose_name,
                copies,
            } => write!(
                formatter,
                "player {player_index}'s Constructed deck has {copies} copies of {rules_purpose_name}; it may have at most four"
            ),
        }
    }
}

impl std::error::Error for GameStateError {}

#[cfg(test)]
mod tests {
    use super::{Card, DEFAULT_STARTING_LIFE_TOTAL, Deck, GameState, GameStateError};

    fn basic_deck() -> Deck {
        Deck::new(vec![Card::new("Plains"); 60])
    }

    #[test]
    fn stores_the_player_count_for_a_two_player_game() {
        let game_state = GameState::new(vec![basic_deck(), basic_deck()]).expect("decks are valid");

        assert_eq!(game_state.player_count(), 2);
        assert_eq!(game_state.players().len(), game_state.player_count());
        assert_eq!(
            game_state.players()[0].life_total(),
            DEFAULT_STARTING_LIFE_TOTAL
        );
        assert_eq!(game_state.players()[0].deck().cards().len(), 60);
        assert!(game_state.is_two_player_game());
        assert!(!game_state.is_multiplayer_game());
    }

    #[test]
    fn stores_the_player_count_for_a_multiplayer_game() {
        let game_state =
            GameState::new(vec![basic_deck(), basic_deck(), basic_deck(), basic_deck()])
                .expect("decks are valid");

        assert_eq!(game_state.player_count(), 4);
        assert_eq!(game_state.players().len(), game_state.player_count());
        assert!(!game_state.is_two_player_game());
        assert!(game_state.is_multiplayer_game());
    }

    #[test]
    fn rejects_games_with_fewer_than_two_players() {
        assert_eq!(
            GameState::new(vec![basic_deck()]),
            Err(GameStateError::PlayerCount(super::PlayerCountError {
                player_count: 1,
            }))
        );
    }

    #[test]
    fn rejects_a_constructed_deck_with_fewer_than_sixty_cards() {
        let result = GameState::new(vec![Deck::new(vec![Card::new("Plains"); 59]), basic_deck()]);

        assert_eq!(
            result,
            Err(GameStateError::DeckTooSmall {
                player_index: 0,
                deck_size: 59,
            })
        );
    }

    #[test]
    fn allows_any_number_of_basic_lands() {
        assert!(GameState::new(vec![basic_deck(), basic_deck()]).is_ok());
    }

    #[test]
    fn identifies_basic_land_names() {
        assert!(Card::new("Snow-Covered Forest").is_basic_land());
        assert!(Card::new("Snow-Covered Wastes").is_basic_land());
        assert!(!Card::new("Llanowar Elves").is_basic_land());
    }

    #[test]
    fn rejects_more_than_four_copies_of_a_nonbasic_card() {
        let deck = Deck::new(vec![Card::new("Lightning Bolt"); 60]);

        assert_eq!(
            GameState::new(vec![deck, basic_deck()]),
            Err(GameStateError::TooManyCardCopies {
                player_index: 0,
                rules_purpose_name: "Lightning Bolt".to_owned(),
                copies: 5,
            })
        );
    }

    #[test]
    fn uses_the_rules_purpose_name_when_counting_card_copies() {
        let mut cards = vec![Card::new("Plains"); 55];
        cards.extend((0..5).map(|index| {
            Card::with_rules_purpose_name(format!("Interchangeable Card {index}"), "Shared Name")
        }));

        assert_eq!(
            GameState::new(vec![Deck::new(cards), basic_deck()]),
            Err(GameStateError::TooManyCardCopies {
                player_index: 0,
                rules_purpose_name: "Shared Name".to_owned(),
                copies: 5,
            })
        );
    }

    #[test]
    fn allows_four_copies_when_names_share_rules_purpose_names() {
        let mut cards = vec![Card::new("Plains"); 48];
        cards.extend(vec![Card::new("Card With Its Own Name"); 4]);
        cards.extend((0..4).map(|index| {
            Card::with_rules_purpose_name(
                format!("Interchangeable Card {index}"),
                "Interchangeable Name",
            )
        }));
        cards.extend([
            Card::new("Partially Interchangeable Name"),
            Card::new("Partially Interchangeable Name"),
            Card::with_rules_purpose_name("Alternate Name One", "Partially Interchangeable Name"),
            Card::with_rules_purpose_name("Alternate Name Two", "Partially Interchangeable Name"),
        ]);

        assert!(GameState::new(vec![Deck::new(cards), basic_deck()]).is_ok());
    }

    #[test]
    fn creates_a_limited_game_with_more_than_four_copies_of_a_card() {
        let limited_deck = Deck::new(vec![Card::new("Lightning Bolt"); 40]);

        let game_state = GameState::new_limited(vec![limited_deck, basic_deck()])
            .expect("Limited decks may contain any number of duplicates");

        assert_eq!(game_state.player_count(), 2);
        assert_eq!(game_state.players()[0].deck().cards().len(), 40);
    }

    #[test]
    fn rejects_a_limited_deck_with_fewer_than_forty_cards() {
        let limited_deck = Deck::new(vec![Card::new("Lightning Bolt"); 39]);

        assert_eq!(
            GameState::new_limited(vec![limited_deck, basic_deck()]),
            Err(GameStateError::LimitedDeckTooSmall {
                player_index: 0,
                deck_size: 39,
            })
        );
    }
}
