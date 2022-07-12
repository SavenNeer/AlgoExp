//main.rs
fn main(){
    let w:[i32;5] = [0,3,2,4,1];//物品的体积
    let v:[i32;5] = [0,3,4,1,2];//物品的价值
    let n:usize = 4;//物品的个数
    let c:usize = 8;//背包最大容量
    //
    println!("物品的个数:{}",n);
    println!("背包最大容量:{}",c);
    println!("物品体积列表:{:?}",&w[1..]);
    println!("物品价值列表:{:?}",&v[1..]);
    //
    let mut dp:[[i32;9];5] = [[0;9];5];
    //初始化
    let mut i:usize = 1;
    let res = loop {
        let mut j:usize = 1;
        loop {
            if j >= (w[i] as usize) {
                let val = dp[i-1][j-(w[i] as usize)]+v[i];
                if dp[i-1][j] < val {
                    dp[i][j] = val;
                }else {
                    dp[i][j] = dp[i-1][j];
                }
            }
            j += 1;
            if j > c { break; }
        }
        i += 1;
        if i > n { break dp[n][c]; }
    };
    println!("\n生成dp规划数组如下:");
    print!(" 容积: ");
    for j in 1..c+1 { print!("{} ",j) }
    println!("");
    for i in 1..n+1 {
        print!("物品{}: ",i);
        for j in 1..c+1 {
            print!("{} ",dp[i][j]);
        }
        print!("\n");
    }
    println!("背包存放的最大价值:{}",res);
    let mut sel:[bool;5] = [false;5];
    let mut i:usize = 4;
    let mut j:usize = c;
    let sta = loop {
        if dp[i][j] == dp[i-1][j] {
            //说明i物品没有放入背包
            sel[i] = false;
        }else if j >= (w[i] as usize) && dp[i][j] == dp[i-1][j-(w[i] as usize)]+v[i] {
            //说明i物品放入了背包
            sel[i] = true;
            j -= w[i] as usize;
        }
        i -= 1;
        if i == 0 { break &sel[1..]; }
    };
    println!("最优解选择策略:{:?}",sta);
}


