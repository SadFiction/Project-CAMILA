

use super::*;

#[derive(Debug, Default)]
pub struct CamilaGetResponseService;

#[tonic::async_trait]
impl CamilaGetResponse for CamilaGetResponseService{
    
    async fn pop_response_from_queue(&self, _: Request<camila_response::sorts_by::Empty>)
    -> RpcResult<camila_response::Response>{
        let mut log_queue = LOG_QUEUE.lock().await;
        let response = match log_queue.pop_queue().await {
            Ok(log) =>{log},
            Err(_) => {
                camila_response::Response{
                    success : Some(camila_response::response::Success::Status(camila_response::ResponseStatus::QueueEmpty as i32))
                }
            }
        };
        
        Ok(tonic::Response::new(response))
    }

    async fn all_response_queue(&self, _: Request<camila_response::sorts_by::Empty>)
    -> RpcResult<Self::AllResponseQueueStream>{
        let mut log_queue = LOG_QUEUE.lock().await;
        let (tx, rx) = mpsc::channel(256);

        tokio::spawn(
            async move{
                while  let Ok(_) = log_queue.pop_queue().await {
                    
                        let x = Self.pop_response_from_queue(tonic::Request::new(camila_response::sorts_by::Empty{})).await.unwrap().into_inner();

                            tx.send(Ok(x))
                            .await.expect("working rx");
                }
        
            }
        );
        let out_stream = ReceiverStream::new(rx);


        Ok(tonic::Response::new(Box::pin(out_stream) as Self::AllResponseQueueStream))
    }

    async fn responses_from_i_ds(&self, request: Request<camila_response::sorts_by::IDs>)
    -> RpcResult<Self::ResponsesFromIDsStream>{
        let mut log_queue = LOG_QUEUE.lock().await;
        let (tx, rx): (mpsc::Sender<Result<camila_response::Response, tonic::Status>>, mpsc::Receiver<Result<camila_response::Response, tonic::Status>>) = mpsc::channel(256);
        log_queue.getItemsByID(tx, &request.get_ref().id.as_slice()).await.expect("getItemsByID");

        let out_stream = ReceiverStream::new(rx);
        Ok(tonic::Response::new(Box::pin(out_stream) as Self::ResponsesFromIDsStream))
    }   


    async fn responses_from_types(&self, request: Request<camila_response::sorts_by::Types>)
    -> RpcResult<Self::ResponsesFromTypesStream>{
        let mut log_queue = LOG_QUEUE.lock().await;
        let (tx, rx): (mpsc::Sender<Result<camila_response::Response, tonic::Status>>, mpsc::Receiver<Result<camila_response::Response, tonic::Status>>) = mpsc::channel(256);

        log_queue.getItemsByType(tx, request.get_ref().r#type.as_slice()).await.expect("getItemsByType");

        let out_stream = ReceiverStream::new(rx);
        Ok(tonic::Response::new(Box::pin(out_stream) as Self::ResponsesFromTypesStream))
    }

    async fn responses_from_time(&self, request: Request<camila_response::sorts_by::Time>)
    -> RpcResult<Self::ResponsesFromTimeStream>{
        let mut log_queue = LOG_QUEUE.lock().await;
        let (tx, rx): (mpsc::Sender<Result<camila_response::Response, tonic::Status>>, mpsc::Receiver<Result<camila_response::Response, tonic::Status>>) = mpsc::channel(256);

        log_queue.getItemsByTime(tx, request.get_ref().start_time_milli, request.get_ref().end_time_milli).await.expect("getItemsByTime");

        let out_stream = ReceiverStream::new(rx);
        Ok(tonic::Response::new(Box::pin(out_stream) as Self::ResponsesFromTimeStream))
    }


    type AllResponseQueueStream = ResponseStream<camila_response::Response>;

    type ResponsesFromIDsStream = ResponseStream<camila_response::Response>;

    type ResponsesFromTypesStream = ResponseStream<camila_response::Response>;

    type ResponsesFromTimeStream = ResponseStream<camila_response::Response>;
}



