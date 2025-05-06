
use tokio::sync::mpsc;
use std::time::Duration;
use std::net::SocketAddr;

use tokio::time::sleep;
use tonic::transport::Server;
use CAMILAlib::*;
use CAMILAlib::{camila_command_service_mod, camila_get_response_service_mod};










async fn tick_signal(tx: mpsc::Sender<bool>,tick_speed: u64) -> () {
    let tick_speed = if tick_speed < 1000 {1000-tick_speed} else {1};

    loop {
        
        sleep(Duration::from_millis(tick_speed)).await;
        tx.send(true).await.expect("tx message");
    }
}
  
async fn main_loop(mut rx: mpsc::Receiver<bool>) -> (){
    let mut tick_nr: u64 = 0;

    loop {
        
        if let Some(_status) = rx.recv().await {
            tick_nr += 1;
            println!("Tick: {}", tick_nr);
            CAMILAlib::CAMILA_GRID.lock().await.update().await;
            
        }
    }
}




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    
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
    
    let (tx, rx) : (mpsc::Sender<bool>, mpsc::Receiver<bool>) = mpsc::channel(0);

    tokio::join!(tick_signal(tx, 1000), main_loop(rx));

    Ok(())

}
