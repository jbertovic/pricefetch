
/// finds min of series
pub fn min(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        None
    } else {
        let mut v = series.to_vec();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Some(v[0])
    }
}
/// finds max of series
pub fn max(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        None
    } else {
        let mut v = series.to_vec();
        v.sort_by(|a, b| b.partial_cmp(a).unwrap());
        Some(v[0])
    }
}
/// moving simple moving average based on supplied window length n
pub fn n_window_sma(n: usize, series: &[f64]) -> Option<Vec<f64>> {
    if series.len() < n { 
        None 
    } else {
        let sma: Vec<f64> = series.windows(n).map(|slice|slice.iter().sum::<f64>()/(n as f64)).collect();
        Some(sma)
    }
}
/// price change from start of series to end of series
/// return relative change and absolute difference as a tuple
/// assumes series can't have zero price
pub fn price_diff(series: &[f64]) -> Option<(f64, f64)> {
    if series.len() < 2 {
        None
    } else {
        let start = *series.first().unwrap();
        let end = *series.last().unwrap();
        if end==start {
            Some((0.0, 0.0))
        } else {
            Some(((end-start)/start, end-start))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minmax_series_pick_correct() {
        let test_num: Vec<f64> = vec!(1.0,2.0,3.0);
        assert_eq!(min(&test_num), Some(1.0));
        assert_eq!(max(&test_num), Some(3.0));
    }
    #[test]
    fn minmax_two_same_numbers_picks_one() {
        let test_num: Vec<f64> = vec!(1.0,1.0);
        assert_eq!(min(&test_num), Some(1.0));
        assert_eq!(max(&test_num), Some(1.0));
    }
    #[test]
    fn minmax_only_one_number_picks_one() {
        let test_num: Vec<f64> = vec!(1.0);
        assert_eq!(min(&test_num), Some(1.0));
        assert_eq!(max(&test_num), Some(1.0));
    }
    #[test]
    fn minmax_can_pick_correct_with_two_negative_numbers() {
        let test_num: Vec<f64> = vec!(-1.0, -5.0, -10.0);
        assert_eq!(min(&test_num), Some(-10.0));
        assert_eq!(max(&test_num), Some(-1.0));
    }
    #[test]
    fn minmax_empty_returns_none() {
        let test_num: Vec<f64> = vec!();
        assert_eq!(min(&test_num), None);
        assert_eq!(max(&test_num), None);
    }
    #[test]
    fn sma_check_forumla_with_positive_numbers() {
        let test_num: Vec<f64> = 
            vec!(1.0,10.0,15.0,10.0,12.0,15.0,13.0,16.0,15.0,15.0,20.0,15.0,17.0);
        let ans_5: Vec<f64> = vec!(9.6,12.4,13.0,13.2,14.2,14.8,15.8,16.2,16.4);
        let ans_10: Vec<f64> = vec!(12.2,14.1,14.6,14.8);
        assert_eq!(n_window_sma(1, &test_num), Some(test_num.clone()));
        assert_eq!(n_window_sma(5, &test_num), Some(ans_5));
        assert_eq!(n_window_sma(10, &test_num), Some(ans_10));
    }
    #[test]
    fn sma_empty_returns_none() {
        let test_num: Vec<f64> = vec!();
        assert_eq!(n_window_sma(1, &test_num), None);
    }
    #[test]
    fn sma_lessthan_nwindow_returns_none() {
        let test_num: Vec<f64> = vec!(1.0,2.0,3.0);
        assert_eq!(n_window_sma(3, &test_num), Some(vec!(2.0 as f64)));
        assert_eq!(n_window_sma(4, &test_num), None);
    }
    #[test]
    fn price_diff_equal_numbers_returns_zerochange() {
        assert_eq!(price_diff(&[1.0, 1.0]), Some((0.0, 0.0)));
    }
    #[test]
    fn price_diff_nonumbers_or_onenumber_returns_none() {
        assert_eq!(price_diff(&[]), None);
        assert_eq!(price_diff(&[1.0]), None);
    }
    #[test]
    fn price_diff_positivechg_returns_positivechg() {
        assert_eq!(price_diff(&[10.0, 11.0, 12.0]), Some((0.2, 2.0)))
    }
    #[test]
    fn price_diff_negativechg_returns_negativechg() {
        assert_eq!(price_diff(&[10.0, 9.0, 8.0]), Some((-0.2, -2.0)))
    }
}