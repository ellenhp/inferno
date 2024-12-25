pub static EARTH_RADIUS_APPROX: f64 = 6_371_000f64;

pub fn lat_lng_to_cartesian(lat_lng: (f64, f64)) -> [f64; 3] {
    let lat = lat_lng.0;
    let lng = lat_lng.1;
    if !lat.is_finite() || !lng.is_finite() {
        return [0.0; 3];
    }
    let lat = lat.to_radians();
    let lng = lng.to_radians();
    [
        EARTH_RADIUS_APPROX * lat.cos() * lng.sin(),
        EARTH_RADIUS_APPROX * lat.cos() * lng.cos(),
        EARTH_RADIUS_APPROX * lat.sin(),
    ]
}

pub fn cartesian_to_lat_lng(coords: [f64; 3]) -> (f64, f64) {
    let lng = f64::atan2(coords[0], coords[1]);
    let lat = (coords[2] / EARTH_RADIUS_APPROX).asin();
    let lat = lat.to_degrees();
    let lng = lng.to_degrees();
    (lat, lng)
}
