#![feature(default_type_params)]

use std::io;
use std::io::{TcpStream, BufferedStream, IoResult};
use std::io::net::ip::{SocketAddr, Ipv4Addr}; 
use std::io::net::addrinfo::get_host_addresses;

struct Host {
    // The name of the host that was requested
    name: String,
    // Port!
    port: u16,
}
// GET <url> HTTP/1.1
// Host: <name:port>\r\n

struct UrlPath {
    path: String,
    query: Vec<(String, String)>,
}

impl UrlPath {
    fn query_to_string(&self) -> String {
        let mut out = String::new();

        if self.query.len() == 0 {
            return out;
        }

        fn append_pair(s: &mut String, pair: &(String, String)) {
            let &(ref k, ref v) = pair;
            s.push_str(k.as_slice());
            s.push_char('=');
            s.push_str(v.as_slice());
        }

        let mut it = self.query.iter();
        append_pair(&mut out, it.next().unwrap());

        for pair in it {
            out.push_char('&');
            append_pair(&mut out, pair);
        }

        out
    }
}

// only writes HTTP 1.1 GET requests!
pub struct RequestWriter<S = TcpStream> {
    stream: Option<BufferedStream<S>>,

    /// The originating IP address of the request.
    remote_addr: SocketAddr,

    /// The host name and IP address that the request was sent to
    host: Host,

    /// The path being requested
    path: UrlPath,
}

impl<S = TcpStream> RequestWriter<S> {
    fn new(hostname: String, port: u16, path: UrlPath) -> IoResult<RequestWriter<S>> {
        let addrs = try!(get_host_addresses(hostname.as_slice()));
        let addr = addrs.move_iter().find(|&a| {
            match a {
                Ipv4Addr(..) => true,
                _ => false,
            }
        });

        let sa = SocketAddr { ip: addr.unwrap(), port: port };

        Ok(RequestWriter { stream: None, remote_addr: sa,
                        host: Host { name: hostname, port: port },
                        path: path })

    }
}


impl<S: Reader + Writer = TcpStream> RequestWriter<S> {
    /// Connect to the remote host; fails if already connected.
    /// Returns ``true`` upon success and ``false`` upon failure (also use conditions).
    pub fn connect(&mut self) -> IoResult<()> {
        if !self.stream.is_none() {
            fail!("Already connected");
        }

        let host = format!("{}", self.remote_addr.ip).as_slice();
        self.stream = TcpStream::connect(host, self.remote_addr.port);

        Ok(())
    }

    pub fn write_headers(&mut self) -> IoResult<()> {
        if self.stream.is_none() {
            try!(self.connect());
        }

        let ref path = self.path;

        try!(write!(self.stream.get_mut_ref() as &mut Writer,
            "GET {}{}{} HTTP/1.1\r\n",
            path.path.as_slice(),
            if path.query.len() > 0 { "?" } else { "" },
            path.query_to_string()));

        try!(write!(self.stream.get_mut_ref() as &mut Writer,
            "Host: {}:{}\r\n",
            self.host.name, self.host.port));

        Ok(())
    }
}


fn main() {
    let path = UrlPath { path: "/".to_string(), query: vec!() };
    let mut rw = RequestWriter::new("127.0.0.1".to_string(), 8012, path);
    /*
    let mut stream = TcpStream::connect("127.0.0.1", 8012);

    let req_str = "GET / HTTP/1.1";

    match stream.write(req_str.as_bytes()) {
        Err(e) => { 
            println!("Error: {}", e); 
        },
        Ok(_) => {
            let mut buf: [u8, ..3] = [0, 0, 0];

            match stream.read(buf) {
                Err(e) => { 
                    println!("Error: {}", e); 
                },
                Ok(_) => {},
            }
            
            println!("{} {} {}", buf[0], buf[1], buf[2]);
        },
    }
    */
}

