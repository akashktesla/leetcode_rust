#![allow(warnings)]

use std::collections::HashSet;
//VALID sudoko
pub fn main(){
    let board:Vec<Vec<char>> = vec![
    vec!['5','3','.','.','7','.','.','.','.'],
    vec!['6','.','.','1','9','5','.','.','.'],
    vec!['.','9','8','.','.','.','.','6','.'],
    vec!['8','.','.','.','6','.','.','.','3'],
    vec!['4','.','.','8','.','3','.','.','1'],
    vec!['7','.','.','.','2','.','.','.','6'],
    vec!['.','6','.','.','.','.','2','8','.'],
    vec!['.','.','.','4','1','9','.','.','5'],
    vec!['.','.','.','.','8','.','.','7','9']];
    let is_s = is_valid_sudoku(board);
    println!("{}",is_s);
}

fn is_valid_sudoku(board:Vec<Vec<char>>)->bool{
    let mut box_set:HashSet<char> = HashSet::new();
    //row
    for i in 0..9{
        let mut row_set:HashSet<char> = HashSet::new();
        for j in 0..9{
            if board[i][j] != '.' {
                if !row_set.insert(board[i][j]){
                    return false
                }
            }
        }
    }
    //col
    for i in 0..9{
        let mut col_set:HashSet<char> = HashSet::new();
        for j in 0..9{
            if board[j][i] != '.' {
                if !col_set.insert(board[j][i]){
                    return false
                }
            }
        }
    }
    //subset
    for i in 0..3{
        let mut sub_set:HashSet<char> = HashSet::new();
        for j in 0..3{
            for k in 3*i..3*i+3{
                for l in 3*j..3*j+3{
                    if board[k][l] != '.'{
                        if !sub_set.insert(board[k][l]){
                            return false
                        }
                    }
                }
            }
        }
    }
    return true;
}
