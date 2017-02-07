//engine.rs

use std::thread;
use std::time;
use std::collections::HashMap;

use uuid::Uuid;

use mud_request::{Request, RequestType};
use mud_response::response::{self, Response};
use remote_client::RemoteClient;

const ENGINE_TICK_RATE: u32 = 10; //10Hz, ergo 1000 / 10 = 100ms delay per engine tick
const MAX_CLIENT_COUNT: usize = 8;

pub struct Engine {
    session_token: String,
    responses: Vec<Response>,
    delay_duration_ms: u64,
    clients: HashMap<Uuid, RemoteClient>,
    debug_mode: bool,
}

impl Engine {
    pub fn new(token: String, debug: bool) -> Engine {
        Engine {
            session_token: token,
            responses: Vec::new(),
            delay_duration_ms: (1000 / ENGINE_TICK_RATE) as u64,
            clients: HashMap::new(),
            debug_mode: debug,
        }
    }

    pub fn process_request(&mut self, req: &Request) {
        match req.req_type {
            RequestType::Connect => {
                if self.clients.len() < MAX_CLIENT_COUNT {
                    if let Ok(name) = String::from_utf8(req.contents.clone()) {
                        let uid = Uuid::new_v4();
                        let new_client = RemoteClient {
                            user_id: uid,
                            addr: req.addr,
                            user_name: name.clone(),
                        };
                        if self.debug_mode {
                            let client = new_client.clone();
                            println!("Client connecting: {} (id: {})", client.user_name, client.user_id);
                        }
                        self.clients.insert(new_client.user_id, new_client);
                        let welcome = format!("Connection successful.  Welcome, {}.", name);
                        let res = Engine::create_connection_success_response(welcome, uid, req);
                        self.responses.push(res);
                    } else {
                        let res = Engine::create_connection_reject_response("Invalid name.", req);
                        self.responses.push(res);
                    }
                } else {
                    let res = Engine::create_connection_reject_response("Maximum connection count reached.",
                                                                        req);
                    self.responses.push(res);
                }
            },
            RequestType::Disconnect => {
                if self.clients.contains_key(&req.client_id) {
                    if self.debug_mode {
                        let client = self.clients[&req.client_id].clone();
                        println!("Client disconnecting: {} (id: {})", client.user_name, client.user_id);
                    }
                    self.clients.remove(&req.client_id);
                    let res = Engine::create_dc_acknowledge_response("Disconnected.",
                                                                     req);
                    self.responses.push(res);
                } else {
                    //someone tried to disconnect without connecting first? ignore...
                }
            },
            _ => {
                let res = Engine::create_token_response(self.session_token.clone(), req);
                self.responses.push(res);
            },
        }
    }

    pub fn get_responses(&self) -> Vec<Response> {
        self.responses.clone()
    }

    pub fn perform_tick(&mut self) {
        //process incoming requests
        //update game state
        thread::sleep(time::Duration::from_millis(self.delay_duration_ms));
        self.responses.clear();
    }

    //---request-processing functions

    fn create_token_response(token: String, req: &Request) -> Response {
        Response {
            code: response::CODE_TOKEN_ACKNOWLEDGE,
            addr: req.addr,
            numerical_data: Vec::default(),
            textual_data: token,
            object_data: Vec::default(),
        }
    }

    fn create_connection_success_response(msg: String, user_id: Uuid, req: &Request) -> Response {
        //send a response containing a success code, message, and new user ID
        Response {
            code: response::CODE_CONNECT_ACCEPT,
            addr: req.addr,
            numerical_data: Vec::default(),
            textual_data: msg.to_string(),
            object_data: user_id.as_bytes().to_vec(),
        }
    }

    fn create_dc_acknowledge_response(msg: &'static str, req: &Request) -> Response {
        Response {
            code: response::CODE_DISCONNECT_ACK,
            addr: req.addr,
            numerical_data: Vec::default(),
            textual_data: msg.to_string(),
            object_data: Vec::default(),
        }
    }

    fn create_connection_reject_response(msg: &'static str, req: &Request) -> Response {
        Response {
            code: response::CODE_CONNECT_REJECT,
            addr: req.addr,
            numerical_data: Vec::default(),
            textual_data: msg.to_string(),
            object_data: Vec::default(),
        }
    }

    //----------------------------end
}
