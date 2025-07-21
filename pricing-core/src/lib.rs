use rand::prelude::*;
use rand_distr::StandardNormal;
use rayon::prelude::*;

/// Simuliert den Preis einer europäischen Call‑Option per Black‑Scholes Monte‑Carlo.
/// 
/// # Parameter
/// - `paths`: Anzahl der Pfade
/// - `s0`: Spot‑Preis zu t=0
/// - `k`: Strike‑Preis
/// - `r`: risikofreier Zinssatz (annualisiert)
/// - `sigma`: Volatilität (annualisiert)
/// - `t`: Zeit bis zur Fälligkeit (in Jahren)
///
/// # Rückgabe
/// Erwarteter, abgezinster Payoff als f64


pub fn bs_mc(
    paths: usize,
    s0: f64,
    k: f64,
    r: f64,
    sigma: f64,
    t: f64,
) -> f64 {
    let mut rng = thread_rng();
    let drift = (r - 0.5 * sigma * sigma) * t;
    let vol_sqrt = sigma * t.sqrt();
    let mut sum_payoff = 0.0_f64;

    for _ in 0..paths {
        let z: f64 = rng.sample(StandardNormal);
        let st = s0 * (drift + vol_sqrt * z).exp();
        let payoff = (st - k).max(0.0);
        sum_payoff += payoff;
    }

    // Abzinsen auf t=0
    let discounted = (sum_payoff / (paths as f64)) * (-r * t).exp();
    discounted
}

/// Parallele Variante des Black‑Scholes MC.
pub fn bs_mc_parallel(
    paths: usize,
    s0: f64,
    k: f64,
    r: f64,
    sigma: f64,
    t: f64,
) -> f64 {
    let drift = (r - 0.5 * sigma * sigma) * t;
    let vol_sqrt = sigma * t.sqrt();

    let sum_payoff: f64 = (0..paths)
        .into_par_iter()
        .map_init(
            || thread_rng(),
            |rng, _| {
                let z: f64 = rng.sample(StandardNormal);
                let st = s0 * (drift + vol_sqrt * z).exp();
                (st - k).max(0.0)
            },
        )
        .sum();

    (sum_payoff / (paths as f64)) * (-r * t).exp()
}
