use pricing_core::{bs_mc, bs_mc_parallel};

#[test]
fn test_bs_mc_t_zero() {
    let s0 = 100.0;
    let k = 110.0;
    let price = bs_mc(10_000, s0, k, 0.03, 0.2, 0.0);
    let expected = (s0 - k).max(0.0);
    assert!((price - expected).abs() < 1e-12);
}

/// Test für Zero-Volatilität: st deterministisch = s0·exp(r·T)
#[test]
fn test_bs_mc_zero_volatility_zero_price() {
    let price = bs_mc(10_000, 100.0, 120.0, 0.05, 0.0, 1.0);
    // st = 100·exp(0.05·1)=~105.127, payoff = max(105.127-120,0)=0
    assert_eq!(price, 0.0);
}

/// Gleicher Test für die parallele Variante
#[test]
fn test_bs_mc_parallel_zero_volatility_zero_price() {
    let price = bs_mc_parallel(10_000, 100.0, 120.0, 0.05, 0.0, 1.0);
    assert_eq!(price, 0.0);
}

/// Test, dass keine NaN oder Infinite zurückgegeben wird
#[test]
fn test_bs_mc_finite() {
    let p1 = bs_mc(1_000, 50.0, 45.0, 0.02, 0.3, 0.5);
    let p2 = bs_mc_parallel(1_000, 50.0, 45.0, 0.02, 0.3, 0.5);
    assert!(p1.is_finite());
    assert!(p2.is_finite());
}