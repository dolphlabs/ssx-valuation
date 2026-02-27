use crate::ValuationEngine;

pub mod arsenal;
pub mod aston_villa;
pub mod bournemouth;
pub mod brentford;
pub mod brighton;
pub mod chelsea;
pub mod crystal_palace;
pub mod everton;
pub mod fulham;
pub mod ipswich;
pub mod leicester;
pub mod liverpool;
pub mod man_city;
pub mod man_united;
pub mod newcastle;
pub mod nottingham_forest;
pub mod southampton;
pub mod tottenham;
pub mod west_ham;
pub mod wolves;

pub fn seed_premier_league(engine: &mut ValuationEngine) {
    arsenal::seed(engine);
    aston_villa::seed(engine);
    bournemouth::seed(engine);
    brentford::seed(engine);
    brighton::seed(engine);
    chelsea::seed(engine);
    crystal_palace::seed(engine);
    everton::seed(engine);
    fulham::seed(engine);
    ipswich::seed(engine);
    leicester::seed(engine);
    liverpool::seed(engine);
    man_city::seed(engine);
    man_united::seed(engine);
    newcastle::seed(engine);
    nottingham_forest::seed(engine);
    southampton::seed(engine);
    tottenham::seed(engine);
    west_ham::seed(engine);
    wolves::seed(engine);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ValuationEngine;

    #[test]
    fn test_seed_premier_league() {
        let mut engine = ValuationEngine::new();
        seed_premier_league(&mut engine);
        
        // Assertions
        assert_eq!(engine.club_states.len(), 20, "Should have 20 clubs seeded");
        
        // Check a few clubs
        assert!(engine.names.values().any(|n| n == "Manchester City"), "Man City should be present");
        assert!(engine.names.values().any(|n| n == "Arsenal"), "Arsenal should be present");
        assert!(engine.names.values().any(|n| n == "Southampton"), "Southampton should be present");
        
        // Check player count (approximate, since we added ~20 players per club)
        assert!(engine.player_states.len() >= 390, "Should have at least 400 players seeded (20 clubs * ~20 players)");
        
        // Check a specific player (Haaland)
        let haaland_id = 1005;
        assert!(engine.player_states.contains_key(&haaland_id));
        assert_eq!(engine.names.get(&haaland_id).unwrap(), "Erling Haaland");
        
        // Check rivalries
        let city_id = 1;
        let utd_id = 2;
        let city = engine.club_states.get(&city_id).unwrap();
        assert!(city.rivals.iter().any(|(id, _)| *id == utd_id));
    }
}
