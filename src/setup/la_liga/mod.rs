use crate::ValuationEngine;

pub mod real_madrid;
pub mod barcelona;
pub mod atletico_madrid;
pub mod girona;
pub mod athletic_club;
pub mod real_sociedad;
pub mod real_betis;
pub mod villarreal;
pub mod valencia;
pub mod alaves;
pub mod osasuna;
pub mod getafe;
pub mod celta_vigo;
pub mod sevilla;
pub mod mallorca;
pub mod las_palmas;
pub mod rayo_vallecano;
pub mod leganes;
pub mod real_valladolid;
pub mod espanyol;

pub fn seed(engine: &mut ValuationEngine) {
    real_madrid::seed(engine);
    barcelona::seed(engine);
    atletico_madrid::seed(engine);
    girona::seed(engine);
    athletic_club::seed(engine);
    real_sociedad::seed(engine);
    real_betis::seed(engine);
    villarreal::seed(engine);
    valencia::seed(engine);
    alaves::seed(engine);
    osasuna::seed(engine);
    getafe::seed(engine);
    celta_vigo::seed(engine);
    sevilla::seed(engine);
    mallorca::seed(engine);
    las_palmas::seed(engine);
    rayo_vallecano::seed(engine);
    leganes::seed(engine);
    real_valladolid::seed(engine);
    espanyol::seed(engine);
}
