//engine.rs

use mud_request::Request;
use mud_response::Response;
use std::thread;
use std::time;
use std::sync::mpsc::{self, Sender, Receiver};
use std::collections::VecDeque;

const ENGINE_TICK_RATE: u32 = 10; //10Hz, ergo 1000 / 10 = 100ms delay per engine tick
const MAX_OUTGOING_RESPONSES: usize = 128;

pub struct Engine {
    session_token: String,
    response_tx: Sender<Response>,
    response_rx: Receiver<Response>,
    request_tx: Sender<Request>,
    request_rx: Receiver<Request>,
    request_queue: VecDeque<Request>,
    response_queue: VecDeque<Response>,
    delay_duration_ms: u64,
}

impl Engine {
    pub fn new(token: String) -> Engine {
        let (res_tx, res_rx) = mpsc::channel();
        let (req_tx, req_rx) = mpsc::channel();
        Engine {
            session_token: token,
            response_tx: res_tx,
            response_rx: res_rx,
            request_tx: req_tx,
            request_rx: req_rx,
            request_queue: VecDeque::new(),
            response_queue: VecDeque::new(),
            delay_duration_ms: (1000 / ENGINE_TICK_RATE) as u64,
        }
    }

    pub fn process_request(&self, req: &Request) {
        let _ = self.request_tx.send(req.clone());
    }

    pub fn get_responses(&self) -> Vec<Response> {
        let mut responses: Vec<Response> = Vec::with_capacity(MAX_OUTGOING_RESPONSES);
        while let Ok(res) = self.response_rx.try_recv() {
            responses.push(res);
            if responses.len() >= MAX_OUTGOING_RESPONSES { break; } //don't let this get too big
        }
        responses
    }

    pub fn run(&self) {
        let res_tx = self.response_tx.clone();
        thread::sleep(time::Duration::from_millis(self.delay_duration_ms));
        //process incoming requests
        //update game state
        //send responses
    }
}
