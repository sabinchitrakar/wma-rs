#![feature(external_doc)]

use ta_common::fixed_queue::FixedQueue;
use ta_common::traits::Indicator;


#[doc(include = "../README.md")]
pub struct WMA {
    period: u32,
    history: FixedQueue<f64>,
    divisor: f64,

}

impl WMA {
    pub fn new(period: u32) -> WMA {
        Self {
            period,
            history: FixedQueue::new(period),
            divisor: (period * (period + 1) / 2) as f64,
        }
    }
}

impl Indicator<f64, Option<f64>> for WMA {
    fn next(&mut self, input: f64) -> Option<f64> {
        self.history.add(input);
        if self.history.is_full() {
            let mut wma = 0.0;
            for i in 0..self.history.size() as i32 {
                wma += self.history.at(i).map(|v| v * (i + 1) as f64).unwrap();
            }
            return Some(wma / self.divisor);
        }
        return None;
    }

    fn reset(&mut self) {
        self.history.reset();
    }
}


#[cfg(test)]
mod tests {
    use crate::WMA;
    use ta_common::traits::Indicator;

    #[test]
    fn it_works() {
        let mut wma = WMA::new(5);
        assert_eq!(wma.next(81.59), None);
        assert_eq!(wma.next(81.06), None);
        assert_eq!(wma.next(82.87), None);
        assert_eq!(wma.next(83.00), None);
        assert_eq!(wma.next(83.61), Some(82.82466666666667));
        assert_eq!(wma.next(83.15), Some(83.066));
        assert_eq!(wma.next(82.84), Some(83.1));
        assert_eq!(wma.next(83.99), Some(83.39866666666667));
        assert_eq!(wma.next(84.55), Some(83.80933333333334));
        assert_eq!(wma.next(84.36), Some(84.05333333333333));
        assert_eq!(wma.next(85.53), Some(84.63733333333333));
        assert_eq!(wma.next(86.54), Some(85.39933333333333));
        assert_eq!(wma.next(86.89), Some(86.03133333333334));
        assert_eq!(wma.next(87.77), Some(86.76333333333332));
        assert_eq!(wma.next(87.29), Some(87.12066666666666));
    }
}
