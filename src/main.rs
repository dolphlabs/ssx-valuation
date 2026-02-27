use ssx_valuation::ValuationEngine;
use ssx_valuation::setup::seed_premier_league;

fn main() {
    println!("--- SSX Valuation Engine Seeding ---");
    let mut engine = ValuationEngine::new();
    
    println!("Seeding Premier League data...");
    seed_premier_league(&mut engine);
    
    println!("Seeding complete!");
    println!("Total Clubs: {}", engine.club_states.len());
    println!("Total Players: {}", engine.player_states.len());
    
    // Print top 5 valued clubs
    let mut clubs: Vec<_> = engine.club_states.values().collect();
    clubs.sort_by(|a, b| b.intrinsic_value.cmp(&a.intrinsic_value));
    
    println!("\nTop 5 Clubs by Intrinsic Value:");
    for club in clubs.iter().take(5) {
        let name = engine.names.get(&club.id).cloned().unwrap_or_else(|| "Unknown".to_string());
        println!("{}: {}", name, club.intrinsic_value);
    }

    // Print top 5 valued players
    let mut players: Vec<_> = engine.player_states.iter().collect();
    players.sort_by(|(_, a), (_, b)| b.intrinsic_value.cmp(&a.intrinsic_value));

    println!("\nTop 5 Players by Intrinsic Value:");
    for (id, player) in players.iter().take(5) {
        let name = engine.names.get(id).cloned().unwrap_or_else(|| "Unknown".to_string());
        println!("{}: {}", name, player.intrinsic_value);
    }
}
