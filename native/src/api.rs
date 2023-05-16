use std::collections::HashMap;
use std::sync::Mutex;
use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode, Error};
use ws::util::Token;


lazy_static! {
    static ref DATA_MAP: Mutex<HashMap<String, Sender>> = {
        let map: HashMap<String, Sender> = HashMap::new();
        Mutex::new(map)
    };
}

struct Client {
    sender: Sender,
    host: String,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        DATA_MAP.lock().unwrap().insert(self.host.to_owned(), self.sender.to_owned());
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("<receive> '{}'. ", msg);
        Ok(())
    }

    fn on_close(&mut self, _code: CloseCode, _reasonn: &str) {
        DATA_MAP.lock().unwrap().remove(&self.host);
    }

    fn on_timeout(&mut self, _event: Token) -> Result<()> {
        DATA_MAP.lock().unwrap().remove(&self.host);
        self.sender.shutdown().expect("shutdown error");
        Ok(())
    }

    fn on_error(&mut self, _err: Error) {
        DATA_MAP.lock().unwrap().remove(&self.host);
    }

    fn on_shutdown(&mut self) {
        DATA_MAP.lock().unwrap().remove(&self.host);
    }

}


pub fn websocket_connect(host: String) {
    if let Err(err) = connect(host.to_owned(), |out| {
        Client {
            sender: out,
            host: host.to_owned(),
        }
    }) {
        println!("Failed to create WebSocket due to: {:?}", err);
    }
}


pub fn send_message(host: String, message: String) {
    let binding = DATA_MAP.lock().unwrap();
    let sender = binding.get(&host.to_owned());
    
    match sender {
        Some(s) => {
            if s.send(message).is_err() {
                println!("Websocket couldn't queue an initial message.")
            };
        } ,
        None => println!("None")
    }
}


pub fn websocket_disconnect(host: String) {
    DATA_MAP.lock().unwrap().remove(&host.to_owned());
}