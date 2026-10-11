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
    /// The name fans actually say ("Man City", "Wolves", "PSG") - what the
    /// Fan Zone's plain-words copy and share cards use where the full name
    /// is a mouthful. `name` stays the formal one.
    pub short_name: &'static str,
    pub short_code: &'static str,
    pub league: &'static str,
    /// Primary kit colour as a hex string - drives the crest discs and the
    /// scoreboard in the terminal. Text colour on top of it is picked for
    /// contrast client-side, so light kits (Fulham, Juventus) stay legible.
    pub color: &'static str,
}

const PL: &str = "Premier League";
const LA_LIGA: &str = "La Liga";
const SERIE_A: &str = "Serie A";
const BUNDESLIGA: &str = "Bundesliga";
const LIGUE_1: &str = "Ligue 1";

pub const ROSTER: &[ClubMeta] = &[
    // Premier League
    ClubMeta { id: 1, name: "Manchester City", short_name: "Man City", short_code: "MCI", league: PL, color: "#6CABDD" },
    ClubMeta { id: 2, name: "Manchester United", short_name: "Man United", short_code: "MUN", league: PL, color: "#DA291C" },
    ClubMeta { id: 3, name: "Tottenham Hotspur", short_name: "Tottenham", short_code: "TOT", league: PL, color: "#132257" },
    ClubMeta { id: 4, name: "Arsenal", short_name: "Arsenal", short_code: "ARS", league: PL, color: "#EF0107" },
    ClubMeta { id: 5, name: "Liverpool", short_name: "Liverpool", short_code: "LIV", league: PL, color: "#C8102E" },
    ClubMeta { id: 6, name: "Chelsea", short_name: "Chelsea", short_code: "CHE", league: PL, color: "#034694" },
    ClubMeta { id: 7, name: "Newcastle United", short_name: "Newcastle", short_code: "NEW", league: PL, color: "#241F20" },
    ClubMeta { id: 8, name: "Aston Villa", short_name: "Aston Villa", short_code: "AVL", league: PL, color: "#670E36" },
    ClubMeta { id: 9, name: "West Ham United", short_name: "West Ham", short_code: "WHU", league: PL, color: "#7A263A" },
    ClubMeta { id: 10, name: "Brighton & Hove Albion", short_name: "Brighton", short_code: "BHA", league: PL, color: "#0057B8" },
    ClubMeta { id: 11, name: "Wolverhampton Wanderers", short_name: "Wolves", short_code: "WOL", league: PL, color: "#FDB913" },
    ClubMeta { id: 12, name: "Fulham", short_name: "Fulham", short_code: "FUL", league: PL, color: "#E6E6E6" },
    ClubMeta { id: 13, name: "Bournemouth", short_name: "Bournemouth", short_code: "BOU", league: PL, color: "#DA291C" },
    ClubMeta { id: 14, name: "Crystal Palace", short_name: "Palace", short_code: "CRY", league: PL, color: "#1B458F" },
    ClubMeta { id: 15, name: "Brentford", short_name: "Brentford", short_code: "BRE", league: PL, color: "#E30613" },
    ClubMeta { id: 16, name: "Everton", short_name: "Everton", short_code: "EVE", league: PL, color: "#003399" },
    ClubMeta { id: 17, name: "Nottingham Forest", short_name: "Forest", short_code: "NFO", league: PL, color: "#DD0000" },
    ClubMeta { id: 18, name: "Ipswich Town", short_name: "Ipswich", short_code: "IPS", league: PL, color: "#3A64A3" },
    ClubMeta { id: 19, name: "Leicester City", short_name: "Leicester", short_code: "LEI", league: PL, color: "#003090" },
    ClubMeta { id: 20, name: "Southampton", short_name: "Southampton", short_code: "SOU", league: PL, color: "#D71920" },
    // La Liga
    ClubMeta { id: 2001, name: "Real Madrid", short_name: "Real Madrid", short_code: "RMA", league: LA_LIGA, color: "#FEBE10" },
    ClubMeta { id: 2002, name: "FC Barcelona", short_name: "Barcelona", short_code: "BAR", league: LA_LIGA, color: "#A50044" },
    ClubMeta { id: 2003, name: "Atletico Madrid", short_name: "Atletico", short_code: "ATM", league: LA_LIGA, color: "#CB3524" },
    ClubMeta { id: 2004, name: "Girona FC", short_name: "Girona", short_code: "GIR", league: LA_LIGA, color: "#CD2534" },
    ClubMeta { id: 2005, name: "Athletic Club", short_name: "Athletic", short_code: "ATH", league: LA_LIGA, color: "#EE2523" },
    ClubMeta { id: 2006, name: "Real Sociedad", short_name: "Real Sociedad", short_code: "RSO", league: LA_LIGA, color: "#0067B1" },
    ClubMeta { id: 2007, name: "Real Betis", short_name: "Betis", short_code: "BET", league: LA_LIGA, color: "#00954C" },
    ClubMeta { id: 2008, name: "Villarreal CF", short_name: "Villarreal", short_code: "VIL", league: LA_LIGA, color: "#FFE667" },
    ClubMeta { id: 2009, name: "Valencia CF", short_name: "Valencia", short_code: "VAL", league: LA_LIGA, color: "#EE3524" },
    ClubMeta { id: 2010, name: "Deportivo Alaves", short_name: "Alaves", short_code: "ALA", league: LA_LIGA, color: "#0761AF" },
    ClubMeta { id: 2011, name: "CA Osasuna", short_name: "Osasuna", short_code: "OSA", league: LA_LIGA, color: "#D91A21" },
    ClubMeta { id: 2012, name: "Getafe CF", short_name: "Getafe", short_code: "GET", league: LA_LIGA, color: "#005999" },
    ClubMeta { id: 2013, name: "RC Celta", short_name: "Celta", short_code: "CEL", league: LA_LIGA, color: "#8AC3EE" },
    ClubMeta { id: 2014, name: "Sevilla FC", short_name: "Sevilla", short_code: "SEV", league: LA_LIGA, color: "#D4021D" },
    ClubMeta { id: 2015, name: "RCD Mallorca", short_name: "Mallorca", short_code: "MLL", league: LA_LIGA, color: "#E20613" },
    ClubMeta { id: 2016, name: "UD Las Palmas", short_name: "Las Palmas", short_code: "LPA", league: LA_LIGA, color: "#FFE400" },
    ClubMeta { id: 2017, name: "Rayo Vallecano", short_name: "Rayo", short_code: "RAY", league: LA_LIGA, color: "#E53027" },
    ClubMeta { id: 2018, name: "CD Leganes", short_name: "Leganes", short_code: "LEG", league: LA_LIGA, color: "#005AA9" },
    ClubMeta { id: 2019, name: "Real Valladolid", short_name: "Valladolid", short_code: "VLL", league: LA_LIGA, color: "#5C2483" },
    ClubMeta { id: 2020, name: "RCD Espanyol", short_name: "Espanyol", short_code: "ESP", league: LA_LIGA, color: "#007FC8" },
    // Serie A
    ClubMeta { id: 3001, name: "Inter Milan", short_name: "Inter", short_code: "INT", league: SERIE_A, color: "#0068A8" },
    ClubMeta { id: 3002, name: "AC Milan", short_name: "Milan", short_code: "ACM", league: SERIE_A, color: "#FB090B" },
    ClubMeta { id: 3003, name: "Juventus", short_name: "Juventus", short_code: "JUV", league: SERIE_A, color: "#E6E6E6" },
    ClubMeta { id: 3004, name: "Atalanta", short_name: "Atalanta", short_code: "ATA", league: SERIE_A, color: "#1E71B8" },
    ClubMeta { id: 3005, name: "Napoli", short_name: "Napoli", short_code: "NAP", league: SERIE_A, color: "#12A0D7" },
    ClubMeta { id: 3006, name: "AS Roma", short_name: "Roma", short_code: "ROM", league: SERIE_A, color: "#8E1F2F" },
    ClubMeta { id: 3007, name: "Lazio", short_name: "Lazio", short_code: "LAZ", league: SERIE_A, color: "#87D8F7" },
    ClubMeta { id: 3008, name: "Fiorentina", short_name: "Fiorentina", short_code: "FIO", league: SERIE_A, color: "#482E92" },
    ClubMeta { id: 3009, name: "Torino", short_name: "Torino", short_code: "TOR", league: SERIE_A, color: "#8A1E03" },
    ClubMeta { id: 3010, name: "Bologna", short_name: "Bologna", short_code: "BOL", league: SERIE_A, color: "#1A2F48" },
    ClubMeta { id: 3011, name: "Genoa", short_name: "Genoa", short_code: "GEN", league: SERIE_A, color: "#AD1919" },
    ClubMeta { id: 3012, name: "Monza", short_name: "Monza", short_code: "MZA", league: SERIE_A, color: "#E30613" },
    ClubMeta { id: 3013, name: "Hellas Verona", short_name: "Verona", short_code: "VER", league: SERIE_A, color: "#FFD200" },
    ClubMeta { id: 3014, name: "Udinese", short_name: "Udinese", short_code: "UDI", league: SERIE_A, color: "#E6E6E6" },
    ClubMeta { id: 3015, name: "Cagliari", short_name: "Cagliari", short_code: "CAG", league: SERIE_A, color: "#A50E2E" },
    ClubMeta { id: 3016, name: "Lecce", short_name: "Lecce", short_code: "LEC", league: SERIE_A, color: "#FFED00" },
    ClubMeta { id: 3017, name: "Empoli", short_name: "Empoli", short_code: "EMP", league: SERIE_A, color: "#005BAC" },
    ClubMeta { id: 3018, name: "Parma", short_name: "Parma", short_code: "PAR", league: SERIE_A, color: "#FFD200" },
    ClubMeta { id: 3019, name: "Como", short_name: "Como", short_code: "COM", league: SERIE_A, color: "#004B87" },
    ClubMeta { id: 3020, name: "Venezia", short_name: "Venezia", short_code: "VEN", league: SERIE_A, color: "#F47920" },
    // Bundesliga
    ClubMeta { id: 4001, name: "Bayer Leverkusen", short_name: "Leverkusen", short_code: "LEV", league: BUNDESLIGA, color: "#E32221" },
    ClubMeta { id: 4002, name: "Bayern Munich", short_name: "Bayern", short_code: "BAY", league: BUNDESLIGA, color: "#DC052D" },
    ClubMeta { id: 4003, name: "VfB Stuttgart", short_name: "Stuttgart", short_code: "VFB", league: BUNDESLIGA, color: "#E32219" },
    ClubMeta { id: 4004, name: "RB Leipzig", short_name: "Leipzig", short_code: "RBL", league: BUNDESLIGA, color: "#DD0741" },
    ClubMeta { id: 4005, name: "Borussia Dortmund", short_name: "Dortmund", short_code: "BVB", league: BUNDESLIGA, color: "#FDE100" },
    ClubMeta { id: 4006, name: "Eintracht Frankfurt", short_name: "Frankfurt", short_code: "SGE", league: BUNDESLIGA, color: "#E1000F" },
    ClubMeta { id: 4007, name: "TSG Hoffenheim", short_name: "Hoffenheim", short_code: "TSG", league: BUNDESLIGA, color: "#1961B5" },
    ClubMeta { id: 4008, name: "FCH Heidenheim", short_name: "Heidenheim", short_code: "HDH", league: BUNDESLIGA, color: "#E2001A" },
    ClubMeta { id: 4009, name: "Werder Bremen", short_name: "Bremen", short_code: "SVW", league: BUNDESLIGA, color: "#1D9053" },
    ClubMeta { id: 4010, name: "SC Freiburg", short_name: "Freiburg", short_code: "SCF", league: BUNDESLIGA, color: "#E2001A" },
    ClubMeta { id: 4011, name: "FC Augsburg", short_name: "Augsburg", short_code: "FCA", league: BUNDESLIGA, color: "#BA3733" },
    ClubMeta { id: 4012, name: "VfL Wolfsburg", short_name: "Wolfsburg", short_code: "WOB", league: BUNDESLIGA, color: "#65B32E" },
    ClubMeta { id: 4013, name: "Mainz 05", short_name: "Mainz", short_code: "M05", league: BUNDESLIGA, color: "#C3141E" },
    ClubMeta { id: 4014, name: "Borussia Monchengladbach", short_name: "Gladbach", short_code: "BMG", league: BUNDESLIGA, color: "#E6E6E6" },
    ClubMeta { id: 4015, name: "Union Berlin", short_name: "Union Berlin", short_code: "FCU", league: BUNDESLIGA, color: "#EB1923" },
    ClubMeta { id: 4016, name: "VfL Bochum", short_name: "Bochum", short_code: "BOC", league: BUNDESLIGA, color: "#005BA4" },
    ClubMeta { id: 4017, name: "FC St. Pauli", short_name: "St. Pauli", short_code: "STP", league: BUNDESLIGA, color: "#6E4C33" },
    ClubMeta { id: 4018, name: "Holstein Kiel", short_name: "Holstein Kiel", short_code: "KIE", league: BUNDESLIGA, color: "#00467F" },
    // Ligue 1
    ClubMeta { id: 5001, name: "Paris Saint-Germain", short_name: "PSG", short_code: "PSG", league: LIGUE_1, color: "#004170" },
    ClubMeta { id: 5002, name: "AS Monaco", short_name: "Monaco", short_code: "ASM", league: LIGUE_1, color: "#E7192B" },
    ClubMeta { id: 5003, name: "Lille OSC", short_name: "Lille", short_code: "LIL", league: LIGUE_1, color: "#E01E13" },
    ClubMeta { id: 5004, name: "Stade Brestois 29", short_name: "Brest", short_code: "BRS", league: LIGUE_1, color: "#E1000F" },
    ClubMeta { id: 5005, name: "OGC Nice", short_name: "Nice", short_code: "NIC", league: LIGUE_1, color: "#C4161C" },
    ClubMeta { id: 5006, name: "Olympique Lyonnais", short_name: "Lyon", short_code: "OLY", league: LIGUE_1, color: "#1A3D8F" },
    ClubMeta { id: 5007, name: "RC Lens", short_name: "Lens", short_code: "RCL", league: LIGUE_1, color: "#FFD100" },
    ClubMeta { id: 5008, name: "Olympique de Marseille", short_name: "Marseille", short_code: "OM", league: LIGUE_1, color: "#2FAEE0" },
    ClubMeta { id: 5009, name: "Stade de Reims", short_name: "Reims", short_code: "REI", league: LIGUE_1, color: "#E2001A" },
    ClubMeta { id: 5010, name: "Stade Rennais FC", short_name: "Rennes", short_code: "REN", league: LIGUE_1, color: "#E13327" },
    ClubMeta { id: 5011, name: "Toulouse FC", short_name: "Toulouse", short_code: "TFC", league: LIGUE_1, color: "#5B2D8E" },
    ClubMeta { id: 5012, name: "Montpellier HSC", short_name: "Montpellier", short_code: "MTP", league: LIGUE_1, color: "#F26522" },
    ClubMeta { id: 5013, name: "RC Strasbourg Alsace", short_name: "Strasbourg", short_code: "RCS", league: LIGUE_1, color: "#009FE3" },
    ClubMeta { id: 5014, name: "FC Nantes", short_name: "Nantes", short_code: "NAN", league: LIGUE_1, color: "#FCD405" },
    ClubMeta { id: 5015, name: "Le Havre AC", short_name: "Le Havre", short_code: "HAC", league: LIGUE_1, color: "#87B6D8" },
    ClubMeta { id: 5016, name: "AJ Auxerre", short_name: "Auxerre", short_code: "AUX", league: LIGUE_1, color: "#005BAC" },
    ClubMeta { id: 5017, name: "Angers SCO", short_name: "Angers", short_code: "ANG", league: LIGUE_1, color: "#E6E6E6" },
    ClubMeta { id: 5018, name: "AS Saint-Etienne", short_name: "Saint-Etienne", short_code: "STE", league: LIGUE_1, color: "#00A859" },
];

/// The names fans give some of the seeded rivalries ("Manchester derby").
/// Only real, established names: a rivalry without one is just "rivals" in
/// the UI. Each pair is a rivalry the engine itself seeds (tested below),
/// in either order.
pub const DERBIES: &[(u32, u32, &str)] = &[
    (1, 2, "Manchester derby"),
    (2, 5, "North West derby"),
    (3, 4, "North London derby"),
    (5, 16, "Merseyside derby"),
    (3, 6, "London derby"),
    (3, 9, "London derby"),
    (4, 6, "London derby"),
    (4, 9, "London derby"),
    (6, 9, "London derby"),
    (6, 12, "West London derby"),
    (6, 15, "West London derby"),
    (12, 15, "West London derby"),
    (8, 11, "West Midlands derby"),
    (10, 14, "M23 derby"),
    (13, 20, "South Coast derby"),
    (17, 19, "East Midlands derby"),
    (2001, 2002, "El Clásico"),
    (2001, 2003, "Madrid derby"),
    (2002, 2020, "Barcelona derby"),
    (2005, 2006, "Basque derby"),
    (2007, 2014, "Seville derby"),
    (2008, 2009, "Valencian Community derby"),
    (3001, 3002, "Milan derby"),
    (3001, 3003, "Derby d'Italia"),
    (3003, 3009, "Turin derby"),
    (3006, 3007, "Rome derby"),
    (4002, 4005, "Der Klassiker"),
    (5001, 5008, "Le Classique"),
];

/// The derby name for two clubs, in either order.
pub fn derby_name(a: u32, b: u32) -> Option<&'static str> {
    DERBIES.iter().find(|(x, y, _)| (*x == a && *y == b) || (*x == b && *y == a)).map(|(_, _, n)| *n)
}

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

    #[test]
    fn every_club_has_a_short_fan_name() {
        // Short enough for a chip or a share card, and never longer than
        // the formal name it stands in for.
        for club in ROSTER {
            assert!(!club.short_name.is_empty() && club.short_name.len() <= 14, "{} short_name {:?}", club.name, club.short_name);
            assert!(club.short_name.len() <= club.name.len(), "{} short_name is longer than its name", club.name);
        }
    }

    #[test]
    fn every_club_colour_is_a_six_digit_hex() {
        for club in ROSTER {
            let hex = club.color.strip_prefix('#').unwrap_or("");
            assert!(
                hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()),
                "club {} has a malformed colour {:?}",
                club.id,
                club.color
            );
        }
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

    #[test]
    fn every_derby_is_a_seeded_rivalry() {
        let mut engine = ValuationEngine::new();
        seed_big_five_leagues(&mut engine);
        let rivals = |a: u32, b: u32| {
            engine.club_states.get(&crate::ClubId(a)).is_some_and(|c| c.rivals.iter().any(|(id, _)| id.0 == b))
        };
        let mut seen = HashSet::new();
        for (a, b, name) in DERBIES {
            assert!(rivals(*a, *b) || rivals(*b, *a), "{name}: {a} and {b} aren't seeded rivals");
            assert!(seen.insert((*a.min(b), *a.max(b))), "{name} listed twice");
        }
        assert_eq!(derby_name(2, 1), Some("Manchester derby"));
        assert_eq!(derby_name(1, 4), None);
    }
}
