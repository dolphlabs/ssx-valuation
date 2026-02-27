use ssx_valuation::ValuationEngine;
use ssx_valuation::setup::seed_big_five_leagues;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

fn main() {
    println!("--- SSX Valuation Engine Seeding ---");
    let mut engine = ValuationEngine::new();
    
    println!("Seeding Big Five Leagues data...");
    seed_big_five_leagues(&mut engine);
    
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

    // --- Simulation Mode ---
    println!("\n--- Starting Mock Season Simulation (duration_weeks=4) ---");
    let events = ssx_valuation::setup::generate_mock_season(4);
    println!("Generated {} events.", events.len());

    // Choose a pair to monitor: Liverpool (5) vs Wolves (11)
    let team1_id = 5;
    let team2_id = 11;
    
    let mut last_rate = engine.get_club_pair_exchange_rate(team1_id, team2_id);
    if last_rate.is_zero() {
        last_rate = dec!(1.0); // Fallback
    }

    let mut replay = ssx_valuation::replay::HistoricalReplay::new(&mut engine);

    let mut total_jitter = Decimal::ZERO;
    let mut goal_count = 0;
    let start_instant = std::time::Instant::now();

    let time_series = replay.replay(events, team1_id, team2_id);
    
    let total_duration = start_instant.elapsed();
    let avg_latency_us = total_duration.as_micros() as f64 / time_series.len() as f64;

    // Export to CSV
    use std::fs::File;
    use std::io::Write;
    let mut file = File::create("simulation_results.csv").expect("Unable to create file");
    writeln!(file, "timestamp,exchange_rate").expect("Unable to write header");

    for (ts, rate) in &time_series {
        writeln!(file, "{},{}", ts, rate).expect("Unable to write data");

        // Simple Jitter heuristic
        let diff = (*rate - last_rate).abs();
        if diff > Decimal::ZERO {
            let jitter = (diff / last_rate) * dec!(100.0);
            total_jitter += jitter;
            goal_count += 1;
        }
        last_rate = *rate;
    }

    println!("\n--- Simulation Metrics ---");
    if goal_count > 0 {
        println!("Exchange Rate Jitter: {:.2}% per event (Target: < 5%)", total_jitter / Decimal::from(goal_count));
    }
    println!("Processing Latency: {:.2}us per event (Target: < 1000us)", avg_latency_us);
    println!("Data exported to simulation_results.csv");
    
    // WhistleEnd Convergence check
    let final_vol = engine.player_states.get(&1203).map(|p| p.volatility_factor).unwrap_or(Decimal::ONE);
    println!("WhistleEnd Convergence (V_fact of Salah): {} (Target: 1.0)", final_vol);
}
