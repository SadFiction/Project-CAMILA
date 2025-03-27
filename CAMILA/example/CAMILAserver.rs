use tonic::transport::Server;
use CAMILAlib;

#[tokio::main]
async fn main(){
    let addr = "[::1]:50051".parse().unwrap();

    let service = CAMILAlib::MessageService::default();
    
    let serv = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(CAMILAlib::proto::FILE_DESCRIPTOR_SET).build().unwrap();

    Server::builder()
    .add_service(serv)
    .add_service(CAMILAlib::MessageServer::new(service))
    .serve(addr)
    .await.unwrap();

}