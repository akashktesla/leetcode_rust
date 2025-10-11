#![allow(warnings)]
//imports

pub fn main(){
     let grid = vec![vec![3,1,1],vec![2,5,1],vec![1,5,5],vec![2,1,1]];
     let max = cherry_pick(grid);
     println!("Max cherries: {}",max);
         
}

fn cherry_pick(grid:Vec<Vec<i32>>)->i32{
    let row_len = grid.len();
    let col_len = grid[0].len();
    // println!("row_len: {}, col_len: {}",row_len,col_len);
    let row = 0;
    let mut col1 = 0;
    let mut col2 = col_len-1;
    let mut memory:Vec<Vec<Vec<i32>>> = vec![vec![vec![-1;col_len];col_len];row_len];
    return pick_cherry(&grid,&mut memory,&row,&col1,&col2,&row_len,&col_len);
}

fn pick_cherry(grid:&Vec<Vec<i32>>,memory:&mut Vec<Vec<Vec<i32>>>,row:&usize,col1:&usize,col2:&usize,row_len:&usize,col_len:&usize)->i32{
    let len = grid.len();
    let mut cost = 0;
    let row = *row;
    let col1 = *col1; 
    let col2 = *col2;
    let col_len = *col_len;
    let row_len = *row_len;
    println!("row: {} col1: {} col2: {}",row,col1,col2);
    let cache =  memory[row][col1][col2]; 
    if cache >= 0{
            return cache ; 
    }

    if col1!=col2{
        cost = grid[row][col1] + grid[row][col2]
    }

    else{
        cost = grid[row][col1]
    }

    let mut val1 = 0 ;
    let mut val2 = 0 ;
    let mut val3 = 0 ;

    let mut best = 0;
    let trow = row+1;
    for i in [1,0,-1]{
        for j in [1,0,-1]{
            let tcol1 = (col1 as i32 +i) as usize;
            let tcol2 = (col2 as i32 +j) as usize;
            if trow >= 0 && tcol1 >= 0 && tcol2 >= 0 && trow < row_len && tcol1 < col_len && tcol2 < col_len{
                let val = pick_cherry(grid,memory,&trow,&tcol1,&tcol2,&row_len,&col_len);
                best = best.max(val);
            }
        }
    }

    let total = cost + best;
    memory[row][col1][col2] = total;
    return total;
}




