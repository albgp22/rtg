use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Http,
    Https,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Http => "http",
                Self::Https => "https",
            }
        )
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum HttpVersion {
    V1_0,
    V1_1,
    V2_0,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]

pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Connect,
    Trace,
    Patch,
}

impl HttpMethod {
    pub fn to_method(&self) -> http::Method {
        match self {
            Self::Get => http::Method::GET,
            Self::Post => http::Method::POST,
            Self::Put => http::Method::PUT,
            Self::Delete => http::Method::DELETE,
            Self::Head => http::Method::HEAD,
            Self::Options => http::Method::OPTIONS,
            Self::Connect => http::Method::CONNECT,
            Self::Trace => http::Method::TRACE,
            Self::Patch => http::Method::PATCH,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TrafficScenarioConfig {
    pub name: String,
    pub rate: u64,
    pub description: String,
    pub author: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TrafficScenario {
    pub config: TrafficScenarioConfig,
    pub servers: Vec<Server>,
    pub requests: Vec<Request>,
    pub responses: Vec<Response>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Server {
    pub id: u32,
    pub protocol: Protocol,
    pub host: String,
    pub port: u32,
    pub authorization: bool,
    pub http_version: HttpVersion,
    pub authz_token: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Request {
    pub id: u32,
    pub server_id: u32,
    pub path: String,
    pub method: HttpMethod,
    pub content: RequestContent,
    pub depends: Vec<u32>,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RequestContent {
    pub headers: HashMap<String, String>,
    pub body: Value,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ResponseContent {
    pub headers: HashMap<String, String>,
    pub body: Value,
    pub status: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Response {
    pub id: u32,
    pub request_id: u32,
    pub expected: ResponseContent,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RuntimeStatistics {}

#[derive(Error, Debug)]
pub enum TrafficScenarioError {
    #[error("Traffic scenario json file parse error")]
    ParseError(#[from] serde_json::Error),
    #[error("Traffic scenario file could not be opened/read")]
    ReadFileError(#[from] io::Error),
}

impl TrafficScenario {
    pub fn from_file(file: &str) -> Result<TrafficScenario, TrafficScenarioError> {
        let mut file = File::open(file)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let ts = serde_json::from_str(&data)?;

        Ok(ts)
    }

    pub fn add_server(&mut self, server: Server) {
        self.servers.push(server);
    }

    pub fn add_request(&mut self, request: Request) {
        self.requests.push(request);
    }

    pub fn add_response(&mut self, response: Response) {
        self.responses.push(response);
    }

    pub fn build_dependency_graph(&self) -> (Vec<u32>, HashMap<u32, Vec<u32>>) {
        let ids = self.requests.iter().map(|r| r.id).collect::<Vec<u32>>();
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        for id in ids.iter() {
            graph.insert(*id, self.get_request_by_id(*id).depends.clone());
        }

        (ids, graph)
    }

    pub fn get_response_by_request_id(&self, request_id: u32) -> Option<&Response> {
        self.responses.iter().find(|r| r.request_id == request_id)
    }

    pub fn get_request_by_id(&self, request_id: u32) -> &Request {
        self.requests
            .iter()
            .find(|r| r.id == request_id)
            .expect(&format!("Request {} not found", request_id))
    }

    pub fn get_server_by_id(&self, server_id: u32) -> &Server {
        self.servers
            .iter()
            .find(|s| s.id == server_id)
            .expect(&format!("Server {} not found", server_id))
    }
}
