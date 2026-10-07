#![allow(unused_parens)]
#![allow(non_snake_case)]

use nannou::prelude::*;
use nannou::glam::IVec2;
use nannou::rand::Rng;
use nannou::rand::thread_rng;

const WIDTH:u32=680;
const HEIGHT:u32=680;
const X_SPAN:usize=12;
const Y_SPAN:usize=12;
const X_OFFSET:f32=-220.;
const Y_OFFSET:f32=-220.;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile{PENDING, BLANK, NS, EW, NE, NW, SE, SW, NSE, NSW, NEW, SEW}

fn main() {
    nannou::app(model).size(WIDTH, HEIGHT).update(update).run();
}

struct Model {
	board_cell: [[Cell;X_SPAN];Y_SPAN],
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let _window = app.window(window_id).unwrap();  
	let mut board_cell: [[Cell;X_SPAN];Y_SPAN]=[[Cell::new(0,0); X_SPAN];Y_SPAN];
	for j in 0..Y_SPAN {
		for i in 0..X_SPAN {
		    board_cell[j][i]=Cell::new(i,j);
		}
	}
	let model:Model = Model {
		board_cell: board_cell,
	};
	return model;
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let next_cell:Vec<IVec2> = find_next_cell(&model);
    if(next_cell.len()==1){	
        collapse_cell(model, (next_cell[0][0] as usize), (next_cell[0][1] as usize));
    }
	else
	{
		app.set_loop_mode(LoopMode::loop_once());
	}
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
	draw.background().color(gray(192 as u8));
    display_board(&draw, &model);  
	draw.to_frame(app, &frame).unwrap();
}

fn display_board(draw:&nannou::Draw , model:&Model){
	let board_cell=&model.board_cell;
	let push_draw=draw.x_y(X_OFFSET, Y_OFFSET);
	for j in 0..Y_SPAN {
		for i in 0..X_SPAN {
		    board_cell[j][i].display_cell(&push_draw);
		}
	}
}

fn find_next_cell(model: &Model) -> Vec<IVec2> {
	let board_cell=&model.board_cell;
    let mut board_search_lowest_number_of_possible_tiles:usize=11;  // Starts at inital number and reduces with board search.
    let mut list_of_cells:Vec<IVec2> = Vec::new();  // or vec![];
    for j in 0..Y_SPAN {
		for i in 0..X_SPAN {
		    if(board_cell[j][i].get_number_of_cell_possible_tiles()==board_search_lowest_number_of_possible_tiles){  
                list_of_cells.push(IVec2::new(i as i32,j as i32));     
			}
            else if(board_cell[j][i].get_number_of_cell_possible_tiles()>1  &&  board_cell[j][i].get_number_of_cell_possible_tiles()<board_search_lowest_number_of_possible_tiles){
                board_search_lowest_number_of_possible_tiles=board_cell[j][i].get_number_of_cell_possible_tiles(); 
                list_of_cells = vec![];
                list_of_cells.push(IVec2::new(i as i32,j as i32));
		    }				
	    }
    }
	if(list_of_cells.len()==0){    return list_of_cells;    }
	else{
		return vec![list_of_cells[ (thread_rng().gen_range(0..(list_of_cells.len()))) as usize  ]]
	}
}

fn collapse_cell(model:&mut Model, x:usize, y:usize) {  
    let board_cell= &mut model.board_cell;
    let collapse_cell_tile:Tile=board_cell[y][x].get_cell_possible_tiles()[(thread_rng().gen_range(0..board_cell[y][x].get_number_of_cell_possible_tiles())) as usize];
    board_cell[y][x].set_cell_value(collapse_cell_tile);
    let tiles_with_n:Vec<Tile>= Vec::from([Tile::NS, Tile::NE, Tile::NW, Tile::NSE, Tile::NSW, Tile::NEW]);
    let tiles_without_n:Vec<Tile>= Vec::from([Tile::BLANK, Tile::EW, Tile::SE, Tile::SW, Tile::SEW]);
    let tiles_with_s:Vec<Tile> = vec![Tile::NS, Tile::SE, Tile::SW, Tile::NSE, Tile::SEW, Tile::NSW];
    let tiles_without_s:Vec<Tile> = vec![Tile::BLANK, Tile::EW, Tile::NE, Tile::NW, Tile::NEW];
	  
    if(y<Y_SPAN-1){
        if(tiles_without_n.contains(&collapse_cell_tile)){  // Tiles without N
            for invalid_as_neighbour_tile in &tiles_with_s {  // Tiles with S
                board_cell[y+1][x].remove_as_possible_tile(invalid_as_neighbour_tile);
            }
        } 
        if(tiles_with_n.contains(&collapse_cell_tile)){  // Tiles with N
            for invalid_as_neighbour_tile in &tiles_without_s {  // Tiles without S
               board_cell[y+1][x].remove_as_possible_tile(invalid_as_neighbour_tile);
            }
        }
    }  
    if(y>0){
        if(tiles_without_s.contains(&collapse_cell_tile)){  // Tiles without S
            for invalid_as_neighbour_tile in &tiles_with_n {  // Tiles with N
                board_cell[y-1][x].remove_as_possible_tile(invalid_as_neighbour_tile);
            }
        } 
        if(tiles_with_s.contains(&collapse_cell_tile)){  // Tiles with S
            for invalid_as_neighbour_tile in &tiles_without_n {  // Tiles without N
               board_cell[y-1][x].remove_as_possible_tile(invalid_as_neighbour_tile);
            }
        }		 
    }
      
	let tiles_with_e:Vec<Tile> = vec![Tile::EW, Tile::NE, Tile::SE, Tile::NSE, Tile::SEW, Tile::NEW];
    let tiles_without_e:Vec<Tile> = vec![Tile::BLANK, Tile::NS, Tile::SW, Tile::NW, Tile::NSW];
    let tiles_with_w:Vec<Tile> = vec![Tile::EW, Tile::SW, Tile::NW, Tile::SEW, Tile::NSW, Tile::NEW];
    let tiles_without_w:Vec<Tile> = vec![Tile::BLANK, Tile::NS, Tile::NE, Tile::SE, Tile::NSE];
	  
    if(x>0){
        if(tiles_without_w.contains(&collapse_cell_tile)){  // Tiles without W
		    board_cell[y][x-1].remove_all_as_possible_tiles(& tiles_with_e); // Tiles with E
		}
		if(tiles_with_w.contains(&collapse_cell_tile)){  // Tiles with W
		    board_cell[y][x-1].remove_all_as_possible_tiles(& tiles_without_e); // Tiles without E
		}
    }  
    if(x<X_SPAN-1){
		if(tiles_without_e.contains(&collapse_cell_tile)){  // Tiles without E
		    board_cell[y][x+1].remove_all_as_possible_tiles(& tiles_with_w); // Tiles with W
		}
        if(tiles_with_e.contains(&collapse_cell_tile)){  // Tiles with E
		    board_cell[y][x+1].remove_all_as_possible_tiles(& tiles_without_w); // Tiles without W
		}		 
    }
}

fn raw_window_event(app: &App, _model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    if let nannou::winit::event::WindowEvent::KeyboardInput { input, .. } = event {
        if let (Some(nannou::winit::event::VirtualKeyCode::F), true) =
            (input.virtual_keycode, input.state == nannou::winit::event::ElementState::Pressed)
        {
            let window = app.main_window();
            let fullscreen = window.fullscreen().is_some();
            window.set_fullscreen(!fullscreen);
        }
    }
} 

#[derive(Copy, Clone)]
struct Cell {
	posX:f32,
	posY:f32,
	cell_possible_tiles:[Tile;11],
}

impl Cell {
	fn new(i:usize, j:usize)->Cell{
	    let x:usize=i;
	    let y:usize=j;
        let posX:f32=(x*40) as f32;
        let posY:f32=(y*40) as f32;
	    let cell_possible_tiles:[Tile;11]=[Tile::BLANK, Tile::NS, Tile::EW, Tile::NE, Tile::NW, Tile::SE, Tile::SW, Tile::NSE, Tile::NSW, Tile::NEW, Tile::SEW];	   
	    return Cell{    posX:posX,
					    posY:posY,
					    cell_possible_tiles:cell_possible_tiles,
					 };
	}
	
	fn get_number_of_cell_possible_tiles(&self)->usize {
		let mut counter:usize=0;
		for tile in self.cell_possible_tiles {
			if tile!=Tile::PENDING {   counter+=1;  }
		}
		return counter;
	}
	
	fn get_cell_value(&self)->Tile {
		if(self.get_number_of_cell_possible_tiles()==1){   
		   let mut tile_value=Tile::PENDING;
		   for tile in self.cell_possible_tiles {
			  if tile!=Tile::PENDING {   tile_value=tile; break;  }
		   }
		   return(tile_value);   
		}
        else{   return(Tile::PENDING);   }
	}
	
	fn get_cell_possible_tiles(&self)->Vec<Tile> {
		let mut vec_of_cell_possible_tiles:Vec<Tile>=Vec::<Tile>::new();
		for tile in self.cell_possible_tiles {
			if tile!=Tile::PENDING {   vec_of_cell_possible_tiles.push(tile);  }
		}
		return vec_of_cell_possible_tiles;
	}
	
	fn set_cell_value(&mut self, cell_tile:Tile){
		for tile in &mut self.cell_possible_tiles {
			if(*tile!=cell_tile)  {   *tile=Tile::PENDING;   }
		}
	}
	
	fn remove_as_possible_tile(&mut self, invalid_tile:&Tile){
		for tile in &mut self.cell_possible_tiles {			
			if(*tile == *invalid_tile) {   *tile=Tile::PENDING; break;   }
		}
	}
	
	fn remove_all_as_possible_tiles(&mut self, invalid_tiles:& Vec<Tile>){
		for invalid_as_neighbour_tile in invalid_tiles {
		    for tile in &mut self.cell_possible_tiles {
			    if(*tile ==*invalid_as_neighbour_tile) {   *tile=Tile::PENDING; break;   }
		    }
		}
	}
	
	fn display_cell(&self, push_draw:&nannou::Draw){
		let posX=self.posX;
		let posY=self.posY;
        push_draw.rect().x_y(posX, posY).w_h(40., 40.).color(gray(64 as u8));
		push_draw.rect().x_y(posX+1., posY+1.).w_h(39., 39.).color(BLACK);
        let cell_value:Tile=self.get_cell_value();
        if (cell_value==Tile::BLANK) {    }  
        else if (cell_value==Tile::NS){   push_draw.rect().x_y(posX, posY).w_h(6., 40.);   }  
        else if (cell_value==Tile::EW){   push_draw.rect().x_y(posX, posY).w_h(40., 6.);   }  
        else if (cell_value==Tile::NE){   push_draw.rect().x_y(posX, posY+10.).w_h(6., 20.); push_draw.rect().x_y(posX+10., posY).w_h(20., 6.); push_draw.ellipse().x_y(posX, posY).radius(3.);   }  
        else if (cell_value==Tile::SE){   push_draw.rect().x_y(posX, posY-10.).w_h(6., 20.); push_draw.rect().x_y(posX+10., posY).w_h(20., 6.); push_draw.ellipse().x_y(posX, posY).radius(3.);  }     
        else if (cell_value==Tile::SW){   push_draw.rect().x_y(posX, posY-10.).w_h(6., 20.); push_draw.rect().x_y(posX-10., posY).w_h(20., 6.); push_draw.ellipse().x_y(posX, posY).radius(3.);   }
        else if (cell_value==Tile::NW){   push_draw.rect().x_y(posX, posY+10.).w_h(6., 20.); push_draw.rect().x_y(posX-10., posY).w_h(20., 6.); push_draw.ellipse().x_y(posX, posY).radius(3.);   }
        else if (cell_value==Tile::NSE){   push_draw.rect().x_y(posX, posY).w_h(6., 40.); push_draw.rect().x_y(posX+10., posY).w_h(20., 6.);  }  
        else if (cell_value==Tile::SEW){   push_draw.rect().x_y(posX, posY).w_h(40., 6.); push_draw.rect().x_y(posX, posY-10.).w_h(6., 20.);  }  
        else if (cell_value==Tile::NSW){   push_draw.rect().x_y(posX, posY).w_h(6., 40.); push_draw.rect().x_y(posX-10., posY).w_h(20., 6.);  }  
        else if (cell_value==Tile::NEW){   push_draw.rect().x_y(posX, posY).w_h(40., 6.); push_draw.rect().x_y(posX, posY+10.).w_h(6., 20.);  } 
        else{   push_draw.text(& self.get_number_of_cell_possible_tiles().to_string()).x_y( posX, posY);   }
    }
}


