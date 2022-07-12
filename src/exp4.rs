//main.rs

#[allow(unused_must_use)]
#[allow(non_snake_case)]
#[allow(dead_code)]

fn task_schedule(f:&mut Vec<Vec<u32>>,a:&Vec<u32>,b:&Vec<u32>,n:usize) -> u32 {
    let mut sumA:usize = a[1] as usize;
    for x in 0..a[1] {
        f[1][x as usize] = b[1];
    }
    f[1][a[1] as usize] = match a[1] < b[1] {
        true => 0,
        false => b[1],
    };
    //初始化
    for i in 2..n+1 {
        for j in 0..n+1 {
            f[i][j] = u32::MAX;
        }
    }
    //更新
    for k in 2..n+1 {
        sumA += a[k] as usize;
        for x in 0..sumA+1 {
            f[k][x] = match x < a[k] as usize {
                true => f[k-1][x] + b[k],
                false => match f[k-1][x]+b[k] < f[k-1][x-(a[k] as usize)] {
                    true => f[k-1][x]+b[k],
                    false => f[k-1][x-(a[k] as usize)],
                }
            };
        }
    }
    let mut min = u32::MAX;
    for i in 0..sumA+1 {
        min = u32::min(min,u32::max(f[n][i],i as u32));
    }
    min
}
//读取不同机器上的加工时间
fn vec_readin() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    (String::from("0 ") + &line)
                .trim()
                .split_ascii_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
}
//主函数
fn main(){
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    //读取物件总数量
    let n = line.trim().parse::<usize>().unwrap();
    //读取加工时间
    let a:Vec<u32> = vec_readin();
    let b:Vec<u32> = vec_readin();
    println!("a = {:?}",a);
    println!("b = {:?}",b);
    let mut f:Vec<Vec<u32>> = vec![vec![0;32];7];
    let res = task_schedule(&mut f,&a,&b,n);
    println!("{:?}",f);
    println!("最短加工时间:{}",res);
}

// [
//     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [11, 11, 8, 8, 8, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [15, 15, 12, 12, 12, 7, 7, 4, 4, 4, 4, 4, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [26, 26, 23, 23, 23, 18, 18, 15, 15, 15, 15, 15, 12, 12, 11, 7, 7, 4, 4, 4, 4, 4, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [29, 29, 26, 26, 26, 21, 21, 18, 18, 18, 18, 18, 15, 15, 14, 10, 10, 7, 7, 7, 7, 7, 4, 4, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [33, 33, 29, 29, 26, 25, 25, 21, 21, 18, 18, 18, 18, 18, 15, 14, 14, 10, 10, 7, 7, 7, 7, 7, 4, 4, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// ]


