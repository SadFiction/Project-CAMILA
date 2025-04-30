use super::*;

#[derive(Debug, Default)]
pub struct CamilaCommandService;


impl CamilaCommandService {


    const TICK_GET: i32 = camila_command::CommandType::TickGet as i32;
    const TICK_SET: i32 = camila_command::CommandType::TickSet as i32;
    const _SET_CAMILA_OBJECT: i32 = camila_command::CommandType::SetCamilaObject as i32;
    const _GET_CAMILA_OBJECT: i32 = camila_command::CommandType::GetCamilaObject as i32;
    const INPUT_SEND: i32 = camila_command::CommandType::InputSend as i32;
    const INPUT_SEND_SERIES: i32 = camila_command::CommandType::InputSeriesSend as i32;
    const GET_OUTPUT: i32 = camila_command::CommandType::GetOutput as i32;
    const GET_OUTPUT_SERIES: i32 = camila_command::CommandType::GetOutputSeries as i32;
    const REWARD: i32 = camila_command::CommandType::Reward as i32;
    const RULESET_GET: i32 = camila_command::CommandType::RulesetGet as i32;
    const RULESET_SET: i32 = camila_command::CommandType::RulesetSet as i32;

    async fn check_validy(request: &camila_command::Command) -> 
        camila_command::CommandStatus{
        let mut status: camila_command::CommandStatus = camila_command::CommandStatus::None;
        match request.r#type{
            Self::TICK_SET => {
                if !request.arguments.is_empty(){
                    request.arguments.iter().for_each( |e| {
                    if e.0.to_lowercase() != "tick"{
                        
                        status = if status == camila_command::CommandStatus::None {camila_command::CommandStatus::ArgumentKeyInvalid}else { camila_command::CommandStatus::None}
                    }
                    else{
                

                    match  e.1.arg_type.as_ref().unwrap() {
                        camila_command::arg_type::ArgType::IntegerType(value) => {
                            if !(value.clone() >=1 || value.clone() <= 1000 ){
                                
                                status = if status == camila_command::CommandStatus::None {camila_command::CommandStatus::ArgumentContentsInvalid} else { camila_command::CommandStatus::None}
                            }
                            else{
                                
                                status = camila_command::CommandStatus::None;
                            }
                        },
                        
                        _ => { status = if status == camila_command::CommandStatus::None {camila_command::CommandStatus::ArgumentContentsInvalid} else { camila_command::CommandStatus::None}}
                    }
                }

            })}
            else{
                status = camila_command::CommandStatus::ArgumentNull
            }
            },
            Self::TICK_GET => status = if !request.arguments.is_empty() {camila_command::CommandStatus::ArgumentNull} else {camila_command::CommandStatus::None} ,
            
            Self::RULESET_SET => status = camila_command::CommandStatus::CurrentlyUnavailable,
            Self::RULESET_GET => status = camila_command::CommandStatus::CurrentlyUnavailable,
            Self::REWARD => status = camila_command::CommandStatus::CurrentlyUnavailable,
            Self::GET_OUTPUT_SERIES=> status = camila_command::CommandStatus::CurrentlyUnavailable,
            Self::GET_OUTPUT => status = camila_command::CommandStatus::CurrentlyUnavailable,
            Self::INPUT_SEND=> status = camila_command::CommandStatus::CurrentlyUnavailable,
            Self::INPUT_SEND_SERIES => status = camila_command::CommandStatus::CurrentlyUnavailable,
            _ => status = camila_command::CommandStatus::ArgumentNull
        };
        println!("{:?}", status);
        return status;
    }
    

}




#[tonic::async_trait]
impl CamilaCommands for CamilaCommandService {

    async fn send_command_cycle(&self, request: Request<camila_command::Command>)
     -> RpcResult<camila_command::Status>{
        let request: &camila_command::Command = request.get_ref(); 
        let mut status = Self::check_validy(request).await;
        let mut cycle_command_queue = CYCLE_COMMAND_QUEUE.lock().await;
        
        if status == camila_command::CommandStatus::None{
            match cycle_command_queue.push_queue(request.clone()).await {
                Err("FILLED") => { status = camila_command::CommandStatus::QueueFullCannotPush},
                Err("FULL") => status = camila_command::CommandStatus::QueueFilled,
                Ok(_) => status = camila_command::CommandStatus::OnQueue,
                Err(_)  => {}
            };
        }
        

        Ok(tonic::Response::new(camila_command::Status{
             status: status as i32
            }
        ))        
    }
    async fn send_command_queue_independent(&self,request: Request<camila_command::Command>)
     ->  RpcResult<camila_command::Status> {
        let request: &camila_command::Command = request.get_ref(); 
        let mut status = Self::check_validy(request).await;
        let mut independent_command_queue = INDEPENDENT_COMMAND_QUEUE.lock().await;
        
        if status == camila_command::CommandStatus::None{
            match independent_command_queue.push_queue(request.clone()).await {
                Err("FILLED") => { status = camila_command::CommandStatus::QueueFullCannotPush},
                Err("FULL") => status = camila_command::CommandStatus::QueueFilled,
                Ok(_) => status = camila_command::CommandStatus::OnQueue,
                Err(_)  => {}
            };
        }
        

        Ok(tonic::Response::new(camila_command::Status{
             status: status as i32
            }
        ))      
    }
    async fn send_command_series_cycle(&self,request: RequestStream<camila_command::Command>)
     -> RpcResult<Self::SendCommandSeriesCycleStream>{
         
        let mut incoming_stream = request.into_inner();

        let (tx, rx) = mpsc::channel(256);

        tokio::spawn(
            async move{
                while let Some(message) = incoming_stream.next().await{
                      match  message{
                            Ok(v) => {



                                let x = Self.send_command_cycle(tonic::Request::new(v)).await.unwrap().into_inner();

                                 tx.send(Ok(x))
                                 .await.expect("working rx")
                            },
                            Err(_) => {}
                      }
                }
        
            }
        );
        let out_stream = ReceiverStream::new(rx);


        Ok(tonic::Response::new(Box::pin(out_stream) as Self::SendCommandSeriesCycleStream))

    }
    async fn send_command_series_queue_independent(&self,request: RequestStream<camila_command::Command>)
     ->  RpcResult<Self::SendCommandSeriesQueueIndependentStream>{
        let mut incoming_stream = request.into_inner();

        let (tx, rx) = mpsc::channel(256);

        tokio::spawn(
            async move{
                while let Some(message) = incoming_stream.next().await{
                      match  message{
                            Ok(v) => {



                                let x = Self.send_command_queue_independent(tonic::Request::new(v)).await.unwrap().into_inner();

                                 tx.send(Ok(x))
                                 .await.expect("working rx")
                            },
                            Err(_) => {}
                      }
                }
        
            }
        );
        let out_stream = ReceiverStream::new(rx);


        Ok(tonic::Response::new(Box::pin(out_stream) as Self::SendCommandSeriesQueueIndependentStream))
        
    }
    async fn send_command_series_tick(&self, request: RequestStream<camila_command::Command>,)
     ->  RpcResult<Self::SendCommandSeriesTickStream>{
        
        let mut incoming_stream = request.into_inner();

        let (tx, rx) = mpsc::channel(256);

        tokio::spawn(
            async move{
                while let Some(message) = incoming_stream.next().await{
                      match  message{
                            Ok(v) => {



                                let x = Self.send_command_tick(tonic::Request::new(v)).await.unwrap().into_inner();

                                 tx.send(Ok(x))
                                 .await.expect("working rx")
                            },
                            Err(_) => {}
                      }
                }
        
            }
        );
        let out_stream = ReceiverStream::new(rx);


        Ok(tonic::Response::new(Box::pin(out_stream) as Self::SendCommandSeriesTickStream))
        

    }
    async fn send_command_tick(&self,request: Request<camila_command::Command> ,)
     ->  RpcResult<camila_command::Status>{
        let request: &camila_command::Command = request.get_ref(); 
        let mut status = Self::check_validy(request).await;
        let mut tick_command_queue = TICK_COMMAND_QUEUE.lock().await;
        
        if status == camila_command::CommandStatus::None{
            match tick_command_queue.push_queue(request.clone()).await {
                Err("FILLED") => { status = camila_command::CommandStatus::QueueFullCannotPush},
                Err("FULL") => status = camila_command::CommandStatus::QueueFilled,
                Ok(_) => status = camila_command::CommandStatus::OnQueue,
                Err(_)  => {}
            };
        }
        

        Ok(tonic::Response::new(camila_command::Status{
             status: status as i32
            }
        ))      
    }
    
    type SendCommandSeriesTickStream = ResponseStream<camila_command::Status>;
    
    type SendCommandSeriesCycleStream = ResponseStream<camila_command::Status>;

    type SendCommandSeriesQueueIndependentStream = ResponseStream<camila_command::Status>;
    

}
