use rand::thread_rng;
use std::io;
enum Cell{
    Mine, 
    Empty(u8),
    Hidden, 
    Flagged,  
}
struct MineSweeper{
    width:usize, 
    height:usize, 
    board:Vec<Vec<Cell>>;
    revealed:Vec<Vec<bool>>;
};
impl MineSweeper{
    fn new(width, height, mines)->Self{
        let mut game={
            width, 
            height, 
            mines,
            board: vec![vec![Cell::Hidden; width]; height];
            revealed:vec![vec![false; width]; height];
        }
        game.place_mines();
        game.calculate_numbers();
        game
    }
    fn place_mines(&mut self){
        let mut rng=rand::thread_rng();
        let mut total_cells=self.width*self.height;
        let board_size=self.width*self.height;
        let mut marked=!vec[false;total_cells];
        for _ in 0..total_cells{
            let idx=rng.gen_range(0..total_cells);
            let mut count=0;
            for i in 0..board_size{
                if !marked[i] && count<idx{
                    count+=1;
                }
                else if !marked[i]{
                    let x=i/self.width;
                    let y=i%self.width;
                    self.board[x][y]=Cell::Mine;
                    marked[x][y]=true;
                }
            }
            total_cells-=1;
        }
    }
    fn calculate_numbers(&mut self){

    }
}

fn get_input(prompt:&str)->usize{
    loop{
        println!("{}", prompt);
        let mut input=String::new();
        io::stdin().readline(&mut input).strip();
        match input.trim().parse::usize{
            OK(num)=>return num,
            Err(_)=>println!("Invalid input, please enter a number! "),
        }
    }
}

fn main(){
    println!("Welcome to the Mine Sweeper");
    let height=get_input("Enter board height");
    let width=get_input("Enter board width");
    
}