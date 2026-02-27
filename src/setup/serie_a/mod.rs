use crate::ValuationEngine;

pub mod inter_milan;
pub mod ac_milan;
pub mod juventus;
pub mod atalanta;
pub mod napoli;
pub mod roma;
pub mod lazio;
pub mod fiorentina;
pub mod torino;
pub mod bologna;
pub mod genoa;
pub mod monza;
pub mod verona;
pub mod udinese;
pub mod cagliari;
pub mod lecce;
pub mod empoli;
pub mod parma;
pub mod como;
pub mod venezia;

pub fn seed(engine: &mut ValuationEngine) {
    inter_milan::seed(engine);
    ac_milan::seed(engine);
    juventus::seed(engine);
    atalanta::seed(engine);
    napoli::seed(engine);
    roma::seed(engine);
    lazio::seed(engine);
    fiorentina::seed(engine);
    torino::seed(engine);
    bologna::seed(engine);
    genoa::seed(engine);
    monza::seed(engine);
    verona::seed(engine);
    udinese::seed(engine);
    cagliari::seed(engine);
    lecce::seed(engine);
    empoli::seed(engine);
    parma::seed(engine);
    como::seed(engine);
    venezia::seed(engine);
}
