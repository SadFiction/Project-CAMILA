use super::*;

#[derive(Debug, Default)]
pub struct CamilaCommandService;

#[tonic::async_trait]
impl CamilaCommands for CamilaCommandService {
    async fn send_command_cycle(&self,request: Request<camila_command::Command>)
     ->  RpcResult<camila_command::Status>{
        todo!()
    }
    async fn send_command_queue_independent(&self,request: Request<camila_command::Command>)
     ->  RpcResult<camila_command::Status> {
        todo!()
    }
    async fn send_command_series_cycle(&self,request: RequestStream<camila_command::Command>)
     -> RpcResult<Self::SendCommandSeriesCycleStream>{
        todo!()   
    }
    async fn send_command_series_queue_independent(&self,request: RequestStream<camila_command::Command>)
     ->  RpcResult<Self::SendCommandSeriesQueueIndependentStream>{
        todo!()
    }
    async fn send_command_series_tick(&self, request: RequestStream<camila_command::Command>,)
     ->  RpcResult<Self::SendCommandSeriesTickStream>{
        todo!()
    }
    async fn send_command_tick(&self,request: Request<camila_command::Command> ,)
     ->  RpcResult<camila_command::Status>{
        todo!()   
    }
    
    type SendCommandSeriesTickStream = ResponseStream<camila_command::Status>;
    
    type SendCommandSeriesCycleStream = ResponseStream<camila_command::Status>;
    
    type SendCommandSeriesQueueIndependentStream = ResponseStream<camila_command::Status>;
    

}
