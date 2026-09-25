#include "cinder/app/App.h"
#include "cinder/app/RendererGl.h"
#include "cinder/gl/gl.h"
#include <vector>
#include <random>
#include <algorithm>

std::random_device rd;
std::mt19937 gen(rd());
using namespace ci;
using namespace ci::app;
using namespace std;
const int WINDOW_WIDTH = 680;
const int WINDOW_HEIGHT = 680;
const int Xspan=12;
const int Yspan=12;
class Cell;
extern Cell*** boardCell;
enum Tile { PENDING, BLANK, NS, EW, NE, NW, SE, SW, NSE, NSW, NEW, SEW };

class Pattern_Tiling_StrategizerApp : public App {
public:
    void setup() override;
    void draw() override;
    void update() override;
    void Pattern_Tiling_StrategizerApp::displayBoard();
    vector<ci::ivec2> findNextCell();
    void mouseDown(MouseEvent event) override;
};

class Cell
{
   private:
      int x;
      int y;
      int posX;
      int posY;
      vector<Tile> cellPossibleTiles = { BLANK, NS, EW, NE, SE, SW, NW, NSE, NSW, NEW, SEW };

   public:
      Cell::Cell(int i, int j) {
         x = i;
         y = j;
         posX = i * 40;
         posY = j * 40;
      }

   private: 
      Tile getCellValue() {
         if (cellPossibleTiles.size() == 1) { return(cellPossibleTiles.at(0)); }
         else { return(PENDING); }
       }

   public: 
      const int getNumberOfCellPossibleTiles() {
         return cellPossibleTiles.size();
      }

      void displayCell() {
         gl::lineWidth(1);
         gl::color(0.25f, 0.25f, 0.25f);
         gl::drawSolidRect(Rectf(posX, posY, posX + 41, posY + 41));
         gl::color(0.f, 0.f, 0.f);
         gl::drawSolidRect(Rectf(posX+1, posY+1, posX + 40, posY + 40));
         gl::color(1.0f, 1.0f, 1.0f);
         Tile cellValue = getCellValue();
         if (cellValue == BLANK) {}
         else if (cellValue == NS) { gl::drawSolidRect(Rectf(posX + 17, posY, posX+23, posY+40)); }
         else if (cellValue == EW) { gl::drawSolidRect(Rectf(posX, posY + 17, posX + 40, posY+23)); }
         else if (cellValue == NE) { gl::drawSolidRect(Rectf(posX + 17, posY, posX+23, posY+20)); gl::drawSolidRect(Rectf(posX + 20, posY + 17, posX + 40, posY + 23)); gl::drawSolidCircle(vec2(posX + 20, posY + 20), 3); }
         else if (cellValue == SE) { gl::drawSolidRect(Rectf(posX + 20, posY + 17, posX + 40, posY + 23)); gl::drawSolidRect(Rectf(posX + 17, posY + 20, posX + 23, posY + 40)); gl::drawSolidCircle(vec2(posX + 20, posY + 20), 3); }
         else if (cellValue == SW) { gl::drawSolidRect(Rectf(posX + 17, posY + 20, posX + 23, posY + 40)); gl::drawSolidRect(Rectf(posX, posY + 17, posX + 20, posY + 23)); gl::drawSolidCircle(vec2(posX + 20, posY + 20), 3); }
         else if (cellValue == NW) { gl::drawSolidRect(Rectf(posX, posY + 17, posX + 20, posY + 23)); gl::drawSolidRect(Rectf(posX + 17, posY, posX + 23, posY+20)); gl::drawSolidCircle(vec2(posX + 20, posY + 20), 3); }
         else if (cellValue == NSE) { gl::drawSolidRect(Rectf(posX + 17, posY, posX + 23, posY + 40)); gl::drawSolidRect(Rectf(posX + 20, posY + 17, posX + 40, posY +23)); }
         else if (cellValue == SEW) { gl::drawSolidRect(Rectf(posX, posY + 17, posX + 40, posY + 23)); gl::drawSolidRect(Rectf(posX + 17, posY + 20, posX + 23, posY + 40)); }
         else if (cellValue == NSW) { gl::drawSolidRect(Rectf(posX + 17, posY, posX + 23, posY + 40)); gl::drawSolidRect(Rectf(posX, posY + 17, posX + 20, posY + 23)); }
         else if (cellValue == NEW) { gl::drawSolidRect(Rectf(posX, posY + 17, posX + 40, posY + 23)); gl::drawSolidRect(Rectf(posX + 17, posY, posX + 23, posY + 20)); }
         else {   gl::drawString(std::to_string(getNumberOfCellPossibleTiles()),vec2(posX+10, posY+15), ci::Color::white(), ci::Font("Arial", 17));   }
      }

   private:
      bool contains(vector<Tile> vectorOfTiles, Tile tile)
      {
         for (int index = 0; index < vectorOfTiles.size(); index++)
         {
            if (vectorOfTiles.at(index) == tile) { return true; }
         }
         return false;
      }

      bool remove(vector<Tile> & boardCellPossibleTiles, Tile tileToRemove)
	  {
		 std::vector<Tile>::iterator it = std::find(boardCellPossibleTiles.begin(), boardCellPossibleTiles.end(), tileToRemove);
         if (it != boardCellPossibleTiles.end()) { boardCellPossibleTiles.erase(it); return true; }
         else { return false; }
	  }

   public:
      void collapseCell()
      {  
         int N = cellPossibleTiles.size();
          std::uniform_int_distribution<int> distrib(0, N - 1);
          int random_num = distrib(gen);
          Tile collapseCellTile = cellPossibleTiles.at(random_num);
          cellPossibleTiles = { collapseCellTile };

          vector<Tile> tilesWithN = { NS, NE, NW, NSE, NSW, NEW };
          vector<Tile> tilesWithoutN = { BLANK, EW, SE, SW, SEW };
          vector<Tile> tilesWithS = { NS, SE, SW, NSE, SEW, NSW };
          vector<Tile> tilesWithoutS = { BLANK, EW, NE, NW, NEW };
          if (y > 0) {
             if (Cell::contains(tilesWithoutN, collapseCellTile)) {  // Tiles without N
                for (Tile invalidAsNeighbourTile : tilesWithS) {  // Tiles with S
			 		Cell::remove(boardCell[y - 1][x]->cellPossibleTiles, invalidAsNeighbourTile);
                }				
            }

            if (Cell::contains(tilesWithN, collapseCellTile)) {  // Tiles with N
                for (Tile invalidAsNeighbourTile : tilesWithoutS) {  // Tiles without S
                    Cell::remove(boardCell[y - 1][x]->cellPossibleTiles, invalidAsNeighbourTile);
                }
            }
      }

      if (y < Yspan - 1) {
         if(Cell::contains(tilesWithoutS, collapseCellTile)) {  // Tiles without S
            for(Tile invalidAsNeighbourTile : tilesWithN) {  // Tiles with N
               Cell::remove(boardCell[y + 1][x]->cellPossibleTiles, invalidAsNeighbourTile);
            }
         }

         if(Cell::contains(tilesWithS, collapseCellTile)) {  // Tiles with S
            for(Tile invalidAsNeighbourTile : tilesWithoutN) {  // Tiles without N
               Cell::remove(boardCell[y + 1][x]->cellPossibleTiles, invalidAsNeighbourTile);
            }
         }
      }             
      vector<Tile>  tilesWithE = { EW, NE, SE, NSE, SEW, NEW };
      vector<Tile> tilesWithoutE = { BLANK, NS, SW, NW, NSW };
      vector<Tile> tilesWithW = { EW, SW, NW, SEW, NSW, NEW };
      vector<Tile> tilesWithoutW = { BLANK, NS, NE, SE, NSE };
      if (x > 0) {
         if (Cell::contains(tilesWithoutW, collapseCellTile)) {  // Tiles without W
            for (Tile invalidAsNeighbourTile : tilesWithE) {  // Tiles with E
               Cell::remove(boardCell[y][x-1]->cellPossibleTiles, invalidAsNeighbourTile);
            }
         }

         if (Cell::contains(tilesWithW, collapseCellTile)) {  // Tiles with W
            for (Tile invalidAsNeighbourTile : tilesWithoutE) {  // Tiles without E
                Cell::remove(boardCell[y][x - 1]->cellPossibleTiles, invalidAsNeighbourTile);
            }
         }
      }

      if (x < Xspan - 1) {
         if (Cell::contains(tilesWithoutE, collapseCellTile)) {  // Tiles without E
            for (Tile invalidAsNeighbourTile : tilesWithW) {  // Tiles with W
               Cell::remove(boardCell[y][x + 1]->cellPossibleTiles, invalidAsNeighbourTile);
            }
         }

         if (Cell::contains(tilesWithE, collapseCellTile)) {  // Tiles with E
            for (Tile invalidAsNeighbourTile : tilesWithoutW) {  // Tiles without W
               Cell::remove(boardCell[y][x + 1]->cellPossibleTiles, invalidAsNeighbourTile);
            }
         }
      }
   }
};

Cell*** boardCell = new Cell** [Yspan];

void Pattern_Tiling_StrategizerApp::setup()
{
    for (int j = 0; j < Yspan; j++) {
        boardCell[j] = new Cell * [Xspan];
        for (int i = 0; i < Xspan; i++) {
            boardCell[j][i] = new Cell(i, j);
        }
    }
}

void Pattern_Tiling_StrategizerApp::draw()
{
    gl::clear(Color(0.75f, 0.75f, 0.75f));
	displayBoard();
}

void Pattern_Tiling_StrategizerApp::update()
{
	vector<ci::ivec2> nextCell=findNextCell();
	if(nextCell.size() == 1) {
		boardCell[nextCell.at(0)[1]][nextCell.at(0)[0]]->collapseCell();
	}
	else
	{
       
	}
}

void Pattern_Tiling_StrategizerApp::displayBoard(){
	gl::pushModelMatrix();
	   gl::translate(100,100);
	   for(int j=0; j<Yspan; j++){
          for(int i=0; i<Xspan; i++){
             boardCell[j][i]->displayCell();
          }
       }
	gl::popModelMatrix();
}

void Pattern_Tiling_StrategizerApp::mouseDown(MouseEvent event)
{
}

vector<ci::ivec2> Pattern_Tiling_StrategizerApp::findNextCell(){
   int boardSearchLowestNumberOfPossibleTiles=11;  // Starts at inital number and reduces with board search.
   vector<ci::ivec2> listOfCells = {};
   for(int j=0; j<Yspan; j++){
      for(int i=0; i<Xspan; i++){
         if (boardCell[j][i]->getNumberOfCellPossibleTiles() == boardSearchLowestNumberOfPossibleTiles) {
            listOfCells.push_back(   {i,j}   );
         }
         else if(boardCell[j][i]->getNumberOfCellPossibleTiles()>1  &&  boardCell[j][i]->getNumberOfCellPossibleTiles()<boardSearchLowestNumberOfPossibleTiles){
            boardSearchLowestNumberOfPossibleTiles=boardCell[j][i]->getNumberOfCellPossibleTiles(); 
            listOfCells = {};
            listOfCells.push_back(   {i,j}   );
         }
      }
   }
   if(listOfCells.size()==0){   return listOfCells;   }
   else{
	  int N=listOfCells.size();
	  std::uniform_int_distribution<int> distrib(0, N-1);
	  int random_num = distrib(gen);
      return { listOfCells.at(random_num) } ;
   }
}


CINDER_APP( Pattern_Tiling_StrategizerApp, RendererGl, [](App::Settings *settings){
   settings->setTitle("Pattern Tiling Strategizer");
   settings->setWindowSize(WINDOW_WIDTH, WINDOW_HEIGHT);
})
//----------------------------------------------------------
