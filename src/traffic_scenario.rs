use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::{self, Read};
use thiserror::Error;
use url::Url;

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Http,
    Https,
}

impl ToString for Protocol {
    fn to_string(&self) -> String {
        match self {
            Self::Http => "http",
            Self::Https => "https",
        }
        .to_string()
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
pub struct TrafficScenario {
    servers: Vec<Server>,
    requests: Vec<Request>,
    responses: Vec<Response>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Server {
    id: u32,
    protocol: Protocol,
    host: String,
    port: u32,
    authorization: bool,
    http_version: HttpVersion,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Request {
    id: u32,
    server_id: u32,
    path: String,
    method: HttpMethod,
    content: RequestContent,
    depends: Vec<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RequestContent {
    headers: HashMap<String, String>,
    body: Value,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ResponseContent {
    headers: HashMap<String, String>,
    body: Value,
    status: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Header {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Response {
    id: u32,
    request_id: u32,
    expected: ResponseContent,
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

    pub fn build_dependency_graph(&self) -> HashMap<u32, Vec<u32>> {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();

        graph
    }

    pub fn run(&self) -> Result<RuntimeStatistics, TrafficScenarioError> {
        let request = self.requests[0].clone();
        let server = self.servers[0].clone();
        let endpoint = format!(
            "{}://{}:{}{}",
            server.protocol.to_string(),
            server.host,
            server.port,
            request.path
        );
        let client = match server.http_version {
            HttpVersion::V1_0 | HttpVersion::V1_1 => reqwest::blocking::Client::builder()
                .http1_only()
                .build()
                .unwrap(),
            HttpVersion::V2_0 => reqwest::blocking::Client::builder()
                .http2_prior_knowledge()
                .build()
                .unwrap(),
        };
        client
            .request(request.method.to_method(), Url::parse(&endpoint).unwrap())
            .json(&request.content.body)
            .send()
            .unwrap();
        Ok(RuntimeStatistics {})
    }
}
