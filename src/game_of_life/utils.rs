use std::io::Write;

use super::resources::Durations;

pub fn save_durations_to_file(durations: &Durations) {
    let mut file = std::fs::File::create("durations_ecs.txt").unwrap();
    for duration in durations.0.iter() {
        let duration_str = format!("{:?}", duration);
        file.write_all(duration_str.as_bytes())
            .expect("Unable to write data");
        file.write_all(b"\n").expect("Unable to write data");
    }
}
