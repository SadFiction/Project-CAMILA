use std::{error::Error, net::SocketAddr};

use tonic::transport::Server;
use CAMILAlib::*;
use CAMILAlib::{camila_command_service_mod, camila_get_response_service_mod};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    
    let addr: SocketAddr = "[::1]:50051".parse()?;
    let service_1: camila_command_service_mod::CamilaCommandService = camila_command_service_mod::CamilaCommandService::default();
    let service_2: camila_get_response_service_mod::CamilaGetResponseService = camila_get_response_service_mod::CamilaGetResponseService::default();

    let descriptor = tonic_reflection::server::Builder::configure()
                                .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET).build()?;

    Server::builder()
        .add_service(CamilaCommandsServer::new(service_1))
        .add_service(CamilaGetResponseServer::new(service_2))
        .add_service(descriptor)
        .serve(addr)
        .await?;
    Ok(())

}
