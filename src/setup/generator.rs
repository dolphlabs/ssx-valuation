use crate::{MatchEvent};
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use rand_distr::{Distribution, Poisson};

pub fn generate_mock_season(duration_weeks: u32) -> Vec<(u64, MatchEvent)> {
    let mut rng = thread_rng();
    let mut events = Vec::new();
    let seconds_per_week = 604_800;
    let current_ts = 1000; // Starting timestamp

    let club_ids: Vec<u32> = (1..=20).collect();
    // Assuming each club has players with IDs like 1000 + (club_id-1)*50 + i
    // In our seeding:
    // Man City (1): 1001-1020
    // Man United (2): 1051-1071
    // ...
    // Southampton (20): 1951-1970
    
    let get_player_ids = |club_id: u32| -> Vec<u32> {
        let base = 1000 + (club_id - 1) * 50;
        (1..=20).map(|i| base + i).collect()
    };

    let poisson = Poisson::new(1.4).unwrap(); // Avg 1.4 per team => 2.8 per match

    for week in 0..duration_weeks {
        let week_start = current_ts + (week as u64 * seconds_per_week);
        
        for match_idx in 0..10 {
            // Pick two random clubs
            let mut teams = club_ids.choose_multiple(&mut rng, 2);
            let team_a_id = *teams.next().unwrap();
            let team_b_id = *teams.next().unwrap();
            
            let match_ts = week_start + (match_idx as u64 * 3600 * 2); // Spread matches every 2 hours
            
            let goals_a = poisson.sample(&mut rng) as u32;
            let goals_b = poisson.sample(&mut rng) as u32;
            
            let players_a = get_player_ids(team_a_id);
            let players_b = get_player_ids(team_b_id);

            // Team A Goals
            for _ in 0..goals_a {
                let pid = *players_a.choose(&mut rng).unwrap();
                let minute = rng.gen_range(1..90);
                events.push((match_ts + (minute as u64 * 60), MatchEvent::Goal {
                    team_id: team_a_id,
                    opponent_id: team_b_id,
                    player_id: pid,
                    minute,
                }));
            }

            // Team B Goals
            for _ in 0..goals_b {
                let pid = *players_b.choose(&mut rng).unwrap();
                let minute = rng.gen_range(1..90);
                events.push((match_ts + (minute as u64 * 60), MatchEvent::Goal {
                    team_id: team_b_id,
                    opponent_id: team_a_id,
                    player_id: pid,
                    minute,
                }));
            }

            // Injury / RedCard (0.05 prob each per match)
            if rng.gen_bool(0.05) {
                let team_id = if rng.gen_bool(0.5) { team_a_id } else { team_b_id };
                let opponent_id = if team_id == team_a_id { team_b_id } else { team_a_id };
                let players = if team_id == team_a_id { &players_a } else { &players_b };
                let pid = *players.choose(&mut rng).unwrap();
                let minute = rng.gen_range(1..90);
                events.push((match_ts + (minute as u64 * 60), MatchEvent::RedCard {
                    team_id,
                    opponent_id,
                    player_id: pid,
                }));
            }

            if rng.gen_bool(0.05) {
                let team_id = if rng.gen_bool(0.5) { team_a_id } else { team_b_id };
                let players = if team_id == team_a_id { &players_a } else { &players_b };
                let pid = *players.choose(&mut rng).unwrap();
                let minute = rng.gen_range(1..90);
                events.push((match_ts + (minute as u64 * 60), MatchEvent::Injury {
                    player_id: pid,
                    severity: rng.gen_range(10..50),
                }));
            }

            // WhistleEnd
            events.push((match_ts + 5400, MatchEvent::WhistleEnd {
                team_a_id,
                team_b_id,
            }));
        }
    }

    events.sort_by_key(|(ts, _)| *ts);
    events
}
