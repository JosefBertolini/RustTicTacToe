use crate::text_io;

pub fn read_move() -> (i32, i32) {

    let mut row: char;
    let mut col: i32;
    let mut converted_row: i32;
    loop {
        text_io::scan!("{},{}\r\n", row, col);
        if row == 'A' || row == 'B' || row == 'C' {
            converted_row = match row {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                _ => 3,
            };
        } else {
            ask_again(1);
            continue;
        }
        if col < 0 || col > 3 {
            ask_again(0);
            continue;
        } else {
            break;
        }
    }
    let converted_col = col - 1;
    (converted_row, converted_col)
}

fn ask_again(which: i32) {
    if which == 1 {
        println!("Invaild row, must be the character A, B, or C");
    } else {
        println!("Invaild column, must be 1, 2, or 3");
    }
}
