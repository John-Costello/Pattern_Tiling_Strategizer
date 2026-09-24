const Xspan=12, Yspan=12;
const boardCell = [];

function setup() {
   createCanvas(680, 680);
   for(let j=0; j<Yspan; j++){
      let row=[];
      for(let i=0; i<Xspan; i++){
         row.push(new Cell(i, j));
      }
      boardCell.push(row);
   }
}

function draw() {
   background(192);
   displayBoard();
   update();
}

function displayBoard(){
   push();
      translate(100,100);
      for(let j=0; j<Yspan; j++){
         for(let i=0; i<Xspan; i++){
            boardCell[j][i].displayCell();
         }
      }
   pop();
}

function update(){
   let nextCell=findNextCell();
   if(nextCell.length==1){
      boardCell[nextCell[0][1]][nextCell[0][0]].collapseCell();
   }
   else
   {
      noLoop();    // All cells collapsed!!
   }
}

function findNextCell(){
   let boardSearchLowestNumberOfPossibleTiles=11;  // Starts at inital number and reduces with board search.
   let listOfCells= [];
   for(let j=0; j<Yspan; j++){
      for(let i=0; i<Xspan; i++){
         if(boardCell[j][i].getNumberOfCellPossibleTiles()==boardSearchLowestNumberOfPossibleTiles){
            listOfCells.push([i,j]);   }
         else if(boardCell[j][i].getNumberOfCellPossibleTiles()>1  &&  boardCell[j][i].getNumberOfCellPossibleTiles()<boardSearchLowestNumberOfPossibleTiles){
            boardSearchLowestNumberOfPossibleTiles=boardCell[j][i].getNumberOfCellPossibleTiles();
            listOfCells = [];
            listOfCells.push([i,j]);   
         }
      }
   }
   if(listOfCells.length==0){   return listOfCells;   }
   else{
      return(   [listOfCells[Math.floor(Math.random()*listOfCells.length)]]   );
   }   
}

const Tile = Object.freeze({
   PENDING:"PENDING", 
   BLANK:"BLANK",
   NS:"NS", EW:"EW", NE:"NE", NW:"NW", SE:"SE", SW:"SW",
   NSE:"NSE", NSW:"NSW", NEW:"NEW", SEW:"SEW"
 });
 
class Cell
{
   constructor(i, j){  
      this.x=i;
      this.y=j;
      this.posX=i*40;
      this.posY=j*40;
      this.cellPossibleTiles=[Tile.BLANK, Tile.NS, Tile.EW, Tile.NE, Tile.SE, Tile.SW, Tile.NW, Tile.NSE, Tile.NSW,Tile. NEW, Tile.SEW];
   }
    
   getCellValue(){
      if(this.cellPossibleTiles.length==1){   return(this.cellPossibleTiles[0]);   }
      else{   return(Tile.PENDING);   }   
   }
    
   getNumberOfCellPossibleTiles(){
      return this.cellPossibleTiles.length;
   }
   
   displayCell(){
      strokeWeight(1);
      stroke(64); 
      fill(0);   
      rect(this.posX, this.posY, 40, 40);     
      stroke(255);
      fill(255);
      let cellValue=this.getCellValue();
      if (cellValue===Tile.BLANK) {    }  
      else if (cellValue===Tile.NS){   rect(this.posX+17, this.posY , 6, 40);   }  
      else if (cellValue===Tile.EW){   rect(this.posX, this.posY+17 , 40, 6);   }  
      else if (cellValue===Tile.NE){   rect(this.posX+17, this.posY , 6, 20); rect(this.posX+20, this.posY+17 , 20, 6); circle(this.posX+20, this.posY+20, 6);   }  
      else if (cellValue===Tile.SE){   rect(this.posX+20, this.posY+17 , 20, 6); rect(this.posX+17, this.posY+20 , 6, 20); circle(this.posX+20, this.posY+20, 6);  }     
      else if (cellValue===Tile.SW){   rect(this.posX+17, this.posY+20 , 6, 20); rect(this.posX, this.posY+17 , 20, 6); circle(this.posX+20, this.posY+20, 6);   }
      else if (cellValue===Tile.NW){   rect(this.posX, this.posY+17 , 20, 6); rect(this.posX+17, this.posY , 6, 20); circle(this.posX+20, this.posY+20, 6);   }
      else if (cellValue===Tile.NSE){   rect(this.posX+17, this.posY , 6, 40); rect(this.posX+20, this.posY+17 , 20, 6);  }  
      else if (cellValue===Tile.SEW){   rect(this.posX, this.posY+17 , 40, 6); rect(this.posX+17, this.posY+20 , 6, 20);  }  
      else if (cellValue===Tile.NSW){   rect(this.posX+17, this.posY , 6, 40); rect(this.posX, this.posY+17 , 20, 6);  }  
      else if (cellValue===Tile.NEW){   rect(this.posX, this.posY+17 , 40, 6); rect(this.posX+17, this.posY , 6, 20);  } 
      else{   text(this.getNumberOfCellPossibleTiles(), this.posX+15, this.posY+25);   }
   }
   
   collapseCell()
   {
      let collapseCellTile=this.cellPossibleTiles[Math.floor(Math.random()*this.cellPossibleTiles.length)];
      this.cellPossibleTiles = [collapseCellTile];
      let tilesWithN = [Tile.NS, Tile.NE, Tile.NW, Tile.NSE, Tile.NSW, Tile.NEW];
      let tilesWithoutN = [Tile.BLANK, Tile.EW, Tile.SE, Tile.SW, Tile.SEW];
      let tilesWithS = [Tile.NS, Tile.SE, Tile.SW, Tile.NSE, Tile.SEW, Tile.NSW];
      let tilesWithoutS = [Tile.BLANK, Tile.EW, Tile.NE, Tile.NW, Tile.NEW];
      if(this.y>0){ 
         if(tilesWithoutN.includes(collapseCellTile)){  // Tiles without N
            for(let invalidAsNeighbourTile of tilesWithS){  // Tiles with S
               this.remove(boardCell[this.y-1][this.x].cellPossibleTiles, invalidAsNeighbourTile);
            }
         } 
         if(tilesWithN.includes(collapseCellTile)){  // Tiles with N           
            for(let invalidAsNeighbourTile of tilesWithoutS){  // Tiles without S
               this.remove(boardCell[this.y-1][this.x].cellPossibleTiles, invalidAsNeighbourTile);
            }
         }
      }  
      if(this.y<Yspan-1){ 
         if(tilesWithoutS.includes(collapseCellTile)){  // Tiles without S
            for(let invalidAsNeighbourTile of tilesWithN){  // Tiles with N
               this.remove(boardCell[this.y+1][this.x].cellPossibleTiles, invalidAsNeighbourTile);
            }
         } 
         if(tilesWithS.includes(collapseCellTile)){  // Tiles with S
            for(let invalidAsNeighbourTile of tilesWithoutN){  // Tiles without N
               this.remove(boardCell[this.y+1][this.x].cellPossibleTiles, invalidAsNeighbourTile);
            }
         }
      }
      
      let tilesWithE = [Tile.EW, Tile.NE, Tile.SE, Tile.NSE, Tile.SEW, Tile.NEW];
      let tilesWithoutE = [Tile.BLANK, Tile.NS, Tile.SW, Tile.NW, Tile.NSW];
      let tilesWithW = [Tile.EW, Tile.SW, Tile.NW, Tile.SEW, Tile.NSW, Tile.NEW];
      let tilesWithoutW = [Tile.BLANK, Tile.NS, Tile.NE, Tile.SE, Tile.NSE];
      if(this.x>0){
         if(tilesWithoutW.includes(collapseCellTile)){  // Tiles without W
            for(let invalidAsNeighbourTile of tilesWithE){  // Tiles with E
               this.remove(boardCell[this.y][this.x-1].cellPossibleTiles, invalidAsNeighbourTile);
            }
         } 
         if(tilesWithW.includes(collapseCellTile)){  // Tiles with W
            for(let invalidAsNeighbourTile of tilesWithoutE){  // Tiles without E
               this.remove(boardCell[this.y][this.x-1].cellPossibleTiles, invalidAsNeighbourTile);
            }
         }
      }  
      if(this.x<Xspan-1){
         if(tilesWithoutE.includes(collapseCellTile)){  // Tiles without E
            for(let invalidAsNeighbourTile of tilesWithW){  // Tiles with W
               this.remove(boardCell[this.y][this.x+1].cellPossibleTiles, invalidAsNeighbourTile);
            }
         } 
         if(tilesWithE.includes(collapseCellTile)){  // Tiles with E
            for(let invalidAsNeighbourTile of tilesWithoutW){  // Tiles without W
               this.remove(boardCell[this.y][this.x+1].cellPossibleTiles, invalidAsNeighbourTile);
            }
         }      
      }
   }
   
   remove(boardCellPossibleTiles, tileToRemove){
      const index=boardCellPossibleTiles.indexOf(tileToRemove);
         if(index!==-1){   boardCellPossibleTiles.splice(index,1);   return true;   }
         else{   return false;   }
   }
}
