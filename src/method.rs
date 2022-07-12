
pub mod method {

    pub fn solve1(n:usize,c:usize,d:usize
        ,dp:&mut Vec<Vec<Vec<i32>>>
        ,w:&Vec<usize>,b:&Vec<usize>,v:&Vec<i32>) -> i32 {
        //dp [1,n]
        for i in 1..n+1 {
            for j in 1..c+1 {
                for k in 1..d+1 {
                    let mut val = 0;
                    if j >= w[i] && k >= b[i] {
                        val = dp[i-1][j-w[i]][k-b[i]]+v[i];
                    }
                    dp[i][j][k] = i32::max(dp[i-1][j][k],val);
                }
            }
        }
        return dp[n][c][d];
    }

    pub fn solve2(n:usize,c:usize,d:usize
        ,dp:&mut Vec<Vec<i32>>
        ,w:&Vec<usize>,b:&Vec<usize>,v:&Vec<i32>) -> i32 {
        //dp [1,n]
        for i in 1..n+1 {
            for j in (0..c+1).rev() { // c -> 0
                for k in (0..d+1).rev() { // d -> 0
                    if j >= w[i] && k >= b[i] {
                        dp[j][k] = i32::max(
                            dp[j][k],
                            dp[j-w[i]][k-b[i]]+v[i]
                        );
                    }
                }
            }
        }
        return dp[c][d];
    }

}
