use super::*;

#[derive(Debug, Default)]
pub struct CamilaGetResponseService;

#[tonic::async_trait]
impl CamilaGetResponse for CamilaGetResponseService{
    
    async fn pop_response_from_queue(&self, request: Request<camila_response::sorts_by::Empty>)
    -> RpcResult<camila_response::Response>{
        todo!()
    }

    async fn all_response_queue(&self, request: Request<camila_response::sorts_by::Empty>)
    -> RpcResult<Self::AllResponseQueueStream>{
        todo!()
    }

    async fn responses_from_i_ds(&self, request: Request<camila_response::sorts_by::IDs>)
    -> RpcResult<Self::ResponsesFromIDsStream>{
        todo!()
    }

    async fn responses_from_types(&self, request: Request<camila_response::sorts_by::Types>)
    -> RpcResult<Self::ResponsesFromTypesStream>{
        todo!()
    }

    async fn responses_from_time(&self, request: Request<camila_response::sorts_by::Time>)
    -> RpcResult<Self::ResponsesFromTimeStream>{
        todo!()
    }


    type AllResponseQueueStream = ResponseStream<camila_response::Response>;

    type ResponsesFromIDsStream = ResponseStream<camila_response::Response>;

    type ResponsesFromTypesStream = ResponseStream<camila_response::Response>;

    type ResponsesFromTimeStream = ResponseStream<camila_response::Response>;
}



