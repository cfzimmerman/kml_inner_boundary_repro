use kml::KmlReader;

// parsed_kml.txt generated with `cargo run > parsed_kml.txt`
fn main() {
    let mut kml_reader = KmlReader::<_, f64>::from_path("./inner_boundaries.kml").unwrap();
    let kml = kml_reader.read().unwrap();
    println!("{:#?}", kml);
}
