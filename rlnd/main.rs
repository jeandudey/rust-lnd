extern crate lightning;
extern crate rlnd_chainintf;

use lightning::chain::chaininterface::{ConfirmationTarget, FeeEstimator as FeeEst};
use rlnd_chainintf::bitcoind::FeeEstimator;

fn main() {
    let fee_estimator = FeeEstimator::new("http://localhost:18332".to_string(),
                                          Some("root".to_string()),
                                          Some("testnet".to_string()));
	let sat_per_kw = fee_estimator
        .get_est_sat_per_1000_weight(ConfirmationTarget::Background);
    println!("{}", sat_per_kw);
}
