use crate::ValuationEngine;

pub mod psg;
pub mod monaco;
pub mod lille;
pub mod brest;
pub mod nice;
pub mod lyon;
pub mod lens;
pub mod marseille;
pub mod reims;
pub mod rennes;
pub mod toulouse;
pub mod montpellier;
pub mod strasbourg;
pub mod nantes;
pub mod le_havre;
pub mod auxerre;
pub mod angers;
pub mod saint_etienne;
pub mod paris_fc;
pub mod lorient;
pub mod troyes;
pub mod le_mans;

pub fn seed(engine: &mut ValuationEngine) {
    psg::seed(engine);
    monaco::seed(engine);
    lille::seed(engine);
    brest::seed(engine);
    nice::seed(engine);
    lyon::seed(engine);
    lens::seed(engine);
    marseille::seed(engine);
    reims::seed(engine);
    rennes::seed(engine);
    toulouse::seed(engine);
    montpellier::seed(engine);
    strasbourg::seed(engine);
    nantes::seed(engine);
    le_havre::seed(engine);
    auxerre::seed(engine);
    angers::seed(engine);
    saint_etienne::seed(engine);
    paris_fc::seed(engine);
    lorient::seed(engine);
    troyes::seed(engine);
    le_mans::seed(engine);
}
