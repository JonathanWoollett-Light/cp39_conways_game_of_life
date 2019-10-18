use std::env;
use std::i8;

//Conway's game of life

// Try: `cargo run 0 0 1 0 2 0 2 1 1 2`
fn main() {
    let live_cells_string_coords:Vec<String> = env::args().collect();
    let mut live_cells_tuple_coords:Vec<(i8,i8)> = Vec::with_capacity((live_cells_string_coords.len() / 2) + 1);

    let mut min_y = i8::MAX;
    let mut max_y = i8::MIN;
    let mut min_x = i8::MAX;
    let mut max_x = i8::MIN;

    for i in 0..(live_cells_string_coords.len() / 2) {
        let x: i8 = live_cells_string_coords[(2 * i) + 1].parse().unwrap();
        let y: i8 = live_cells_string_coords[(2 * i) + 2].parse().unwrap();

        if x > max_x { max_x = x; }
        if x < min_x { min_x = x; }
        if y > max_y { max_y = y; }
        if y < min_y { min_y = y; }

        live_cells_tuple_coords.push((x,y));
    }

    //println!("live_cells_string_coords.len(): {}\nlive_cells_tuple_coords.len(): {}",live_cells_string_coords.len(),live_cells_tuple_coords.len());
    println!("live_cells_tuple_coords: {:?}",live_cells_tuple_coords);

    //println!("max_y: {}, min_y: {}", max_y, min_y);
    //println!("max_x: {}, min_x: {}", max_x, min_x);

    let y_diff = max_y - min_y;
    let x_diff = max_x - min_x;

    //println!("y_diff: {}",y_diff);
    //println!("x_diff: {}",x_diff);

    let mut board:Vec<Vec<bool>> = vec![vec![false;((2 * x_diff) + 3) as usize];((2 * y_diff) + 3) as usize];

    print_board(&board,min_x,min_y);

    let y_centre = (board.len() as f32 / 4f32).ceil() as i8;
    let x_centre = (board[0].len() as f32 / 4f32).ceil() as i8;

    //println!("x_centre: {}, y_centre: {}",x_centre,y_centre);

    for cell in live_cells_tuple_coords {
        //println!("({},{}) -> ({},{})",cell.0,cell.1,cell.0 - min_x + x_centre,cell.1 - min_y + y_centre);
        let x_coord = cell.0 - min_x + x_centre;
        let y_coord = cell.1 - min_y + y_centre;
        board[y_coord as usize][x_coord as usize] = true;
    }

    print_board(&board,min_x,min_y);

    run(board,min_x,min_y, 240);
}

fn run(mut board:Vec<Vec<bool>>,mut min_x:i8,mut min_y:i8,iterations:u8) -> (){
    for _ in 0..iterations {
        let mut new_board:Vec<Vec<bool>> = board.clone();
        let x_len = board[0].len();
        let mut expand = (false,false,false,false); // (+x,-x,+y,-y)
        let mut shrink = (true,true,true,true); // (+x,-x,+y,-y)
        for y in 1..board.len() - 1 {
            for x in 1..x_len - 1 {
                let mut living_neighbours = 0u8;
                for i in 0..3 {
                    for t in 0..3 {
                        if i == 1 && t == 1 { continue; }
                        //println!("({},{}) -> ({},{}):{}",x,y,y + i - 1,x + t - 1,board[y + i - 1][x + t - 1]);
                        if board[y + i - 1][x + t - 1] { living_neighbours += 1; }
                    }
                }
                //println!("CURRENT: ({},{}) -> {}",x,y,living_neighbours);
                
                if living_neighbours < 2 || living_neighbours > 3 { new_board[y][x] = false; }
                else if living_neighbours == 3 || (living_neighbours == 2 && board[y][x]) { 
                    new_board[y][x] = true;
                    //println!("({},{}) ITS ALIVE!",x,y);

                    //Checks if board needs to expand on any edge
                    if x == x_len-2 { expand.0 = true; }
                    else if x == 1 { expand.1 = true; }
                    if y == board.len()-2 { expand.2 = true; }
                    else if y == 1 { expand.3 = true; }

                    //Checks if board cannot shrink on any edge
                    if x >= x_len-3 { shrink.0 = false; }
                    else if x <= 2 { shrink.1 = false; }
                    if y >= board.len()-3 { shrink.2 = false; }
                    else if y <= 2 { shrink.3 = false; }
                }
                //println!("---------------------------------");
            }
        }
        //print_board(&new_board,min_x,min_y);
        //Expand or shrink board
        if expand.0 {
            for i in 0..new_board.len() {
                new_board[i].push(false);
            }
        }
        else if shrink.0 {
            for i in 0..new_board.len() {
                new_board[i].pop();
            }
        }
        if expand.1 {
            min_x -= 1;
            for i in 0..new_board.len() {
                new_board[i].insert(0, false);
            }
        }
        else if shrink.1 {
            min_x += 1;
            for i in 0..new_board.len() {
                new_board[i].remove(0);
            }
        }
        if expand.2 {
            new_board.push(vec![false;x_len]);
        }
        else if shrink.2 {
            new_board.pop();
        }
        if expand.3 {
            min_y -= 1;
            new_board.insert(0, vec![false;x_len]);
        }
        else if shrink.3 {
            min_y += 1;
            new_board.remove(0);
        }
        board = new_board;
        print_board(&board,min_x,min_y);
    }
}

fn print_board(board: &Vec<Vec<bool>>,min_x:i8,min_y:i8) -> () {
    let x_len = board[0].len();
    for y in 0..board.len() {
        print!("{} | ",format!("{:0>2}",y as i8 + min_y - (board.len() as f32 / 4f32).ceil() as i8));
        for x in 0..x_len {
            print!("{}  ",if board[y][x] { "X" } else { "." })
        }
        println!();
    }
    println!("    {}","---".repeat(x_len));
    print!("     ");
    for x in 0..x_len {
        print!("{} ",format!("{:0>2}", x as i8 + min_x - (board[0].len() as f32 / 4f32).ceil() as i8))
    }
    println!();
}
