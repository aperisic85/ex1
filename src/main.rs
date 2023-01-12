fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut temp = [[0; 3]; 3];
    for x in 0..3 {
        for y in 0..3 {
            temp[x][y] = matrix[y][x]
        }
    }

    temp
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for x in 0..3 {
        print!("|");
        for y in 0..3 {
            print!(" {} ", matrix[x][y]);
        }
        print!("|");
        println!();
    }
}



fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);
    println!("transposed:");
    pretty_print(&transpose(matrix));
    
}
