use ssh2::{Error, Session};
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
use std::fs::File;

pub struct Host {
    pub hostname: String,
    pub username: String,
    pub private_file: String,
}

pub struct Connection {
    sess: Session,
}

impl Connection {
    pub fn new(host: &Host) -> Result<Connection, Error> {
        let tcp = TcpStream::connect(host.hostname.as_str()).unwrap();
        let mut sess = Session::new().unwrap();

        let path = Path::new(host.private_file.as_str());

        sess.set_tcp_stream(tcp);
        sess.handshake()?;
        sess.userauth_pubkey_file(host.username.as_str(), None, path, None)?;
        return Ok(Connection { sess: sess });
    }

    pub fn execute(&self, command: &str, file_path: &Path) ->String {
    	let mut output = File::create(file_path).unwrap();
        let mut channel = self.sess.channel_session().unwrap();
        channel.exec(command).unwrap();

        let mut stream = channel.stream(0);
        let mut stderr_stream = channel.stderr();
       	std::io::copy(&mut stream, &mut output).unwrap();
        let mut stderr= String::new();
        stderr_stream.read_to_string(&mut stderr).unwrap();
        channel.wait_close().unwrap();
        println!("{}", channel.exit_status().unwrap());
        return stderr
    }

    pub fn close(&self) {
        self.sess.disconnect(None, "Disconnected", None).unwrap()
    }
}
