//! Hand-verified mapping of API-Football team ids (2025/26 season) to our
//! own `ClubId`s. Built by fetching the real 2025/26 fixture lists for all 5
//! leagues and cross-referencing team names against `crate::setup::ROSTER` -
//! see the design notes in the backfill plan for the normalization process
//! used to derive this. A static table beats runtime fuzzy-matching here: a
//! wrong silent match would corrupt a club's entire valuation, not just one
//! event.
//!
//! 83 of our 96 rostered clubs play in the real 2025/26 top flight of their
//! league. The other 13 were relegated out of their league for this season
//! and simply have no real fixtures/transfers to replay - their seed value
//! is left unchanged by this backfill, and `NOT_IN_2025_26` lists them
//! explicitly so callers can report that rather than silently doing nothing.

/// `(api_football_team_id, our_club_id)`.
pub const TEAM_MAP: &[(u32, u32)] = &[
    // Premier League
    (50, 1), (33, 2), (47, 3), (42, 4), (40, 5), (49, 6), (34, 7), (66, 8),
    (48, 9), (51, 10), (39, 11), (36, 12), (35, 13), (52, 14), (55, 15),
    (45, 16), (65, 17),
    // La Liga
    (541, 2001), (529, 2002), (530, 2003), (547, 2004), (531, 2005), (548, 2006),
    (543, 2007), (533, 2008), (532, 2009), (542, 2010), (727, 2011), (546, 2012),
    (538, 2013), (536, 2014), (798, 2015), (728, 2017), (540, 2020),
    // Serie A
    (505, 3001), (489, 3002), (496, 3003), (499, 3004), (492, 3005), (497, 3006),
    (487, 3007), (502, 3008), (503, 3009), (500, 3010), (495, 3011), (504, 3013),
    (494, 3014), (490, 3015), (867, 3016), (523, 3018), (895, 3019),
    // Bundesliga
    (168, 4001), (157, 4002), (172, 4003), (173, 4004), (165, 4005), (169, 4006),
    (167, 4007), (180, 4008), (162, 4009), (160, 4010), (170, 4011), (161, 4012),
    (164, 4013), (163, 4014), (182, 4015), (186, 4017),
    // Ligue 1
    (85, 5001), (91, 5002), (79, 5003), (106, 5004), (84, 5005), (80, 5006),
    (116, 5007), (81, 5008), (94, 5010), (96, 5011), (95, 5013), (83, 5014),
    (111, 5015), (108, 5016), (77, 5017), (1063, 5018),
];

/// Rostered clubs with no 2025/26 top-flight fixtures at all (relegated out
/// of their league for this season) - not a mapping failure, a real gap.
pub const NOT_IN_2025_26: &[u32] = &[
    18, 19, 20, // PL: Ipswich Town, Leicester City, Southampton
    2016, 2018, 2019, // La Liga: UD Las Palmas, CD Leganes, Real Valladolid
    3012, 3017, 3020, // Serie A: Monza, Empoli, Venezia
    4016, 4018, // Bundesliga: VfL Bochum, Holstein Kiel
    5009, 5012, // Ligue 1: Stade de Reims, Montpellier HSC
];

pub fn club_id_for_api_team(api_team_id: u32) -> Option<crate::ClubId> {
    TEAM_MAP.iter().find(|(api, _)| *api == api_team_id).map(|(_, club)| crate::ClubId(*club))
}

/// Reverse lookup - the live transfers poller iterates our own tracked
/// clubs and needs the API's team id to query `/transfers?team=`.
pub fn api_team_for_club_id(club_id: crate::ClubId) -> Option<u32> {
    TEAM_MAP.iter().find(|(_, club)| *club == club_id.0).map(|(api, _)| *api)
}
