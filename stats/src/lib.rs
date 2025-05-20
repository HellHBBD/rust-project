use rand::distr::Distribution;
use rand_distr::{Geometric, Weibull};

pub fn get_ber_days() -> u32 {
    let model = Geometric::new(1.0 / 10.0).expect("fail to create model");

    let trials: u64 = model.sample(&mut rand::rng());
    (trials + 1) as u32
}

pub fn get_weibull_days() -> u32 {
    // 指定 Weibull 分布型別為 f64
    let weibull = Weibull::<f64>::new(3.4, 2.5).expect("Invalid parameters");

    // 明確為 f64 才能正確呼叫 .round()
    let days = weibull.sample(&mut rand::rng()).round();
    days.max(1.0) as u32
}
