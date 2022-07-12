//main.rs
pub use std::sync::atomic::AtomicU64;
pub use std::sync::atomic::Ordering;
//覆盖计数
static TILE: AtomicU64 = AtomicU64::new(1);
//分治法棋盘覆盖
fn chess_board(board:&mut [[i32;50];50],tr:i32,tc:i32,dr:i32,dc:i32,size:i32) -> i32{
    //tr:棋盘左上角方格的行号  tc:棋盘左上角方格列号
    //dr:特殊方格所在的行号    dc:特殊方格所在的列号
    //size:棋盘的行数或列数
    if size == 1 { return 0; }
    //获取TILE的值
    let t = TILE.load(Ordering::SeqCst) as i32;
    //TILE++
    TILE.fetch_add(1,Ordering::SeqCst);
    let s = size / 2;
    //覆盖左上象限
    if dr < tr + s && dc < tc + s {
        chess_board(board,tr, tc, dr, dc, s);
    }else {
        board[(tr+s-1) as usize][(tc+s-1) as usize] = t;
        chess_board(board,tr, tc, tr+s-1, tc+s-1, s);
    }
    //覆盖右上象限
    if dr < tr + s && dc >= tc + s {
        chess_board(board,tr, tc+s, dr, dc, s);
    }else {
        board[(tr+s-1) as usize][(tc+s) as usize] = t;
        chess_board(board,tr, tc+s, tr+s-1, tc+s, s);
    }
    //覆盖左下象限
    if dr >= tr + s && dc < tc + s {
        chess_board(board,tr+s, tc, dr, dc, s);
    }else {
        board[(tr+s) as usize][(tc+s-1) as usize] = t;
        chess_board(board,tr+s, tc, tr+s, tc+s-1, s);
    }
    //覆盖右下象限
    if dr >= tr + s && dc >= tc + s {
        chess_board(board,tr+s, tc+s, dr, dc, s);
    }else {
        board[(tr+s) as usize][(tc+s) as usize] = t;
        chess_board(board,tr+s, tc+s, tr+s, tc+s, s);
    }
    return 0;
}
//输出棋盘布局
fn output_board(&mut board:&mut [[i32;50];50],size:i32) {
    for i in 0..size {
        for j in 0..size {
            print!("{} ",board[i as usize][j as usize]);
        }
        println!("");
    }
}
//读取参数
fn readin(size:&mut i32,dr:&mut i32,dc:&mut i32) {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let lis:Vec<&str> = line.split_whitespace().collect();
    *size = lis.get(0).unwrap().parse::<i32>().unwrap();
    *dr = lis.get(1).unwrap().parse::<i32>().unwrap();
    *dc = lis.get(2).unwrap().parse::<i32>().unwrap();
}
//main()
fn main() {
    let mut board:[[i32;50];50] = [[0;50];50];
    let mut size = 0;
    //特殊方格所在的位置
    let mut dr = 0;
    let mut dc = 0;
    //读入输入值
    readin(&mut size,&mut dr,&mut dc);
    // println!("{} {} {}",size,dr,dc);
    chess_board(&mut board,0, 0, dr, dc, size);
    output_board(&mut board,size);
}

