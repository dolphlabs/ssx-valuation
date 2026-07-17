//! Player-name matching, scoped per-club (never a global fuzzy match - each
//! lookup only searches the ~20-30 players already known to belong to the
//! target club). A miss here is cheap in the historical backfill (the
//! club-level Goal/Card effect fires independent of whether `player_id`
//! resolves), and in the live transfers poller a miss just means treating a
//! move as a brand-new signing rather than linking it to an existing record.
use crate::{ClubId, PlayerId, ValuationEngine};
use std::collections::HashMap;

pub struct PlayerIndex {
    /// club_id -> [(normalized_name, PlayerId)]
    by_club: HashMap<u32, Vec<(String, PlayerId)>>,
}

impl PlayerIndex {
    /// Builds the index directly from an engine's own `player_states`/
    /// `names` - reuses the exact same names the live system serves, no
    /// re-parsing of seed `.rs` source required.
    pub fn build(engine: &ValuationEngine) -> Self {
        let mut by_club: HashMap<u32, Vec<(String, PlayerId)>> = HashMap::new();
        for entry in engine.player_states.iter() {
            let player_id = *entry.key();
            let team_id = entry.value().team_id;
            if let Some(name) = engine.names.get(&player_id.0) {
                by_club.entry(team_id.0).or_default().push((normalize(name.as_str()), player_id));
            }
        }
        Self { by_club }
    }

    /// Matches a real event's player name against the given club's own
    /// roster only. Tries an exact normalized-name match first, then falls
    /// back to a surname match if it's unique within that club.
    pub fn find(&self, club_id: ClubId, api_player_name: &str) -> Option<PlayerId> {
        let candidates = self.by_club.get(&club_id.0)?;
        let target = normalize(api_player_name);

        if let Some((_, id)) = candidates.iter().find(|(name, _)| *name == target) {
            return Some(*id);
        }

        let target_surname = target.split(' ').next_back()?;
        let surname_matches: Vec<&PlayerId> = candidates
            .iter()
            .filter(|(name, _)| name.split(' ').next_back() == Some(target_surname))
            .map(|(_, id)| id)
            .collect();
        if surname_matches.len() == 1 {
            return Some(*surname_matches[0]);
        }
        None
    }
}

/// Lowercase, ASCII-fold common Latin diacritics, strip punctuation, collapse
/// whitespace - deliberately simple (not a general internationalization
/// library) but covers the accented names actually present in top-five-
/// league rosters (é, è, ñ, ö, ü, etc.).
pub fn normalize(name: &str) -> String {
    let folded: String = name
        .chars()
        .map(|c| match c {
            'á' | 'à' | 'â' | 'ã' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'ô' | 'õ' | 'ö' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'ñ' => 'n',
            'ç' => 'c',
            'ý' | 'ÿ' => 'y',
            other => other,
        })
        .collect();
    let cleaned: String = folded
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { ' ' })
        .collect();
    cleaned.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PlayerValues, Position};
    use rust_decimal_macros::dec;

    fn engine_with(club_id: u32, players: &[(u32, &str)]) -> ValuationEngine {
        let engine = ValuationEngine::new();
        for (id, name) in players {
            engine.names.insert(*id, name.to_string());
            engine.player_states.insert(
                PlayerId(*id),
                PlayerValues {
                    team_id: ClubId(club_id),
                    intrinsic_value: dec!(50.0),
                    form_weight: dec!(1.0),
                    sentiment_score: dec!(1.0),
                    volatility_factor: dec!(1.0),
                    performance_history: vec![dec!(50.0); 5],
                    position: Position::CM,
                    is_captain: false,
                    active: true,
                },
            );
        }
        engine
    }

    #[test]
    fn normalize_folds_accents_and_punctuation() {
        assert_eq!(normalize("N'Golo Kanté"), "n golo kante");
        assert_eq!(normalize("Jorginho"), "jorginho");
        assert_eq!(normalize("  Bukayo   Saka "), "bukayo saka");
    }

    #[test]
    fn exact_name_match_within_club() {
        let engine = engine_with(4, &[(1155, "Bukayo Saka"), (1154, "Martin Odegaard")]);
        let index = PlayerIndex::build(&engine);
        assert_eq!(index.find(ClubId(4), "Bukayo Saka"), Some(PlayerId(1155)));
    }

    #[test]
    fn accented_api_name_matches_unaccented_seed_name() {
        let engine = engine_with(6, &[(2001, "N'Golo Kante")]);
        let index = PlayerIndex::build(&engine);
        assert_eq!(index.find(ClubId(6), "N'Golo Kanté"), Some(PlayerId(2001)));
    }

    #[test]
    fn unique_surname_fallback_matches() {
        let engine = engine_with(4, &[(1155, "Bukayo Saka")]);
        let index = PlayerIndex::build(&engine);
        // API sometimes reports a shortened/nicknamed first name - surname is still unique.
        assert_eq!(index.find(ClubId(4), "B. Saka"), Some(PlayerId(1155)));
    }

    #[test]
    fn ambiguous_surname_does_not_guess() {
        let engine = engine_with(4, &[(1, "John Smith"), (2, "Peter Smith")]);
        let index = PlayerIndex::build(&engine);
        assert_eq!(index.find(ClubId(4), "Smith"), None);
    }

    #[test]
    fn no_cross_club_matching() {
        let engine = engine_with(4, &[(1155, "Bukayo Saka")]);
        let index = PlayerIndex::build(&engine);
        assert_eq!(index.find(ClubId(6), "Bukayo Saka"), None);
    }

    #[test]
    fn unmatched_name_returns_none() {
        let engine = engine_with(4, &[(1155, "Bukayo Saka")]);
        let index = PlayerIndex::build(&engine);
        assert_eq!(index.find(ClubId(4), "Someone Unrelated"), None);
    }
}
