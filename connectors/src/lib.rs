use models::EndpointDefinition;

pub mod sftp;
pub trait Connector {
    fn connect(&mut self, endpoint: &EndpointDefinition);
    fn open(&mut self, endpoint: &EndpointDefinition);
}

pub trait Reader {
    fn read(&mut self, buffer: &mut Vec<u8>) -> std::io::Result<usize>;
}
pub trait Writer {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize>;
}
