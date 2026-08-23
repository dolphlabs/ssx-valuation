//! Hand-verified mapping of API-Football team ids to our own `ClubId`s.
//! Built by fetching real fixture lists for all 5 leagues and
//! cross-referencing team names against `crate::setup::ROSTER` - see the
//! design notes in the backfill plan for the original normalization
//! process. A static table beats runtime fuzzy-matching here: a wrong
//! silent match would corrupt a club's entire valuation, not just one
//! event.
//!
//! `api_football_team_id` is a stable, season-independent identifier - once
//! a club's row is verified here it never needs to change or be removed,
//! only added to as previously-unmapped (then-relegated) clubs get promoted
//! back into a tracked top flight and are verified for the first time.
//! `ssx-transfers`/`ssx-injuries` iterate `TEAM_MAP` directly to decide
//! which clubs to poll, so entries are never deleted just because a club is
//! *currently* out of its top flight - see `NOT_IN_2026_27` below for the
//! current-season trackability signal instead.

/// `(api_football_team_id, our_club_id)`.
pub const TEAM_MAP: &[(u32, u32)] = &[
    // Premier League
    (50, 1), (33, 2), (47, 3), (42, 4), (40, 5), (49, 6), (34, 7), (66, 8),
    (48, 9), (51, 10), (39, 11), (36, 12), (35, 13), (52, 14), (55, 15),
    (45, 16), (65, 17), (57, 18),
    // La Liga
    (541, 2001), (529, 2002), (530, 2003), (547, 2004), (531, 2005), (548, 2006),
    (543, 2007), (533, 2008), (532, 2009), (542, 2010), (727, 2011), (546, 2012),
    (538, 2013), (536, 2014), (798, 2015), (728, 2017), (540, 2020),
    // Serie A
    (505, 3001), (489, 3002), (496, 3003), (499, 3004), (492, 3005), (497, 3006),
    (487, 3007), (502, 3008), (503, 3009), (500, 3010), (495, 3011), (1579, 3012),
    (504, 3013), (494, 3014), (490, 3015), (867, 3016), (523, 3018), (895, 3019),
    (517, 3020),
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
/// of their league for that season) - not a mapping failure, a real gap.
/// Frozen/historical: describes the completed 2025/26 season specifically
/// (the one `ssx-backfill`'s one-time replay actually processed, at
/// `SEASON = 2025` in `ssx-backfill/src/main.rs`) and is not re-verified
/// each rollover. For "is this club in a tracked top flight *right now*",
/// use `NOT_IN_2026_27` below instead.
pub const NOT_IN_2025_26: &[u32] = &[
    18, 19, 20, // PL: Ipswich Town, Leicester City, Southampton
    2016, 2018, 2019, // La Liga: UD Las Palmas, CD Leganes, Real Valladolid
    3012, 3017, 3020, // Serie A: Monza, Empoli, Venezia
    4016, 4018, // Bundesliga: VfL Bochum, Holstein Kiel
    5009, 5012, // Ligue 1: Stade de Reims, Montpellier HSC
];

/// Rostered clubs with no 2026/27 top-flight fixtures at all (relegated out
/// of their league for the *current* season) - the live counterpart to
/// `NOT_IN_2025_26` above, used by real-time fixture discovery
/// (`ssx-live-oracle`) rather than the historical backfill. Re-verified by
/// live-querying `GET /teams?league={id}&season=2026` for all 5 leagues and
/// cross-referencing against `TEAM_MAP`/`crate::setup::ROSTER`; every id
/// that changed status was independently re-confirmed via a direct
/// `GET /teams?id=` lookup (name + country) before being trusted.
///
/// This set drifts every season (promotions/relegations) and **must be
/// re-verified at every rollover** the same way - do not assume last
/// season's split still holds, and do not derive it from `NOT_IN_2025_26`
/// by assumption.
pub const NOT_IN_2026_27: &[u32] = &[
    9, 11, 19, 20, // PL: West Ham United, Wolverhampton Wanderers, Leicester City, Southampton
    2004, 2015, 2016, 2018, 2019, // La Liga: Girona FC, RCD Mallorca, UD Las Palmas, CD Leganes, Real Valladolid
    3013, 3017, // Serie A: Hellas Verona, Empoli
    4008, 4012, 4016, 4017, 4018, // Bundesliga: FCH Heidenheim, VfL Wolfsburg, VfL Bochum, FC St. Pauli, Holstein Kiel
    5009, 5012, 5014, 5018, // Ligue 1: Stade de Reims, Montpellier HSC, FC Nantes, AS Saint-Etienne
];

pub fn club_id_for_api_team(api_team_id: u32) -> Option<crate::ClubId> {
    TEAM_MAP.iter().find(|(api, _)| *api == api_team_id).map(|(_, club)| crate::ClubId(*club))
}

/// Reverse lookup - the live transfers poller iterates our own tracked
/// clubs and needs the API's team id to query `/transfers?team=`.
pub fn api_team_for_club_id(club_id: crate::ClubId) -> Option<u32> {
    TEAM_MAP.iter().find(|(_, club)| *club == club_id.0).map(|(api, _)| *api)
}

/// Whether a rostered club is currently playing in one of the 5 tracked
/// top-flight leagues this season - an *additional* filter, layered on top
/// of `TEAM_MAP`'s identity mapping, that `ssx-live-oracle`'s real-fixture
/// discovery (upcoming and live) uses so a club that's mapped but relegated
/// out this season (e.g. West Ham, still `TEAM_MAP`-verified so
/// `ssx-transfers`/`ssx-injuries` keep polling it) never gets surfaced as a
/// real Match Pool candidate - including the edge case of two
/// still-mapped-but-relegated clubs meeting in a lower-division match that
/// `/fixtures?live=all` would otherwise report as "both legs tracked".
///
/// Note this is *not* the inverse of "unmapped": a club that's never been
/// verified in `TEAM_MAP` at all (e.g. Leicester City, Southampton - out of
/// the top flight for two seasons running now) is also excluded here for
/// completeness/reporting, even though `club_id_for_api_team` alone would
/// already exclude it from any real discovery (no api id to match against).
pub fn is_in_current_top_flight(club_id: crate::ClubId) -> bool {
    !NOT_IN_2026_27.contains(&club_id.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn team_map_has_no_duplicate_club_or_api_ids() {
        let mapped: std::collections::HashSet<u32> = TEAM_MAP.iter().map(|(_, club)| *club).collect();
        assert_eq!(mapped.len(), TEAM_MAP.len(), "duplicate club id in TEAM_MAP");

        let api_ids: std::collections::HashSet<u32> = TEAM_MAP.iter().map(|(api, _)| *api).collect();
        assert_eq!(api_ids.len(), TEAM_MAP.len(), "duplicate api_football_team_id in TEAM_MAP");
    }

    #[test]
    fn exclusion_lists_have_no_duplicates() {
        let a: std::collections::HashSet<u32> = NOT_IN_2025_26.iter().copied().collect();
        assert_eq!(a.len(), NOT_IN_2025_26.len(), "duplicate club id in NOT_IN_2025_26");

        let b: std::collections::HashSet<u32> = NOT_IN_2026_27.iter().copied().collect();
        assert_eq!(b.len(), NOT_IN_2026_27.len(), "duplicate club id in NOT_IN_2026_27");
    }

    #[test]
    fn a_club_still_team_mapped_but_out_of_the_current_top_flight_is_excluded() {
        // West Ham United - relegated for 2026/27, but TEAM_MAP keeps its
        // entry so ssx-transfers/ssx-injuries keep polling it.
        assert!(TEAM_MAP.iter().any(|(_, club)| *club == 9));
        assert!(!is_in_current_top_flight(crate::ClubId(9)));
    }

    #[test]
    fn a_club_in_this_seasons_top_flight_is_included() {
        assert!(is_in_current_top_flight(crate::ClubId(1))); // Manchester City
    }
}
