use models::EndpointDefinition;
use models::SftpConnection;
use ssh2::Session;
use std::fs::File;
use std::io::Error;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::ops::RangeFull;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use crate::Reader;
use crate::Writer;

use crate::Connector;

// fn stream_file_from_sftp_to_sftps(sftp_src: &ssh2::Sftp, sftp_dests: &[ssh2::Sftp]) {
//     let mut remote_file_src = sftp_src.open(remote_src_path).unwrap();
//     let mut buffers = vec![vec![0; 4096]; sftp_dests.len()]; // Buffer for each destination SFTP

//     loop {
//         let bytes_read = remote_file_src.read(&mut buffers[0]).unwrap();
//         if bytes_read == 0 {
//             break; // End of file
//         }

//         for (i, sftp_dest) in sftp_dests.iter().enumerate() {
//             let mut remote_file_dest = sftp_dest.create(remote_dest_path[i]).unwrap();
//             remote_file_dest
//                 .write_all(&buffers[i][..bytes_read])
//                 .unwrap();
//         }
//     }
// }

// impl Connector for SftpConnection {
//     type current_connection: ssh2::Sftp;

//     fn connect(&self) -> Result<String, Error> {
//         let mut sess = Session::new().unwrap();
//         let tcp = TcpStream::connect((self.hostname.as_str(), self.port)).unwrap();

//         sess.set_tcp_stream(tcp);
//         sess.handshake().unwrap();
//         let pwd = self.user_password.as_ref().unwrap();
//         sess.userauth_password(&self.user_name, pwd.as_str())
//             .unwrap();

//         sess.set_timeout(self.connection_timeout_in_seconds * 1000);
//         current_connection = sess.sftp().unwrap();
//     }
// }

pub struct SftpConnector {
    pub session: Option<ssh2::Sftp>,
    pub file: Option<ssh2::File>,
}

impl Connector for SftpConnector {
    fn connect(&mut self, endpoint: &EndpointDefinition) {
        let connection_attributes = &endpoint.conenction;
        if let models::ConnectionType::Sftp(attrib) = connection_attributes {
            let mut sess = Session::new().unwrap();
            let tcp = TcpStream::connect((attrib.hostname.as_str(), attrib.port)).unwrap();
            sess.set_tcp_stream(tcp);
            sess.handshake().unwrap();
            let pwd = attrib.user_password.as_ref().unwrap();
            sess.userauth_password(&attrib.user_name, pwd.as_str())
                .unwrap();
            sess.set_timeout(attrib.connection_timeout_in_seconds * 1000);
            let the_sess = sess.sftp();
            self.session = Some(the_sess.unwrap()); ;
        }
    }

    fn open(&mut self, endpoint: &EndpointDefinition) {
        let connection_attributes = &endpoint.conenction;
        match &connection_attributes {
            models::ConnectionType::Sftp(attrib) => match (&mut self.session) {
                Some(sess) => match (&attrib.path, &attrib.filename) {
                    (Some(p), Some(f)) => {
                        let mut full_path = PathBuf::new();
                        full_path.push(p);
                        full_path.push(f);
                        let openned_file = sess.open(full_path.as_path());
                        self.file = Some(openned_file.unwrap());
                    }
                    (None, Some(f)) => {
                        self.file = Some(sess.open(Path::new(&f)).unwrap());
                    }
                    (None, None) => panic!("Filename is mandatory"),
                    (Some(_), None) => panic!("Filename is mandatory"),
                },
                _ => panic!("No Session"),
            },
            _ => (),
        }
    }
}

impl Reader for SftpConnector {
    fn read(&mut self, buffer: &mut Vec<u8>) -> std::io::Result<usize> {
        match &mut self.file {
            Some(current_file) => {
                println!("Stats of current File {:?}", current_file.stat()?);
                let file_read: Result<usize, Error> = current_file.read(buffer);
                match file_read {
                  Ok(f) => Ok(f),
                  Err(e) => {
                    panic!("Error reading file. {}", e.to_string());
                  }
                }
            }
            None => panic!("File not initilized"),
        }
    }
}

impl Writer for SftpConnector {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        Ok((0))
    }
}
