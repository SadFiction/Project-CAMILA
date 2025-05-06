use std::vec;


use tokio::time::Instant;

use super::*;



pub struct CamilaObject{
    grid: Vec<Vec<cob::tick::grid::Cell>>,
    
    size_x: u64,
    size_y: u64

}

impl CamilaObject{
    pub fn new(size_x: u64,size_y: u64) -> Self{
        CamilaObject {
            grid: vec![vec![cob::tick::grid::Cell{ structure_layer: 0.0, transmission_layer: 0.0}; size_x as usize]; size_y as usize],
            size_x: size_x,
            size_y: size_y
         
         }
    }
    async fn runCommand(command: camila_command::Command) -> () {
        match camila_command::CommandType::try_from(command.r#type).unwrap() {
            camila_command::CommandType::GetCamilaObject => {},
            camila_command::CommandType::SetCamilaObject=> {},
            camila_command::CommandType::TickGet => {},
            camila_command::CommandType::TickSet => {},
            _ => {}
        }
    }
    
    
    

    pub async fn update(&mut self) {
        let mut _temp_grid: Vec<Vec<cob::tick::grid::Cell>> =  vec![vec![cob::tick::grid::Cell{ structure_layer: 0.0, transmission_layer: 0.0}; self.size_x as usize]; self.size_y as usize];
        let tick_timer: Instant = tokio::time::Instant::now();
      
        //running commands for the tick
        if !TICK_COMMAND_QUEUE.lock().await.is_empty() {
            let command = TICK_COMMAND_QUEUE.lock().await.pop_queue().await.unwrap();
            Self::runCommand(command).await;
            
        }


        for y in 0..(self.size_y -1) as usize {
            for x in 0..(self.size_x -1)  as usize {

                if self.grid[y][y].structure_layer >= 0.5{
                    println!("Updating cells: ");
                    let mut neighbors = vec![vec![Some(cob::tick::grid::Cell{structure_layer: 0.0, transmission_layer: 0.0}) ;3]; 3];
                    
                    // Get the neighborhood of the cell based on its position
                    match (x,y){
                        (0,0) => {
                            neighbors[0][0] = None;
                            neighbors[1][0] = None;
                            neighbors[2][0] = None;
                            neighbors[0][1] = None;
                            neighbors[1][1] = Some(self.grid[x][y].clone()); 
                            neighbors[2][1] = Some(self.grid[x+1][y].clone());
                            neighbors[0][2] = None;
                            neighbors[1][2] = Some(self.grid[x][y+1].clone());
                            neighbors[2][2] = Some(self.grid[x+1][y+1].clone());
                        },
                        (x,0) if x == (self.size_x as usize -1) => {
                            neighbors[0][0] = None;
                            neighbors[1][0] = None;
                            neighbors[2][0] = None;
                            neighbors[0][1] = Some(self.grid[x-1][y].clone());
                            neighbors[1][1] = Some(self.grid[x][y].clone()); 
                            neighbors[2][1] = None;
                            neighbors[0][2] = Some(self.grid[x-1][y+1].clone());
                            neighbors[1][2] = Some(self.grid[x][y+1].clone());
                            neighbors[2][2] = None;
                        },
                        (0 , y) if y == (self.size_y as usize -1) => {
                            neighbors[0][0] = None;
                            neighbors[1][0] = Some(self.grid[x-1][y].clone());
                            neighbors[2][0] = Some(self.grid[x-1][y+1].clone());
                            neighbors[0][1] = None;
                            neighbors[1][1] = Some(self.grid[x][y].clone()); 
                            neighbors[2][1] = Some(self.grid[x+1][y].clone());
                            neighbors[0][2] = None;
                            neighbors[1][2] = None;
                            neighbors[2][2] = None;
                        },
                        (x, y) if x == (self.size_x as usize -1) && y == (self.size_y as usize -1) => {
                            neighbors[0][0] = Some(self.grid[x-1][y-1].clone());
                            neighbors[1][0] = Some(self.grid[x][y-1].clone());
                            neighbors[2][0] = None;
                            neighbors[0][1] = Some(self.grid[x-1][y].clone());
                            neighbors[1][1] = Some(self.grid[x][y].clone()); 
                            neighbors[2][1] = None;
                            neighbors[0][2] = None;
                            neighbors[1][2] = None;
                            neighbors[2][2] = None;
                        },
                        (0,_) => {
                            neighbors[0][0] = None;
                            neighbors[1][0] = Some(self.grid[x-1][y].clone());
                            neighbors[2][0] = Some(self.grid[x-1][y+1].clone());
                            neighbors[0][1] = None;
                            neighbors[1][1] = Some(self.grid[x][y].clone()); 
                            neighbors[2][1] = Some(self.grid[x+1][y].clone());
                            neighbors[0][2] = None;
                            neighbors[1][2] = Some(self.grid[x+1][y].clone());
                            neighbors[2][2] = Some(self.grid[x+1][y+1].clone());
                        }, 
                        (_,0) => {
                            neighbors[0][0] = None;
                            neighbors[1][0] = None;
                            neighbors[2][0] = None;
                            neighbors[0][1] = Some(self.grid[x-1][y].clone());
                            neighbors[1][1] = Some(self.grid[x][y].clone()); 
                            neighbors[2][1] = Some(self.grid[x+1][y].clone());
                            neighbors[0][2] = Some(self.grid[x-1][y+1].clone());
                            neighbors[1][2] = Some(self.grid[x][y+1].clone());
                            neighbors[2][2] = Some(self.grid[x+1][y+1].clone());
                        },
                        (_,y) if y == (self.size_y as usize -1) => {
                            neighbors[0][0] = Some(self.grid[x-1][y-1].clone());
                            neighbors[1][0] = Some(self.grid[x][y-1].clone());
                            neighbors[2][0] = Some(self.grid[x+1][y-1].clone());
                            neighbors[0][1] = Some(self.grid[x-1][y].clone());
                            neighbors[1][1] = Some(self.grid[x][y].clone()); 
                            neighbors[2][1] = Some(self.grid[x+1][y].clone());
                            neighbors[0][2] = None;
                            neighbors[1][2] = None;
                            neighbors[2][2] = None;
                        },
                        (x,_) if x == (self.size_x as usize -1) => {
                            neighbors[0][0] = Some(self.grid[x-1][y-1].clone());
                            neighbors[1][0] = Some(self.grid[x][y-1].clone());
                            neighbors[2][0] = None;
                            neighbors[0][1] = Some(self.grid[x-1][y].clone());
                            neighbors[1][1] = Some(self.grid[x][y].clone()); 
                            neighbors[2][1] = None;
                            neighbors[0][2] = Some(self.grid[x-1][y+1].clone());
                            neighbors[1][2] = Some(self.grid[x][y+1].clone());
                            neighbors[2][2] = None;
                        },
                        (_,_) => {
                            neighbors[0][0] = Some(self.grid[x-1][y-1].clone());
                            neighbors[1][0] = Some(self.grid[x][y-1].clone());
                            neighbors[2][0] = Some(self.grid[x+1][y-1].clone());
                            neighbors[0][1] = Some(self.grid[x-1][y].clone());
                            neighbors[1][1] = Some(self.grid[x][y].clone()); 
                            neighbors[2][1] = Some(self.grid[x+1][y].clone());
                            neighbors[0][2] = Some(self.grid[x-1][y+1].clone());
                            neighbors[1][2] = Some(self.grid[x][y+1].clone());
                            neighbors[2][2] = Some(self.grid[x+1][y+1].clone());
                        }

                    }
                    
                    //apply ruleset here
                }
            }
        }

        let tick_time_elapsed = tick_timer.elapsed().as_millis();
        println!("Tick time: {}ms", tick_time_elapsed);
    }
}