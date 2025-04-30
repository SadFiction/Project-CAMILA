

import { getCurrentWindow } from "@tauri-apps/api/window";
import _throttle from "lodash/throttle"

let div2Top: number | undefined = 0;
let div2Bottom: number | undefined = 0;

//grid parameters
let zoom = 5.5;
let grid_size = 100;
let grid_square = 30;

let grid_array: number[][] = Array.from({ length: grid_size }, () => Array(grid_size).fill(0));;




console.log(grid_array)

//tool parameters
let line_size_radius = 3
let zap_size_radius = 3

let brush_size_radius = 3;
let brush_weight = 1;
let brush_type = "";

let zap_weight = 1;
let line_weight = 1;

let select = "";

function menuOpenClose(){
  let menu = document.querySelector(".menu__overlay") as HTMLElement;


  if (window.getComputedStyle(menu).display == "none"){
    menu.style.display = "flex";
  }
  else {
    menu.style.display = "none";
  }
}
function updateWalkthroughControls() {
  let control = document.querySelector('.tick__walkthrough__control');

  let div2 = control?.getBoundingClientRect();
  if (div2?.bottom)

  if (!(div2?.top == 0 && div2?.bottom == 0 )){
    
    console.log("isDIV")
    div2Top = div2?.top - 8;
    div2Bottom = div2?.bottom ;
    console.log(div2Top)
  }

 
  let div1 = document.querySelector(".tick__walkthrough__view")?.getBoundingClientRect();
  let div1Top = div1?.top;
  let div1Bottom = div1?.bottom

  
  if ((div1Bottom && div1Top && div2Bottom && div2Top))

  if ((div2Top > div1Top && div2Top < div1Bottom)||(div2Bottom > div1Top && div2Bottom < div1Bottom)) {
    (control as HTMLElement).style.display = "none";
    console.log("this")
  }else{
    ((control as HTMLElement)).style.display = "";
  }
  

  
}


//event listeners for top panel tools
window.addEventListener("DOMContentLoaded", () => {
  
  let brush  = document.getElementById("brush") as HTMLElement;
  let zap  = document.getElementById("zap") as HTMLElement;
  let line  = document.getElementById("line") as HTMLElement;

  let brush_options  = document.getElementById("brushToolOptions") as HTMLElement;
  let zap_options  = document.getElementById("zapToolOptions") as HTMLElement;
  let line_options  = document.getElementById("lineToolOptions") as HTMLElement;
  let no_options  = document.getElementById("noTool") as HTMLElement;



  let slider_weight_zap = document.getElementById("zap_weight_slider") as HTMLInputElement;
  let slider_size_zap = document.getElementById("zap_size_slider") as HTMLInputElement;
  let slider_weight_line = document.getElementById("line_weight_slider") as HTMLInputElement;
  let slider_size_line = document.getElementById("line_size_slider") as HTMLInputElement;
  let slider_weight_brush = document.getElementById("brush_weight_slider") as HTMLInputElement;
  let slider_size_brush = document.getElementById("brush_size_slider") as HTMLInputElement;

 slider_weight_zap.oninput = () => {
    (document.getElementById("zap_weight_view") as HTMLElement).innerHTML = slider_weight_zap.value;
    zap_weight = Number(slider_weight_zap.value);
 };
  slider_size_zap.oninput = () => {
    (document.getElementById("zap_size_view") as HTMLElement).innerHTML = slider_size_zap.value;
    zap_size_radius = Number( slider_size_zap.value);
  };
  slider_weight_line.oninput = () => {
    (document.getElementById("line_weight_view") as HTMLElement).innerHTML = slider_weight_line.value;
    line_weight = Number(slider_weight_line.value);
  };
  slider_size_line.oninput = () => {
    (document.getElementById("line_size_view") as HTMLElement).innerHTML = slider_size_line.value;
    line_size_radius = Number(slider_size_line.value);
  };
  slider_weight_brush.oninput = () => {
    (document.getElementById("brush_weight_view") as HTMLElement).innerHTML = slider_weight_brush.value;
    brush_weight = Number(slider_weight_brush.value);
  };
  slider_size_brush.oninput = () => {
    (document.getElementById("brush_size_view") as HTMLElement).innerHTML = slider_size_brush.value;
    brush_size_radius = Number(slider_size_brush.value);
  };

  brush.addEventListener("click", () => {
    if (select != "brush"){
    brush.style.backgroundColor="#2e2e2e";
    zap.style.backgroundColor = "";
    line.style.backgroundColor = "";

    brush_options.style.display = "flex";
    zap_options.style.display = "";
    line_options.style.display = "";
    no_options.style.display = "none";

    select = "brush"

    }
    else{
      brush.style.backgroundColor="";
      zap.style.backgroundColor = "";
      line.style.backgroundColor = "";

      brush_options.style.display = "";
      zap_options.style.display = "";
      line_options.style.display = "";
      no_options.style.display = "";
      select = "none"
    }
    
  })
  zap.addEventListener("click", () => {
    if (select != "zap"){
    brush.style.backgroundColor="";
    zap.style.backgroundColor = "#2e2e2e";
    line.style.backgroundColor = "";

    brush_options.style.display = "";
    zap_options.style.display = "flex";
    line_options.style.display = "";
    no_options.style.display = "none";

    select = "zap"
    }
    else{
      brush.style.backgroundColor="";
      zap.style.backgroundColor = "";
      line.style.backgroundColor = "";

      brush_options.style.display = "";
      zap_options.style.display = "";
      line_options.style.display = "";
      no_options.style.display = "";
      select = "none"
    }
    
  })
  line.addEventListener("click", () => {
    if (select != "line"){
    brush.style.backgroundColor="";
    zap.style.backgroundColor = "";
    line.style.backgroundColor = "#2e2e2e";

    brush_options.style.display = "";
    zap_options.style.display = "";
    line_options.style.display = "flex";
    no_options.style.display = "none";

    select = "line"
    }
    else{
      brush.style.backgroundColor="";
      zap.style.backgroundColor = "";
      line.style.backgroundColor = "";

      brush_options.style.display = "";
      zap_options.style.display = "";
      line_options.style.display = "";
      no_options.style.display = "";
      select = "none"
    }
    
  })



});

//Event listeners for titlebar
window.addEventListener("DOMContentLoaded", () =>{
  const appWindow = getCurrentWindow();
  document.getElementById("more_options")?.addEventListener("click" , menuOpenClose);
  document.getElementById("minimize")?.addEventListener("click",appWindow.minimize);
  document.getElementById("maximise")?.addEventListener("click",appWindow.maximize);
  document.getElementById("close")?.addEventListener("click", appWindow.close);

});
//evemt listeners for grid
window.addEventListener("DOMContentLoaded", () =>{
  document.getElementById("zoom_in")?.addEventListener("click", () =>{
      if (zoom <= 5.5){
      zoom *= 1.2
      drawGrid(grid_square, grid_size, zoom)
      }
  })
  document.getElementById("zoom_out")?.addEventListener("click", () => {
    if (zoom >=0.5){
    zoom /= 1.2
    drawGrid(grid_square, grid_size, zoom)
    }
  })
  window.addEventListener("resize", () => drawGrid(grid_square, grid_size, zoom) )
 
  

  
  let grid_set: boolean = false
  let grid: HTMLCanvasElement = document.getElementById("sandboxGrid") as HTMLCanvasElement;
  const ctx: CanvasRenderingContext2D = grid.getContext("2d") as CanvasRenderingContext2D;
  const gridSet = (grid_dimension: number ,dpr:number) => {
    
    if (grid_set == false){
      grid.width = grid_dimension *dpr;
      grid.height = grid_dimension *dpr;
      ctx.scale(dpr, dpr);
      grid_set=true;
    }else{
      grid.width = grid_dimension;
      grid.height =grid_dimension;
    }
   
  }
  
  let renderQueueSize = 0;
  

  const drawGrid = (square_dimension:number, number_dimension_squares: number, zoom_factor:number) =>{
    ctx.strokeStyle = "#2e2e2e";
    ctx.lineWidth = 1;
    
    square_dimension = square_dimension/zoom_factor
    let grid_dimension_pixels = number_dimension_squares*square_dimension
    renderQueueSize +=1;
    console.log("Rendering:" + renderQueueSize);
     const dpr =  (window.devicePixelRatio )|| 1;

     gridSet(grid_dimension_pixels, dpr)
     
     
     ctx.translate(-20,-20)
     ctx.clearRect(0,0,grid.width, grid.height)
     console.log(grid.height)
      
     for (let x= 5;x <= number_dimension_squares; x += 1){
      if ((x-5)%5==0){
        ctx.strokeStyle="#B22222";
        ctx.lineWidth = 1;
        ctx.beginPath();
        ctx.setLineDash([square_dimension, square_dimension*2]);
        ctx.moveTo(x*square_dimension,40);
        ctx.lineTo(x*square_dimension,grid.height+40);
        ctx.stroke()
        ctx.setLineDash([]);
        ctx.strokeStyle="#2e2e2e";
        ctx.lineWidth = 1;
      }
      for (let y=5; y<=number_dimension_squares; y+=1){
        if ((y-5)%5==0){
          ctx.strokeStyle="#B22222";
          ctx.lineWidth = 1;
          ctx.beginPath();
          ctx.setLineDash([square_dimension, square_dimension*2]);
          ctx.moveTo(40,y*square_dimension);
          ctx.lineTo(grid.width+40, y*square_dimension);
          ctx.stroke()
          ctx.setLineDash([]);
          ctx.strokeStyle="#2e2e2e";
          ctx.lineWidth = 1;
        }
        
        if (grid_array[x-5][y-5] != 0){
          drawSquare(grid_array[x-5][y-5],"#2e2e2e", square_dimension*zoom_factor, number_dimension_squares, zoom_factor, x-5, y-5);
          console.log("rendering squares")
        }

        
        ctx.strokeRect(Math.floor(x*square_dimension),Math.floor(y*square_dimension), square_dimension, square_dimension)
      }
     }
     

    

  }

  const setHexOpacity = (hex: string, opacity: number)  =>{

    if (hex.startsWith("#")) hex = hex.slice(1);   
    if (hex.length !== 6) throw new Error("Invalid hex color");

   
    const alpha = Math.round(opacity * 255).toString(16).padStart(2, "0");

    return "#" + hex + alpha;
  }


  const drawSquare = (weight:number,color:string, square_dimension:number, number_dimension_squares: number, zoom_factor: number, X:number, Y:number) =>{
    square_dimension /= zoom_factor;
    
    
    for (let x= 5;x <= number_dimension_squares; x += 1){
      for (let y=5; y<=number_dimension_squares; y+=1){
        if ( x == (X+5) && y == (Y+5)){
          console.log("square drawn")
        ctx.beginPath();
        ctx.fillStyle=setHexOpacity(color, weight);
        ctx.fillRect(Math.floor(x*square_dimension),Math.floor(y*square_dimension), square_dimension, square_dimension)
        }
    }}}
  
  const getMousePosCanvas = (canvas:HTMLCanvasElement, evt : MouseEvent) => {
    let rect = canvas.getBoundingClientRect();

    return {
        x: ((evt.clientX - rect.left) / (rect.right - rect.left) * canvas.width - (grid_square/zoom)*5 +20),
        y: ((evt.clientY - rect.top) / (rect.bottom - rect.top) * canvas.height -  (grid_square/zoom)*5 +20)
    };
  }




  const circleGet= (X:number, Y:number, radius: number, number_dimension_squares: number) => {
    
    let circle_pos_array: {x:number, y:number}[] = new Array(2*radius**2);


    for (let x= 0;x <= number_dimension_squares; x += 1){
      for (let y=0; y<=number_dimension_squares; y+=1){
          if ((x-X)**2+(y-Y)**2 < radius**2 ){

            circle_pos_array.push({x:x, y:y});
          }

        }
      }
    
    return circle_pos_array;
  }

  const lineGet = (start_x: number, start_y: number, end_x:number, end_y: number, number_dimension_squares: number, line_width: number) =>{
    let line_pos_array: {x:number, y:number}[] = new Array(number_dimension_squares*2);

   


    let gradient:number =  (end_y-start_y)/ (end_x-start_x);

  

    
    if (Number.isNaN(gradient)){
      gradient = 1;
    }
    console.log("Gradient line: " + gradient)


    let xintervalstart = start_x;
    let xintervalend = end_x;
    let yintervalstart = start_y;
    let yintervalend = end_y; 


    if (start_x > end_x && start_y > end_y){
      xintervalstart = end_x;
      xintervalend = start_x;

      yintervalstart = end_y;
      yintervalend = start_y;


      
    }
    else if (start_x < end_x && start_y > end_y){
      yintervalstart = end_y;
      yintervalend = start_y;
      
    }
    else if (start_x > end_x && start_y > end_y){
      yintervalstart = end_y;
      yintervalend = start_y;
      
    }
    else if (start_x > end_x){
      xintervalstart = end_x;
      xintervalend = start_x;

    }
    else if (start_y > end_y){
      yintervalstart = end_y;
      yintervalend = start_y;
    }
   


    for (let x= xintervalstart;x <= xintervalend; x += 1){
      for (let y= yintervalstart; y<=yintervalend; y +=1){
          if ((Math.floor(x) == start_x) && (gradient == Infinity || gradient == -Infinity)){
            
            
            for(let i = -line_width*0.5; i <=line_width*0.5; i += 1){
              line_pos_array.push({x:Math.floor(x+i), y:Math.floor(y)});
            }

          }
          else if ((Math.floor(y) == start_y) && gradient == 0){
            
            
            for(let i = -line_width*0.5; i <=line_width*0.5; i += 1){
              line_pos_array.push({x:Math.floor(x), y:Math.floor(y+i)});
            }

          }

          else if ((Math.floor(y) > Math.floor(gradient*(x- end_x)+ end_y-line_width) ) && (Math.floor(y) < Math.floor(gradient*(x- end_x)+ end_y+line_width) )) {
            
            line_pos_array.push({x:Math.floor(x), y:Math.floor(y)});
          }
          
        }
      }
    
    return line_pos_array;
  };

  let toggle = false;
  let old_position = {x:0, y:0};

  grid.addEventListener("click", (e:MouseEvent) => {
    let position = getMousePosCanvas(grid, e);
    position.x = Math.floor((position.x )/(grid_square/zoom));
    position.y = Math.floor((position.y)/(grid_square/zoom));

    console.log("Canvas x: "+position.x + " Canvas y: "+ position.y)
    
    

    if (select == "brush"){ 
    const circle = circleGet(position.x,position.y, brush_size_radius, grid_size )
    
    circle.forEach( (e) => {
      if (grid_array[e.x][e.y] + brush_weight < 1){
      grid_array[e.x][e.y] += brush_weight;

      }
      else{
        grid_array[e.x][e.y] = 1
      }
      drawSquare(grid_array[e.x][e.y],"#2e2e2e", grid_square,grid_size,zoom,e.x,e.y);

    })

    }

    if (select == "line"){
    
      if (!toggle){
        old_position.x = position.x;
        old_position.y = position.y;
        toggle = true;

      }
      else{
        const line = lineGet(old_position.x, old_position.y, position.x, position.y, grid_size, line_size_radius)
        line.forEach( (e) => {
          if (grid_array[e.x][e.y] + line_weight < 1){
            grid_array[e.x][e.y] += line_weight;
      
            }
            else{
              grid_array[e.x][e.y] = 1
            }
          drawSquare(grid_array[e.x][e.y],"#2e2e2e", grid_square,grid_size,zoom,e.x,e.y)
        })

        toggle = false;

    }
  }
  });
    
  
  drawGrid(grid_square, grid_size, zoom);


});

window.addEventListener("DOMContentLoaded", () => {
  window.addEventListener("resize", updateWalkthroughControls )
  
}); 



//event listeners for resizing containers
window.addEventListener("DOMContentLoaded", () => {
  const resize = (resizer: HTMLElement) => {

    //get relative position and neighbour containers
    const position: string | null = resizer.getAttribute("data-position");
    const upContainer: HTMLElement | null = resizer.previousElementSibling as HTMLElement || null;
    const downContainer: HTMLElement | null = resizer.nextElementSibling as HTMLElement || null;

    //get position
    let y = 0;
    let upContainerHeight = 0;
    let downContainerHeight = 0;



    const mouseDown = (e: MouseEvent) => {
      //get position of mouse 
      y = e.clientY;

      //set initial height of container
      upContainerHeight = upContainer.getBoundingClientRect().height;
      downContainerHeight = downContainer.getBoundingClientRect().height;
      
      resizer.style.cursor = "row-resize";
      document.body.style.cursor = "row-resize";

      //Listen for changes
      document.addEventListener("mouseup", mouseUp);
      document.addEventListener("mousemove", mouseMove);
    };


    const mouseUp = (_:MouseEvent) => {
      document.body.style.removeProperty("cursor")
      resizer.style.removeProperty('cursor');

      upContainer.style.removeProperty('user-select');
      upContainer.style.removeProperty('pointer-events');

      downContainer.style.removeProperty('user-select');
      downContainer.style.removeProperty('pointer-events');

      
      //remove listeners
      document.removeEventListener('mousemove', mouseMove);
      document.removeEventListener('mouseup', mouseUp);


    };

    const mouseMove = (e: MouseEvent) => {
          //get the change in position of mouse
          const dy = e.clientY - y 
          let minHeight = window.innerHeight*0.006
      
          //update position of up container and get as a percentage of its parent container
          const heightOfParent = (resizer.parentElement as HTMLElement).getBoundingClientRect().height;

          switch (position){
            case "up":
              const newUpContainerHeight = ((upContainerHeight +dy) / heightOfParent) * 100 ;
              if (minHeight <= newUpContainerHeight){

                //unhide if the panel is "pulled" out
                if (upContainer.style.display == "none"){
                  upContainer.style.display = "";
                  resizer.style.position = "";
                }
                upContainer.style.height = newUpContainerHeight + "%";
              }
              else{
                //hide after a certain threshold of minHeight
                upContainer.style.display = "none";
                resizer.style.position = "fixed";
                resizer.style.top = 30 + "px";

              }
              break;

            case "down":

                const newDownContainerHeight = ((downContainerHeight -dy)/ heightOfParent) * 100;
                
                if (minHeight <= newDownContainerHeight){
                downContainer.style.height = newDownContainerHeight + "%";
                  if (downContainer.style.display == "none"){
                      downContainer.style.display = "";
                      resizer.style.position = "";
                  }
                updateWalkthroughControls()
                }
                else {
                  downContainer.style.display = "none";
                  resizer.style.position = "fixed"
                  resizer.style.bottom = 0 + "px"
                  
                }
             
              break;
          }

          //Disable text selection and any other mouse actions whle dragging 
          upContainer.style.userSelect = "none";
          upContainer.style.pointerEvents = "none";
          downContainer.style.userSelect = "none";
          downContainer.style.pointerEvents = "none";
          
          
    }

    //Listen for when the resizer is clicked
    resizer.addEventListener("mousedown", mouseDown);
    
  };

  
  Array.from(document.getElementsByClassName("resizer")).forEach((r) => {
    resize(r as HTMLElement)
  }

  );
  




  /*
let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    
    }
    
  );
  
  }
}

window.addEventListener("DOMContentLoaded", () => {
  

  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
*/















});
