use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 5015;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(270.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Le Havre AC".to_string());

    let players = vec![
        (50701, "Arthur Desmas", Position::GK, dec!(25.0), false),
        (50702, "Loic Nego", Position::RB, dec!(20.0), false),
        (50703, "Arouna Sangante", Position::CB, dec!(35.0), true),
        (50704, "Etienne Youte Kinkoue", Position::CB, dec!(35.0), false),
        (50705, "Christopher Operi", Position::LB, dec!(30.0), false),
        (50706, "Abdoulaye Toure", Position::CDM, dec!(25.0), false),
        (50707, "Oussama Targhalline", Position::CM, dec!(30.0), false),
        (50708, "Yassine Kechta", Position::CM, dec!(30.0), false),
        (50709, "Josué Casimir", Position::RW, dec!(25.0), false),
        (50710, "Antoine Joujou", Position::LW, dec!(30.0), false),
        (50711, "Issa Soumare", Position::ST, dec!(15.0), false),
        // Bench
        (50712, "Mathieu Gorgelin", Position::GK, dec!(5.0), false),
        (50713, "Gautier Lloris", Position::CB, dec!(20.0), false),
        (50714, "Yanis Zouaoui", Position::LB, dec!(15.0), false),
        (50715, "Rassoul Ndiaye", Position::CM, dec!(25.0), false),
        (50716, "Daler Kuzyaev", Position::CM, dec!(35.0), false),
        (50717, "Samuel Grandsir", Position::LW, dec!(25.0), false),
        (50718, "Steve Ngoura", Position::ST, dec!(20.0), false),
        (50719, "Ilyes Housni", Position::ST, dec!(25.0), false),
        (50720, "Andy Logbo", Position::ST, dec!(15.0), false),
    ];

    for (id, name, pos, val, captain) in players {
        engine.player_states.insert(id, PlayerValues {
            team_id: club_id,
            intrinsic_value: val,
            form_weight: dec!(1.0),
            sentiment_score: dec!(1.0),
            volatility_factor: dec!(1.0),
            performance_history: vec![],
            position: pos,
            is_captain: captain,
        });
        engine.names.insert(id, name.to_string());
        if let Some(c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(id);
        }
    }
}
