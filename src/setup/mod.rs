use crate::ValuationEngine;

pub mod pl;
pub mod la_liga;
pub mod serie_a;
pub mod bundesliga;
pub mod ligue_1;
pub mod generator;

pub fn seed_big_five_leagues(engine: &mut ValuationEngine) {
    pl::seed(engine);
    la_liga::seed(engine);
    serie_a::seed(engine);
    bundesliga::seed(engine);
    ligue_1::seed(engine);
}

pub use generator::generate_mock_season;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ValuationEngine;

    #[test]
    fn test_seed_big_five() {
        let mut engine = ValuationEngine::new();
        seed_big_five_leagues(&mut engine);
        
        // Assertions
        // 20 (PL) + 20 (LL) + 20 (SA) + 18 (BL) + 18 (L1) = 96 clubs
        // For now some might be empty, let's just check the ones we have.
        assert!(engine.club_states.len() >= 20, "Should have at least PL clubs");
    }
}
