use std::{
    io::{prelude::*, BufReader, BufWriter},
    net::{TcpListener, TcpStream},
    thread,
    result::Result,
};

const BIND_ADDR: &str = "0.0.0.0:48879";
const DEF_ERROR_RESP: &str = "{}";

#[derive(PartialEq)]
enum Method {
    IsPrime(i64),
}

impl Method {
    fn get_string_constant(&self) -> String {
        match self {
            Method::IsPrime(_) => return "prime".to_string(),
        }
    }
}

struct Message {
    method: String,
    result: bool,
}

impl Message {
    fn new() -> Message {
        Message {
            method: "".to_string(),
            result: false,
        }
    }

    fn process(&mut self, method: &Method) {
        self.method = method.get_string_constant();
        match method {
            Method::IsPrime(x) => {
                if *x < 0 {
                    self.result = false;
                } else {
                    self.result = primes::is_prime(*x as u64);
                }
            }
        }
    }

    fn to_string(&self) -> String {
        return format!("{{\"method\":\"isPrime\",\"prime\":{}}}\n", self.result);
    }
}

#[derive(PartialEq)]
struct ProtocolMalformed;

fn parse_json(json: &str) -> Result<Method, ProtocolMalformed> {
    let parse = json::parse(&json);

    if let Ok(data) = parse {
        if data["method"].is_null() || data["number"].is_null() {
            return Err(ProtocolMalformed);
        }
    
        if let Some(x) = data["method"].as_str() {
            match x {
                "isPrime" => {}, // allowed
                _ => return Err(ProtocolMalformed),
            }
        }
    
        if let Some(x) = data["number"].as_f64() {
            return Ok(Method::IsPrime(x as i64))
        } 
    }

    Err(ProtocolMalformed)
}

fn handle_response(method: &Method) -> String {
    let mut result = Message::new();

    result.process(&method);
    result.to_string()
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(stream.try_clone().unwrap());
    let mut buf_writer = BufWriter::new(&mut stream);

    loop {
        
        let mut request_line = String::new();
        let num_bytes = buf_reader.read_line(&mut request_line).unwrap();
        let method = parse_json(&request_line);
    
        println!("Read {} bytes: {} ", num_bytes, request_line);

        if num_bytes == 0 {
            break;
        }
        
        match method {
            Ok(m) => {
                let response = handle_response(&m);
                buf_writer.write_all({
                    println!("Valid json: Sending response {}", &response);
                    response.as_bytes()
                }).unwrap();
                buf_writer.flush().unwrap();
            },
            Err(_) => {
                println!("Malformed json '{}': responding with err", request_line);
                buf_writer.write_all({
                    DEF_ERROR_RESP.as_bytes()
                }).unwrap();
                buf_writer.flush().unwrap();
                break;
            },
        }
    }
    println!("Disconnect client!");
}

fn main() {
    let listener = TcpListener::bind(BIND_ADDR).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        println!("Connection established!");
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json() {
        let requests = vec![
            "{\"method\":\"isPrime\",\"number\":123}".to_string(),
            "{\"method\":\"isPrime\",\"number\":1}".to_string(),
            "{\"method\":\"isPrime\",\"number\":7119040.0}".to_string(),
            "{\"method\":\"isPrime\",\"number\":7119040.123}".to_string(),
            "{}".to_string(),
        ];

        let expected = vec![
            Ok(Method::IsPrime(123)),
            Ok(Method::IsPrime(1)),
            Ok(Method::IsPrime(7119040)),
            Ok(Method::IsPrime(7119040)),
            Err(ProtocolMalformed),
        ];

        for (i, r) in requests.iter().enumerate() {
            let method = parse_json(r);
            assert!(method == expected[i]);
        }
    }

    #[test]
    fn test_handle_response() {
        let requests = vec![
            Method::IsPrime(123),
            Method::IsPrime(1),
        ];

        let exp_response = vec![
            "{\"method\":\"isPrime\",\"prime\":false}\n".to_string(),
            "{\"method\":\"isPrime\",\"prime\":false}\n".to_string(),
        ];

        for (i, r) in requests.iter().enumerate() {
            assert!(handle_response(r) == exp_response[i]);
        }
    }
}
