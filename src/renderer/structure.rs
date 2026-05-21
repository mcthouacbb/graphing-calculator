pub fn tick_width(viewport_size: f64) -> f64 {
    let i = viewport_size.log10().floor() - 1.0;
    let j = (viewport_size / 2.0).log10().floor() - 1.0;
    let k = (viewport_size / 5.0).log10().floor() - 1.0;
    let candidates = [
        10.0f64.powf(i),
        10.0f64.powf(i + 1.0),
        2.0 * 10.0f64.powf(j),
        2.0 * 10.0f64.powf(j + 1.0),
        5.0 * 10.0f64.powf(k),
        5.0 * 10.0f64.powf(k + 1.0),
    ];

    let mut best_candidate = 0.0;
    let mut best_error = 10000.0;

    for candidate in candidates {
        let error = ((viewport_size / candidate).log10() - 0.9).abs();
        if error < best_error {
            best_candidate = candidate;
            best_error = error;
        }
    }
    best_candidate
}
