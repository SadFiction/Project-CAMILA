#![allow(non_snake_case)]
pub mod proto{
    tonic::include_proto!("camila");
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("CAMILA_descriptor");
}

use tokio::sync::mpsc;


use tokio_stream::{wrappers::ReceiverStream, StreamExt};



pub use std::pin::Pin;
use tokio::sync::Mutex;
use std::sync::Arc;
use once_cell::sync::Lazy;
pub use proto::camila_commands_server::{CamilaCommands, CamilaCommandsServer};
pub use proto::camila_get_response_server::{CamilaGetResponse, CamilaGetResponseServer};
pub use proto::{camila_command, camila_response, CamilaCommand, Cob, cob};
pub use tonic::{Streaming, Request, Response};
pub use tokio_stream::Stream;




type RpcResult<T> = Result<Response<T>, tonic::Status>;

type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, tonic::Status>> + Send>>;
type RequestStream<T> = tonic::Request<tonic::Streaming<T>>;


pub mod camila_command_service_mod;
pub mod camila_get_response_service_mod;
pub mod camila_grid;

const TEMP_QUEUE_SIZE: usize = 100;

pub static CYCLE_COMMAND_QUEUE: Lazy<Arc<Mutex<Queue<camila_command::Command>>>> = Lazy::new(|| Arc::new(Mutex::new(Queue::new(TEMP_QUEUE_SIZE))));
pub static TICK_COMMAND_QUEUE: Lazy<Arc<Mutex<Queue<camila_command::Command>>>> = Lazy::new(|| Arc::new(Mutex::new(Queue::new(TEMP_QUEUE_SIZE))));
pub static INDEPENDENT_COMMAND_QUEUE: Lazy<Arc<Mutex<Queue<camila_command::Command>>>> = Lazy::new(|| Arc::new(Mutex::new(Queue::new(TEMP_QUEUE_SIZE))));

pub static LOG_QUEUE: Lazy<Arc<Mutex<Queue<camila_response::Response>>>> = Lazy::new(|| Arc::new(Mutex::new(Queue::new(TEMP_QUEUE_SIZE))));

pub static CAMILA_GRID: Lazy<Arc<Mutex<camila_grid::CamilaObject>>> = Lazy::new(|| Arc::new(Mutex::new(camila_grid::CamilaObject::new(500, 500))));

pub trait QueueTypes{}
impl QueueTypes for camila_command::Command{}
impl QueueTypes for camila_response::Response{}

#[derive()]
pub struct Queue<T: QueueTypes>{
    queue_size: usize,
    max_size: usize,
    queue: Vec<T>
}

impl<T:QueueTypes> Queue<T>{
    pub fn new(max_size: usize) -> Self{
        Queue {
             queue_size: 0,
             max_size: max_size,
             queue: Vec::with_capacity(max_size)}
    }

    pub fn is_empty(&self) -> bool{
        self.queue_size == 0
    }

    pub async fn push_queue(&mut self, command: T) -> Result<(), &str>{
        
        if self.max_size < self.queue_size{
            
            return Err("FILLED");
        }
        else if self.max_size == self.queue_size{
    
            return Err("FULL");

        }
        else {
            self.queue.push(command);
            self.queue_size += 1;
            return Ok(());
        }
        
    }

    pub async  fn pop_queue(&mut self) -> Result<T, &str>{
        if self.queue_size == 0{
            return Err("QUEUE_EMPTY");
        }
        else {
            self.queue_size -= 1;
            return Ok(self.queue.pop().unwrap()) ;
        }

    }

}

impl Queue<camila_response::Response>{
  

    async fn getItemsByType(&mut self, tx: mpsc::Sender<Result<camila_response::Response,tonic::Status>>, find: &[i32]) -> Result<(), &str>{
         let queue_clone = self.queue.clone();

        for (i, item) in queue_clone.iter().enumerate(){
            match item.clone().success.unwrap() {
                camila_response::response::Success::Output(content) => {
                        for comparison in find.iter(){
                            let tx_clone = tx.clone();
                            
                            
                            if (*comparison).clone() == content.r#type as i32{
                                let item_clone = self.queue.remove(i);
                                tokio::spawn(
                                    async move {tx_clone.send(Ok(item_clone)).await.expect("tx message")
                                    });
                                }
                            }
                            
                    }
                    
                _ => {}
            }
         }
         
    Ok(())
        
    }   
    async fn getItemsByID(&mut self, tx: mpsc::Sender<Result<camila_response::Response,tonic::Status>>, find: &[u64]) -> Result<(), &str>{
        let queue_clone = self.queue.clone();
        for (i, item) in queue_clone.iter().enumerate(){
            match item.clone().success.unwrap() {
                camila_response::response::Success::Output(content) => {
                        for comparison in find.iter(){
                            let tx_clone = tx.clone();
                            
                            
                            if (*comparison).clone() == content.responds_to_id.unwrap(){
                                let item_clone = self.queue.remove(i);
                                tokio::spawn(
                                    async move {tx_clone.send(Ok(item_clone)).await.expect("tx message")
                                    });
                                }
                            }
                        

                    }
                    
                        
                _ => {}
            }
         }
         
    Ok(())
        
    }   
    async fn getItemsByTime(&mut self, tx: mpsc::Sender<Result<camila_response::Response,tonic::Status>>, start_time: u64, end_time: u64) -> Result<(), &str>{
        let queue_clone = self.queue.clone();
        for (i, item) in queue_clone.iter().enumerate(){
            match item.clone().success.unwrap() {
                camila_response::response::Success::Output(content) => {
                        
                        let tx_clone = tx.clone();
                        
                            
                        let mut time_start = start_time;
                        let mut time_end = end_time;
                        if end_time > start_time {
                            time_start = end_time;
                            time_end = start_time;
                        }
                        if content.time_milli >= time_start && content.time_milli <= time_end{
                            let item_clone = self.queue.remove(i);
                            tokio::spawn(
                                async move {tx_clone.send(Ok(item_clone)).await.expect("tx message")
                                });
                        }
                            
                    }
                    
                        
                _ => {}
            }
         }
         
    Ok(())
        
    }   

    

}






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