use std::collections::HashSet;

use reqwest::Client;

use crate::traffic_scenario::{Request, Response, Server, TrafficScenario};

pub struct Executor {}

impl Executor {
    pub fn run_traffic_scenario(ts: &TrafficScenario) {
        let executed: HashSet<u32> = HashSet::new();
        let (ids, deps) = ts.build_dependency_graph();
        let can_execute = |id: u32| {
            let mut can = true;
            for dep in deps.get(&id).unwrap() {
                if !executed.contains(dep) {
                    can = false;
                    break;
                }
            }
            can
        };
        for req in &ts.requests {
            let res = ts.get_response_by_request_id(req.id);
            let server = ts.get_server_by_id(req.server_id);
            Self::run_request(server, req, res);
        }
    }
    fn run_request(server: &Server, req: &Request, res: Option<&Response>) {
        let client = Client::new();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let url = format!("{}://{}:{}/{}", server.protocol, server.host, server.port, req.path);
        let mut http_req = client.request(req.method.to_method(), &url);
        for (k, v) in &req.content.headers {
            http_req = http_req.header(k, v);
        }
        let http_req = http_req.body(reqwest::Body::from(req.content.body.to_string()));
        let http_res = rt.block_on(http_req.send()).unwrap();
        if let Some(res) = res {
            assert_eq!(http_res.status().as_u16(), res.expected.status);
            for (k, v) in &res.expected.headers {
                assert_eq!(http_res.headers().get(k).unwrap(), v);
            }
            let body = rt.block_on(http_res.text()).unwrap();
            assert_eq!(body, res.expected.body.to_string());
        }
    }
}
