use jsonrpc_http_server::jsonrpc_core::{IoHandler, Value, Params};
use jsonrpc_http_server::ServerBuilder;
use std::collections::HashMap;

fn main() {
	let mut io = IoHandler::default();
	io.add_method("addition", |_params: Params| async {
        let map: HashMap<String, Value> = _params.parse().unwrap();
        match map.get::<str>(&"a") {
            Some(a) => {
                match map.get::<str>(&"b") {
                    Some(b) => {
                        let added = a.as_i64().expect("error converting a to i64") + b.as_i64().expect("error converting b to i64");
                        let ret = String::from(added.to_string());
                        Ok(Value::String(ret.to_owned()))
                    },
                    _ => Ok(Value::String("couldn't find it 'a' key".to_owned())),
                }
            },
            _ =>  Ok(Value::String("couldn't find it 'b' key".to_owned())),
        }
	});

	let server = ServerBuilder::new(io)
		.threads(1)
		.start_http(&"0.0.0.0:9999".parse().unwrap())
		.unwrap();

	server.wait();
}