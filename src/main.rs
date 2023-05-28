mod traffic_scenario;
use traffic_scenario::TrafficScenario;
mod executor;
use executor::Executor;

fn main() {
    let ts = TrafficScenario::from_file("examples/single_request_response.json").unwrap();
    println!("{}", serde_json::to_string_pretty(&ts).unwrap());
    Executor::run_traffic_scenario(&ts);
}
