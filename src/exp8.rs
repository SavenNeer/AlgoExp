
//main.rs

fn solve(ans:&mut usize,n:usize,m:usize,curnode:usize,col:&mut Vec<usize>,map:&Vec<Vec<bool>>) -> i32 {
    //递归生成排列树
    if curnode > n {
        *ans = *ans + 1;
        //输出所有的方案
        println!("方案{}:{:?}",ans,&col[1..]);
        return 0;
    } else {
        //选择颜色
        for color in 1..m+1 {
            col[curnode] = color;
            //合法性检查
            if check(n,curnode,map,col) {
                solve(ans,n,m,curnode+1,col,map);
            }
            col[curnode] = 0;
        }
    }
    return -1;
}

//染色合法性检测
fn check(n:usize,node:usize,map:&Vec<Vec<bool>>,col:&mut Vec<usize>) -> bool {
    for u in 1..n+1 {
        if u == node { continue; }
        if map[node][u] && col[node] == col[u] {
            return false;
        }
    }
    true
}

//主函数
fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap(); // 输入
    let input = line.split_whitespace().map(|ch| ch.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let n = input[0]; //顶点数量
    let k = input[1]; //边数量
    let m = input[2]; //颜色数量 [1..m]
    //图存储
    let mut map:Vec<Vec<bool>> = vec![vec![false;n+1];n+1];
    //各个点的颜色 0表示没有染色
    let mut col:Vec<usize> = vec![0;n+1];
    //添加边的闭包函数
    let mut addfunc = |u:usize,v:usize| {
        map[u][v] = true;
        map[v][u] = true;
    };
    //输入图的拓扑结构
    for _ in 0..k {
        //读取一行数据并解析
        line.clear();
        std::io::stdin().read_line(&mut line).unwrap();
        let lis = line.split_whitespace().map(|ch| ch.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        //添加网络结点
        addfunc(lis[0],lis[1]);
    }
    //搜索
    let mut ans = 0;
    solve(&mut ans, n, m, 1, &mut col, &map);
    //输出结果
    println!("ans = {}",ans);
}


