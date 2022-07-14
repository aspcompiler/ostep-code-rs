use std::time::SystemTime;

pub fn spin(howlong: u64) {
    let now = SystemTime::now();
    while now.elapsed().unwrap().as_secs() < howlong {
        // spin
    }
}
