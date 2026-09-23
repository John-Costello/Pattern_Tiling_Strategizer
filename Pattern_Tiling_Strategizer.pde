import java.util.Arrays;
import java.util.ArrayList;

final int Xspan=12;
final int Yspan=12;
Cell[][] boardCell = new Cell[Yspan][Xspan];

void setup(){
   size(680,680);
   for(int j=0; j<Yspan; j++){
      for(int i=0; i<Xspan; i++){
         boardCell[j][i] = new Cell(i,j);      
      }
   }
}

void draw()
{
   background(192);
   displayBoard();
   update();
}

void displayBoard(){
   pushMatrix();
      translate(100,100);
      for(int j=0; j<Yspan; j++){
         for(int i=0; i<Xspan; i++){
            boardCell[j][i].displayCell();
         }
      }
   popMatrix();
}

void update()
{
   ArrayList<int[]> nextCell = findNextCell();
   if(nextCell.size()==1){
      boardCell[nextCell.get(0)[1]][nextCell.get(0)[0]].collapseCell();
   }
   else
   {
      noLoop();  //  "All cells collapsed!! "
   }
}

ArrayList<int[]> findNextCell(){
   int boardSearchLowestNumberOfPossibleTiles=11;  // Starts at inital number and reduces with board search.
   ArrayList<int[]> listOfCells = new ArrayList<int[]>(Arrays.asList(new int[][]{}));
   for(int j=0; j<Yspan; j++){
      for(int i=0; i<Xspan; i++){
         if(boardCell[j][i].getNumberOfCellPossibleTiles()==boardSearchLowestNumberOfPossibleTiles){  
            listOfCells.add(new int[]{i,j});    }
         else if(boardCell[j][i].getNumberOfCellPossibleTiles()>1  &&  boardCell[j][i].getNumberOfCellPossibleTiles()<boardSearchLowestNumberOfPossibleTiles){
            boardSearchLowestNumberOfPossibleTiles=boardCell[j][i].getNumberOfCellPossibleTiles(); 
            listOfCells = new ArrayList<int[]>(Arrays.asList(new int[][]{}));
            listOfCells.add(new int[]{i,j});
         }
      }
   }
   if(listOfCells.size()==0){   return listOfCells;   }
   else{
      return( new ArrayList<int[]>( Arrays.asList(new int[][]{   listOfCells.get( (int)random(listOfCells.size()) )   })) );   
   }
}

enum Tile{PENDING, BLANK, NS, EW, NE, NW, SE, SW, NSE, NSW, NEW, SEW};

class Cell
{
   private int x;
   private int y;
   private int posX;
   private int posY;
   private ArrayList<Tile> cellPossibleTiles  = new ArrayList<Tile>(Arrays.asList(new Tile[]{Tile.BLANK, Tile.NS, Tile.EW, Tile.NE, Tile.SE, Tile.SW, Tile.NW, Tile.NSE, Tile.NSW, Tile.NEW, Tile.SEW }));
   
   public Cell(int i, int j){
       x=i;
       y=j;
       posX=i*40;
       posY=j*40;
   }
   
   public Tile getCellValue(){
      if(this.cellPossibleTiles.size()==1){   return(this.cellPossibleTiles.get(0));   }
      else{   return(Tile.PENDING);   }
   }
   
   public int getNumberOfCellPossibleTiles(){
      return cellPossibleTiles.size();
   }
   
   public void displayCell(){
      strokeWeight(1);
      stroke(64); 
      fill(0);
      rect(posX, posY, 40, 40);
      stroke(255);
      fill(255);
      Tile cellValue=getCellValue();
      if (cellValue==Tile.BLANK) {    }  
      else if (cellValue==Tile.NS){   rect(posX+17, posY , 6, 40);   }  
      else if (cellValue==Tile.EW){   rect(posX, posY+17 , 40, 6);   }  
      else if (cellValue==Tile.NE){   rect(posX+17, posY , 6, 20); rect(posX+20, posY+17 , 20, 6); circle(posX+20, posY+20, 6);   }  
      else if (cellValue==Tile.SE){   rect(posX+20, posY+17 , 20, 6); rect(posX+17, posY+20 , 6, 20); circle(posX+20, posY+20, 6);  }     
      else if (cellValue==Tile.SW){   rect(posX+17, posY+20 , 6, 20); rect(posX, posY+17 , 20, 6); circle(posX+20, posY+20, 6);   }
      else if (cellValue==Tile.NW){   rect(posX, posY+17 , 20, 6); rect(posX+17, posY , 6, 20); circle(posX+20, posY+20, 6);   }
      else if (cellValue==Tile.NSE){   rect(posX+17, posY , 6, 40); rect(posX+20, posY+17 , 20, 6);  }  
      else if (cellValue==Tile.SEW){   rect(posX, posY+17 , 40, 6); rect(posX+17, posY+20 , 6, 20);  }  
      else if (cellValue==Tile.NSW){   rect(posX+17, posY , 6, 40); rect(posX, posY+17 , 20, 6);  }  
      else if (cellValue==Tile.NEW){   rect(posX, posY+17 , 40, 6); rect(posX+17, posY , 6, 20);  } 
      else{   text(this.getNumberOfCellPossibleTiles(), posX+15, posY+25);   }
   }

   public void collapseCell()
   {
      Tile collapseCellTile=this.cellPossibleTiles.get((int)random(cellPossibleTiles.size()));
      this.cellPossibleTiles = new ArrayList<Tile>(Arrays.asList(new Tile[]{collapseCellTile}));
      ArrayList<Tile> tilesWithN = new ArrayList<Tile>(Arrays.asList(new Tile[]{Tile.NS, Tile.NE, Tile.NW, Tile.NSE, Tile.NSW, Tile.NEW}));
      ArrayList<Tile> tilesWithoutN = new ArrayList<Tile>(Arrays.asList(new Tile[]{Tile.BLANK, Tile.EW, Tile.SE, Tile.SW, Tile.SEW}));
      ArrayList<Tile> tilesWithS = new ArrayList<Tile>(Arrays.asList(new Tile[]{Tile.NS, Tile.SE, Tile.SW, Tile.NSE, Tile.SEW, Tile.NSW}));
      ArrayList<Tile> tilesWithoutS = new ArrayList<Tile>(Arrays.asList(new Tile[]{Tile.BLANK, Tile.EW, Tile.NE, Tile.NW, Tile.NEW}));
      if(y>0){
         if(tilesWithoutN.contains(collapseCellTile)){  // Tiles without N
            for(Tile invalidAsNeighbourTile: tilesWithS){  // Tiles with S
               boardCell[y-1][x].cellPossibleTiles.remove(invalidAsNeighbourTile);
            }
         } 
         if(tilesWithN.contains(collapseCellTile)){  // Tiles with N
            for(Tile invalidAsNeighbourTile: tilesWithoutS){  // Tiles without S
               boardCell[y-1][x].cellPossibleTiles.remove(invalidAsNeighbourTile);
            }
         }
      }  
      if(y<Yspan-1){
         if(tilesWithoutS.contains(collapseCellTile)){  // Tiles without S
            for(Tile invalidAsNeighbourTile: tilesWithN){  // Tiles with N
               boardCell[y+1][x].cellPossibleTiles.remove(invalidAsNeighbourTile);
            }
         } 
         if(tilesWithS.contains(collapseCellTile)){  // Tiles with S
            for(Tile invalidAsNeighbourTile: tilesWithoutN){  // Tiles without N
               boardCell[y+1][x].cellPossibleTiles.remove(invalidAsNeighbourTile);
            }
         }
      }
      
      ArrayList<Tile> tilesWithE = new ArrayList<Tile>(Arrays.asList(new Tile[]{Tile.EW, Tile.NE, Tile.SE, Tile.NSE, Tile.SEW, Tile.NEW}));
      ArrayList<Tile> tilesWithoutE = new ArrayList<Tile>(Arrays.asList(new Tile[]{Tile.BLANK, Tile.NS, Tile.SW, Tile.NW, Tile.NSW}));
      ArrayList<Tile> tilesWithW = new ArrayList<Tile>(Arrays.asList(new Tile[]{Tile.EW, Tile.SW, Tile.NW, Tile.SEW, Tile.NSW, Tile.NEW}));
      ArrayList<Tile> tilesWithoutW = new ArrayList<Tile>(Arrays.asList(new Tile[]{Tile.BLANK, Tile.NS, Tile.NE, Tile.SE, Tile.NSE}));
      if(x>0){
         if(tilesWithoutW.contains(collapseCellTile)){  // Tiles without W
            for(Tile invalidAsNeighbourTile: tilesWithE){  // Tiles with E
               boardCell[y][x-1].cellPossibleTiles.remove(invalidAsNeighbourTile);
            }
         } 
         if(tilesWithW.contains(collapseCellTile)){  // Tiles with W
            for(Tile invalidAsNeighbourTile: tilesWithoutE){  // Tiles without E
               boardCell[y][x-1].cellPossibleTiles.remove(invalidAsNeighbourTile);
            }
         }
      }  
      if(x<Xspan-1){
         if(tilesWithoutE.contains(collapseCellTile)){  // Tiles without E
            for(Tile invalidAsNeighbourTile: tilesWithW){  // Tiles with W
               boardCell[y][x+1].cellPossibleTiles.remove(invalidAsNeighbourTile);
            }
         } 
         if(tilesWithE.contains(collapseCellTile)){  // Tiles with E
            for(Tile invalidAsNeighbourTile: tilesWithoutW){  // Tiles without W
               boardCell[y][x+1].cellPossibleTiles.remove(invalidAsNeighbourTile);
            }
         }  
      }
   }
}
