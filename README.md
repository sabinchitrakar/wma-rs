# Weighted Moving Average

```
use wma_rs::WMA;
use ta_common::traits::Indicator;

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


```