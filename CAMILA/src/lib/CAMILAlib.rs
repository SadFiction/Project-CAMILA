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
