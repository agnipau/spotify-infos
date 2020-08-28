pub fn milliseconds_to_time(duration: f64) -> String {
    let milliseconds = ((duration % 1000.0) / 100.0).floor();
    let seconds = ((duration / 1000.0) % 60.0).floor();
    let minutes = ((duration / (1000.0 * 60.0)) % 60.0).floor();
    let hours = ((duration / (1000.0 * 60.0 * 60.0)) % 24.0).floor();

    format!(
        "{:0>2}:{:0>2}:{:0>2}:{:0>2}",
        hours, minutes, seconds, milliseconds
    )
}
