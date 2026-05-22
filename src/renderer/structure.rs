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

pub fn fmt_tick_pos(tick_pos: f64) -> String {
    // scientific notation to 4 sigfigs
    if tick_pos.abs() < 1e-3 || tick_pos.abs() >= 1e6 {
        let scientific_raw = format!("{:.3e}", tick_pos);
        let parts = scientific_raw.split('e').collect::<Vec<&str>>();
        assert!(parts.len() == 2);

        if parts[0].contains('.') {
            parts[0]
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
                + "e"
                + parts[1]
        } else {
            parts[0].to_string() + "e" + parts[1]
        }
    } else {
        let digits_after_decimal = ((-tick_pos.abs().log10().floor()) as i32 + 3).max(0);
        let result = format!(
            "{:.precision$}",
            tick_pos,
            precision = digits_after_decimal as usize
        );
        if result.contains('.') {
            result
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        } else {
            result
        }
    }
}
