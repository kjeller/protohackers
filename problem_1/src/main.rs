use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    result::Result,
    error::Error,
};

const BIND_ADDR: &str = "0.0.0.0:48879";
const DEF_ERROR_RESP: &str = "{}";

#[derive(PartialEq)]
enum Method {
    IsPrime(i32),
}

impl Method {
    fn is_prime(n: i32) -> bool {
        if n <= 1 {
            return false;
        }
            
        for a in 2..n {
            if n % a == 0 {
                return false;
            }
        }
        true
    }

    fn get_string_constant(&self) -> String {
        match self {
            Method::IsPrime(_) => return "prime".to_string(),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Method::IsPrime(x) => return format!("IsPrime: {}", x),
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

    fn process(&mut self, method: Method) {
        self.method = method.get_string_constant();
        match method {
            Method::IsPrime(x) => self.result = Method::is_prime(x),
        }
    }

    fn to_string(&self) -> String {
        return format!("{{\"method\":\"isPrime\",\"prime\":{}}}", self.result);
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
    
        if let Some(x) = data["number"].as_i32() {
            return Ok(Method::IsPrime(x))
        } 
    }

    Err(ProtocolMalformed)
}

fn handle_response(method: Method) -> String {
    let mut result = Message::new();

    result.process(method);
    println!("{}", result.to_string());
    result.to_string()
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();
        let method = parse_json(&request_line);
    
        println!("Got request {}", request_line);
        
        match method {
            Ok(m) => {
                let response = handle_response(m);
                stream.write_all({
                    println!("Valid json: Sending response {}", &response);
                    response.as_bytes()
                }).unwrap();
                stream.flush().unwrap();
            },
            Err(_) => {
                println!("Malformed json: responding with err");
                stream.write_all({
                    DEF_ERROR_RESP.as_bytes()
                }).unwrap();
                stream.flush().unwrap();
                break;
            },
        }
    }
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
            "{}".to_string(),
        ];

        let expected = vec![
            Ok(Method::IsPrime(123)),
            Ok(Method::IsPrime(1)),
            Err(ProtocolMalformed),
        ];

        for (i, r) in requests.iter().enumerate() {
            let mut method = parse_json(r.into());
            assert!(method == expected[i]);
        }
    }

    #[test]
    fn test_json_req_resp() {
        let requests = vec![
            "{\"method\":\"isPrime\",\"number\":123}".to_string(),
            "{\"method\":\"isPrime\",\"number\":1}".to_string(),
            "{\"method\":\"isPrime\",\"number\":\"notnumber\"}".to_string(),
            "{\"method\":\"isprime\",\"number\":1}".to_string(),
        ];

        let exp_response = vec![
            "{\"method\":\"isPrime\",\"prime\":false}".to_string(),
            "{\"method\":\"isPrime\",\"prime\":true}".to_string(),
            "{}".to_string(),
            "{}".to_string(),
        ];

        for (i, r) in requests.iter().enumerate() {
            let mut method = parse_json(r.into());
            assert!(method == expected[i]);
        }
    }

    #[test]
    fn test_is_prime() {
        let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let exp_prime: Vec<bool> = vec![false, true, true, false, true, false, true, false];

        for (i, num) in numbers.iter().enumerate() {
            assert!(Method::is_prime(*num) == exp_prime[i]);
        }
    }

}
