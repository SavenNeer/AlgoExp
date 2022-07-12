use std::io;

#[allow(non_snake_case)]
fn MinRefuelingTimes(oil: &Vec<i32>,M:i32) -> Option<i32> {
    for i in oil {
        if *i > M {
            return None;
        }
    }
    let mut sum = 0;
    let mut cnt = 0;
    let lenth = oil.len();
    for index in 0..lenth {
        sum += oil[index];
        if sum > M {
            cnt += 1;
            sum = oil[index];
        }
    }
    Some(cnt)
}

fn readin(strs:Option<&str>) -> i32{
    let mut buffer = String::new();
    if let Some(content) = strs {
        println!("{}",content);
    }
    io::stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim().to_string();
    buffer.parse::<i32>().unwrap()
}

#[allow(non_snake_case)]
#[allow(unused_variables)]
fn main() {
    let n = readin(Some("加油站总数:"));
    let M = readin(Some("满油最大行车距离:"));
    println!("请输入各个加油站之间的距离:");
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line = line.trim().to_string();
    let lis:Vec<i32> = line.split_ascii_whitespace()
                            .map(|ch| ch.parse::<i32>().unwrap())
                            .collect();
    match MinRefuelingTimes(&lis, M) {
        Some(cnt) => println!("最少加油次数:{}",cnt),
        None => println!("无法到达"),
    }
}

