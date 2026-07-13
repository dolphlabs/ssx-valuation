use serde::Serialize;

/// Canonical, customer-facing metadata for a club: display name, ticker
/// (short code), and league. This is the single source of truth consumed by
/// `ssx-node` (which publishes it to Redis at boot) and, transitively, by the
/// frontend's pair picker — nothing downstream should hardcode club names or
/// short codes of its own.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct ClubMeta {
    pub id: u32,
    pub name: &'static str,
    pub short_code: &'static str,
    pub league: &'static str,
}

const PL: &str = "Premier League";
const LA_LIGA: &str = "La Liga";
const SERIE_A: &str = "Serie A";
const BUNDESLIGA: &str = "Bundesliga";
const LIGUE_1: &str = "Ligue 1";

pub const ROSTER: &[ClubMeta] = &[
    // Premier League
    ClubMeta { id: 1, name: "Manchester City", short_code: "MCI", league: PL },
    ClubMeta { id: 2, name: "Manchester United", short_code: "MUN", league: PL },
    ClubMeta { id: 3, name: "Tottenham Hotspur", short_code: "TOT", league: PL },
    ClubMeta { id: 4, name: "Arsenal", short_code: "ARS", league: PL },
    ClubMeta { id: 5, name: "Liverpool", short_code: "LIV", league: PL },
    ClubMeta { id: 6, name: "Chelsea", short_code: "CHE", league: PL },
    ClubMeta { id: 7, name: "Newcastle United", short_code: "NEW", league: PL },
    ClubMeta { id: 8, name: "Aston Villa", short_code: "AVL", league: PL },
    ClubMeta { id: 9, name: "West Ham United", short_code: "WHU", league: PL },
    ClubMeta { id: 10, name: "Brighton & Hove Albion", short_code: "BHA", league: PL },
    ClubMeta { id: 11, name: "Wolverhampton Wanderers", short_code: "WOL", league: PL },
    ClubMeta { id: 12, name: "Fulham", short_code: "FUL", league: PL },
    ClubMeta { id: 13, name: "Bournemouth", short_code: "BOU", league: PL },
    ClubMeta { id: 14, name: "Crystal Palace", short_code: "CRY", league: PL },
    ClubMeta { id: 15, name: "Brentford", short_code: "BRE", league: PL },
    ClubMeta { id: 16, name: "Everton", short_code: "EVE", league: PL },
    ClubMeta { id: 17, name: "Nottingham Forest", short_code: "NFO", league: PL },
    ClubMeta { id: 18, name: "Ipswich Town", short_code: "IPS", league: PL },
    ClubMeta { id: 19, name: "Leicester City", short_code: "LEI", league: PL },
    ClubMeta { id: 20, name: "Southampton", short_code: "SOU", league: PL },
    // La Liga
    ClubMeta { id: 2001, name: "Real Madrid", short_code: "RMA", league: LA_LIGA },
    ClubMeta { id: 2002, name: "FC Barcelona", short_code: "BAR", league: LA_LIGA },
    ClubMeta { id: 2003, name: "Atletico Madrid", short_code: "ATM", league: LA_LIGA },
    ClubMeta { id: 2004, name: "Girona FC", short_code: "GIR", league: LA_LIGA },
    ClubMeta { id: 2005, name: "Athletic Club", short_code: "ATH", league: LA_LIGA },
    ClubMeta { id: 2006, name: "Real Sociedad", short_code: "RSO", league: LA_LIGA },
    ClubMeta { id: 2007, name: "Real Betis", short_code: "BET", league: LA_LIGA },
    ClubMeta { id: 2008, name: "Villarreal CF", short_code: "VIL", league: LA_LIGA },
    ClubMeta { id: 2009, name: "Valencia CF", short_code: "VAL", league: LA_LIGA },
    ClubMeta { id: 2010, name: "Deportivo Alaves", short_code: "ALA", league: LA_LIGA },
    ClubMeta { id: 2011, name: "CA Osasuna", short_code: "OSA", league: LA_LIGA },
    ClubMeta { id: 2012, name: "Getafe CF", short_code: "GET", league: LA_LIGA },
    ClubMeta { id: 2013, name: "RC Celta", short_code: "CEL", league: LA_LIGA },
    ClubMeta { id: 2014, name: "Sevilla FC", short_code: "SEV", league: LA_LIGA },
    ClubMeta { id: 2015, name: "RCD Mallorca", short_code: "MLL", league: LA_LIGA },
    ClubMeta { id: 2016, name: "UD Las Palmas", short_code: "LPA", league: LA_LIGA },
    ClubMeta { id: 2017, name: "Rayo Vallecano", short_code: "RAY", league: LA_LIGA },
    ClubMeta { id: 2018, name: "CD Leganes", short_code: "LEG", league: LA_LIGA },
    ClubMeta { id: 2019, name: "Real Valladolid", short_code: "VLL", league: LA_LIGA },
    ClubMeta { id: 2020, name: "RCD Espanyol", short_code: "ESP", league: LA_LIGA },
    // Serie A
    ClubMeta { id: 3001, name: "Inter Milan", short_code: "INT", league: SERIE_A },
    ClubMeta { id: 3002, name: "AC Milan", short_code: "ACM", league: SERIE_A },
    ClubMeta { id: 3003, name: "Juventus", short_code: "JUV", league: SERIE_A },
    ClubMeta { id: 3004, name: "Atalanta", short_code: "ATA", league: SERIE_A },
    ClubMeta { id: 3005, name: "Napoli", short_code: "NAP", league: SERIE_A },
    ClubMeta { id: 3006, name: "AS Roma", short_code: "ROM", league: SERIE_A },
    ClubMeta { id: 3007, name: "Lazio", short_code: "LAZ", league: SERIE_A },
    ClubMeta { id: 3008, name: "Fiorentina", short_code: "FIO", league: SERIE_A },
    ClubMeta { id: 3009, name: "Torino", short_code: "TOR", league: SERIE_A },
    ClubMeta { id: 3010, name: "Bologna", short_code: "BOL", league: SERIE_A },
    ClubMeta { id: 3011, name: "Genoa", short_code: "GEN", league: SERIE_A },
    ClubMeta { id: 3012, name: "Monza", short_code: "MZA", league: SERIE_A },
    ClubMeta { id: 3013, name: "Hellas Verona", short_code: "VER", league: SERIE_A },
    ClubMeta { id: 3014, name: "Udinese", short_code: "UDI", league: SERIE_A },
    ClubMeta { id: 3015, name: "Cagliari", short_code: "CAG", league: SERIE_A },
    ClubMeta { id: 3016, name: "Lecce", short_code: "LEC", league: SERIE_A },
    ClubMeta { id: 3017, name: "Empoli", short_code: "EMP", league: SERIE_A },
    ClubMeta { id: 3018, name: "Parma", short_code: "PAR", league: SERIE_A },
    ClubMeta { id: 3019, name: "Como", short_code: "COM", league: SERIE_A },
    ClubMeta { id: 3020, name: "Venezia", short_code: "VEN", league: SERIE_A },
    // Bundesliga
    ClubMeta { id: 4001, name: "Bayer Leverkusen", short_code: "LEV", league: BUNDESLIGA },
    ClubMeta { id: 4002, name: "Bayern Munich", short_code: "BAY", league: BUNDESLIGA },
    ClubMeta { id: 4003, name: "VfB Stuttgart", short_code: "VFB", league: BUNDESLIGA },
    ClubMeta { id: 4004, name: "RB Leipzig", short_code: "RBL", league: BUNDESLIGA },
    ClubMeta { id: 4005, name: "Borussia Dortmund", short_code: "BVB", league: BUNDESLIGA },
    ClubMeta { id: 4006, name: "Eintracht Frankfurt", short_code: "SGE", league: BUNDESLIGA },
    ClubMeta { id: 4007, name: "TSG Hoffenheim", short_code: "TSG", league: BUNDESLIGA },
    ClubMeta { id: 4008, name: "FCH Heidenheim", short_code: "HDH", league: BUNDESLIGA },
    ClubMeta { id: 4009, name: "Werder Bremen", short_code: "SVW", league: BUNDESLIGA },
    ClubMeta { id: 4010, name: "SC Freiburg", short_code: "SCF", league: BUNDESLIGA },
    ClubMeta { id: 4011, name: "FC Augsburg", short_code: "FCA", league: BUNDESLIGA },
    ClubMeta { id: 4012, name: "VfL Wolfsburg", short_code: "WOB", league: BUNDESLIGA },
    ClubMeta { id: 4013, name: "Mainz 05", short_code: "M05", league: BUNDESLIGA },
    ClubMeta { id: 4014, name: "Borussia Monchengladbach", short_code: "BMG", league: BUNDESLIGA },
    ClubMeta { id: 4015, name: "Union Berlin", short_code: "FCU", league: BUNDESLIGA },
    ClubMeta { id: 4016, name: "VfL Bochum", short_code: "BOC", league: BUNDESLIGA },
    ClubMeta { id: 4017, name: "FC St. Pauli", short_code: "STP", league: BUNDESLIGA },
    ClubMeta { id: 4018, name: "Holstein Kiel", short_code: "KIE", league: BUNDESLIGA },
    // Ligue 1
    ClubMeta { id: 5001, name: "Paris Saint-Germain", short_code: "PSG", league: LIGUE_1 },
    ClubMeta { id: 5002, name: "AS Monaco", short_code: "ASM", league: LIGUE_1 },
    ClubMeta { id: 5003, name: "Lille OSC", short_code: "LIL", league: LIGUE_1 },
    ClubMeta { id: 5004, name: "Stade Brestois 29", short_code: "BRS", league: LIGUE_1 },
    ClubMeta { id: 5005, name: "OGC Nice", short_code: "NIC", league: LIGUE_1 },
    ClubMeta { id: 5006, name: "Olympique Lyonnais", short_code: "OLY", league: LIGUE_1 },
    ClubMeta { id: 5007, name: "RC Lens", short_code: "RCL", league: LIGUE_1 },
    ClubMeta { id: 5008, name: "Olympique de Marseille", short_code: "OM", league: LIGUE_1 },
    ClubMeta { id: 5009, name: "Stade de Reims", short_code: "REI", league: LIGUE_1 },
    ClubMeta { id: 5010, name: "Stade Rennais FC", short_code: "REN", league: LIGUE_1 },
    ClubMeta { id: 5011, name: "Toulouse FC", short_code: "TFC", league: LIGUE_1 },
    ClubMeta { id: 5012, name: "Montpellier HSC", short_code: "MTP", league: LIGUE_1 },
    ClubMeta { id: 5013, name: "RC Strasbourg Alsace", short_code: "RCS", league: LIGUE_1 },
    ClubMeta { id: 5014, name: "FC Nantes", short_code: "NAN", league: LIGUE_1 },
    ClubMeta { id: 5015, name: "Le Havre AC", short_code: "HAC", league: LIGUE_1 },
    ClubMeta { id: 5016, name: "AJ Auxerre", short_code: "AUX", league: LIGUE_1 },
    ClubMeta { id: 5017, name: "Angers SCO", short_code: "ANG", league: LIGUE_1 },
    ClubMeta { id: 5018, name: "AS Saint-Etienne", short_code: "STE", league: LIGUE_1 },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ValuationEngine;
    use crate::setup::seed_big_five_leagues;
    use std::collections::HashSet;

    #[test]
    fn roster_has_96_unique_entries() {
        assert_eq!(ROSTER.len(), 96);

        let ids: HashSet<u32> = ROSTER.iter().map(|c| c.id).collect();
        assert_eq!(ids.len(), ROSTER.len(), "duplicate club id in ROSTER");

        let codes: HashSet<&str> = ROSTER.iter().map(|c| c.short_code).collect();
        assert_eq!(codes.len(), ROSTER.len(), "duplicate short_code in ROSTER");
    }

    /// Guards against exactly the drift this table exists to prevent: if a
    /// club is added/removed/renumbered in the seed files under
    /// `setup/{pl,la_liga,serie_a,bundesliga,ligue_1}/*.rs` without updating
    /// ROSTER (or vice versa), this test fails.
    #[test]
    fn roster_matches_seeded_engine() {
        let mut engine = ValuationEngine::new();
        seed_big_five_leagues(&mut engine);

        let seeded_ids: HashSet<u32> = engine.club_states.iter().map(|c| c.key().0).collect();
        let roster_ids: HashSet<u32> = ROSTER.iter().map(|c| c.id).collect();
        assert_eq!(
            roster_ids, seeded_ids,
            "ROSTER and seeded club_states have diverged"
        );

        for club in ROSTER {
            let seeded_name = engine.names.get(&club.id).map(|n| n.clone());
            assert_eq!(
                seeded_name.as_deref(),
                Some(club.name),
                "name mismatch for club id {}",
                club.id
            );
        }
    }
}
