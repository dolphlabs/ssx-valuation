use crate::ValuationEngine;

pub mod leverkusen;
pub mod bayern_munich;
pub mod stuttgart;
pub mod rb_leipzig;
pub mod dortmund;
pub mod frankfurt;
pub mod hoffenheim;
pub mod heidenheim;
pub mod werder_bremen;
pub mod freiburg;
pub mod augsburg;
pub mod wolfsburg;
pub mod mainz;
pub mod gladbach;
pub mod union_berlin;
pub mod bochum;
pub mod st_pauli;
pub mod holstein_kiel;

pub fn seed(engine: &mut ValuationEngine) {
    leverkusen::seed(engine);
    bayern_munich::seed(engine);
    stuttgart::seed(engine);
    rb_leipzig::seed(engine);
    dortmund::seed(engine);
    frankfurt::seed(engine);
    hoffenheim::seed(engine);
    heidenheim::seed(engine);
    werder_bremen::seed(engine);
    freiburg::seed(engine);
    augsburg::seed(engine);
    wolfsburg::seed(engine);
    mainz::seed(engine);
    gladbach::seed(engine);
    union_berlin::seed(engine);
    bochum::seed(engine);
    st_pauli::seed(engine);
    holstein_kiel::seed(engine);
}
