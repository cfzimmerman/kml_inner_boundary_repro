use kml::KmlReader as CurrKmlReader;
use kml_proposed::KmlReader as ProposedKmlReader;

// Output files generated with:
// cargo run -- curr > curr_parser.txt
// cargo run -- proposed > proposed_parser.txt
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let version: &String = args.get(1).expect("Requires argument `curr` or `proposed`");

    match version.as_str() {
        "curr" => {
            let mut curr_kml_reader =
                CurrKmlReader::<_, f64>::from_path("./inner_boundaries.kml").unwrap();
            let kml_curr = curr_kml_reader.read().unwrap();
            println!("{:#?}", kml_curr);
        }
        "proposed" => {
            let mut proposed_kml_reader =
                ProposedKmlReader::<_, f64>::from_path("./inner_boundaries.kml").unwrap();
            let kml_proposed = proposed_kml_reader.read().unwrap();
            println!("{:#?}", kml_proposed);
        }
        _ => panic!("Argument must be either `curr` or `proposed`"),
    }
}
