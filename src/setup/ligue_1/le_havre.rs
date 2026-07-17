use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5015);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(343.72); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Le Havre AC".to_string());

    let players = vec![
        (50701, "Arthur Desmas", Position::GK, dec!(25.00), dec!(1.0), false),
        (50702, "Loic Nego", Position::RB, dec!(23.17), dec!(1.032), false),
        (50703, "Arouna Sangante", Position::CB, dec!(35.04), dec!(1.042), true),
        (50704, "Etienne Youte Kinkoue", Position::CB, dec!(36.92), dec!(1.024), false),
        (50705, "Christopher Operi", Position::LB, dec!(30.00), dec!(1.0), false),
        (50706, "Abdoulaye Toure", Position::CDM, dec!(31.00), dec!(0.964), false),
        (50707, "Oussama Targhalline", Position::CM, dec!(30.00), dec!(1.0), false),
        (50708, "Yassine Kechta", Position::CM, dec!(33.74), dec!(1.048), false),
        (50709, "Josué Casimir", Position::RW, dec!(25.00), dec!(1.0), false),
        (50710, "Antoine Joujou", Position::LW, dec!(30.00), dec!(1.0), false),
        (50711, "Issa Soumare", Position::ST, dec!(39.24), dec!(1.059), false),
        // Bench
        (50712, "Mathieu Gorgelin", Position::GK, dec!(5.00), dec!(1.0), false),
        (50713, "Gautier Lloris", Position::CB, dec!(22.81), dec!(1.055), false),
        (50714, "Yanis Zouaoui", Position::LB, dec!(15.80), dec!(1.010), false),
        (50715, "Rassoul Ndiaye", Position::CM, dec!(57.17), dec!(1.092), false),
        (50716, "Daler Kuzyaev", Position::CM, dec!(35.00), dec!(1.0), false),
        (50717, "Samuel Grandsir", Position::LW, dec!(25.00), dec!(1.0), false),
        (50718, "Steve Ngoura", Position::ST, dec!(20.00), dec!(1.0), false),
        (50719, "Ilyes Housni", Position::ST, dec!(25.00), dec!(1.0), false),
        (50720, "Andy Logbo", Position::ST, dec!(15.00), dec!(1.0), false),
    ];

    for (id, name, pos, val, form_weight, captain) in players {
        engine.player_states.insert(PlayerId(id), PlayerValues {
            team_id: club_id,
            intrinsic_value: val,
            form_weight,
            sentiment_score: dec!(1.0),
            volatility_factor: dec!(1.0),
            performance_history: vec![],
            position: pos,
            is_captain: captain,
            active: true,
        });
        engine.names.insert(id, name.to_string());
        if let Some(mut c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(PlayerId(id));
        }
    }
}
