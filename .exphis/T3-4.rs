
mod method;

fn main(){
    let n = 5;
    let c = 10;
    let d = 14;
    let w = vec![0,1,3,5,2,3]; // 重量
    let b = vec![0,4,5,2,1,4]; // 体积
    let v = vec![0,6,3,2,1,6]; // 价值
    //
    //dp[n+1][c+1][d+1]
    let mut dp1 = vec![vec![vec![0;d+1];c+1];n+1];
    let res1 = method::method::solve1(n,c,d,&mut dp1,&w,&b,&v);
    println!("res1 = {}",res1);
    //
    // dp[c+1][d+1]
    let mut dp2 = vec![vec![0;d+1];c+1];
    let res2 = method::method::solve2(n,c,d,&mut dp2,&w,&b,&v);
    println!("res2 = {}",res2);
}

