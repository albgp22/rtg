mod traffic_scenario;
use traffic_scenario::TrafficScenario;

fn main() {
    let ts = TrafficScenario::from_file("examples/single_request_response.json").unwrap();
    println!("{}", serde_json::to_string_pretty(&ts).unwrap());
    ts.run().unwrap();
}
