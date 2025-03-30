pub mod proto{
    tonic::include_proto!("camila");
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("CAMILA_descriptor");
}

pub use std::pin::Pin;

pub use proto::camila_commands_server::{CamilaCommands, CamilaCommandsServer};
pub use proto::camila_get_response_server::{CamilaGetResponse, CamilaGetResponseServer};


pub use proto::{camila_command, camila_response, CamilaCommand};
pub use tonic::{Streaming, Request, Response};
pub use tokio_stream::Stream;





type RpcResult<T> = Result<Response<T>, tonic::Status>;

type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, tonic::Status>> + Send>>;
type RequestStream<T> = tonic::Request<tonic::Streaming<T>>;


pub mod camila_command_service_mod;
pub mod camila_get_response_service_mod;





/* 
pub use proto::{message_server::{Message, MessageServer}, Status};

pub use tonic::transport::Server;
pub mod  proto{
    tonic::include_proto!("camilaproto");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("CAMILA_descriptor");
}

#[derive(Debug, Default)]
pub struct MessageService{

}

#[tonic::async_trait]
impl  Message for MessageService{
    async fn send_message(
        &self,
        request: tonic::Request<proto::Text>
    ) -> Result<tonic::Response<proto::Stat>, tonic::Status> {
        let input = request.get_ref();
        
        println!("Got a message!! The message is: {:?}", input.text);

        let response = proto::Stat{
            status: proto::Status::Received as i32
        };

        

        Ok(tonic::Response::new(response))

    }
}

*/