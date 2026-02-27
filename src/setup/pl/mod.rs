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

pub fn seed(engine: &mut ValuationEngine) {
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
