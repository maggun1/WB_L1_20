struct LegacyGeoService;

impl LegacyGeoService {
    pub fn get_coordinates(&self) -> (f64, f64) {
        (57.583305, 37.294287)
    }
}
struct Location {
    latitude: f64,
    longitude: f64,
}

trait GeoService {
    fn get_location(&self) -> Location;
}

struct GeoServiceAdapter {
    legacy_service: LegacyGeoService,
}

impl GeoServiceAdapter {
    pub fn new(legacy_service: LegacyGeoService) -> Self {
        GeoServiceAdapter { legacy_service }
    }
}

impl GeoService for GeoServiceAdapter {
    fn get_location(&self) -> Location {
        let (latitude, longitude) = self.legacy_service.get_coordinates();
        Location { latitude, longitude }
    }
}

fn main() {
    let legacy_service = LegacyGeoService;
    let adapter = GeoServiceAdapter::new(legacy_service);

    let location = adapter.get_location();
    println!(
        "Coordinates: latitude = {}, longitude = {}",
        location.latitude, location.longitude
    );
}
