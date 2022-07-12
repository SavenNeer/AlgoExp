// main.rs

#[allow(non_snake_case)]
pub struct Traveling {
    N : usize, //图G的顶点数
    x : Vec<usize>, //当前解
    bestx : Vec<usize>, //当前最优解
    a : Vec<Vec<usize>>, //图G的临界矩阵
    cc : usize, //当前费用
    bestc : usize, //当前最优值
    NoEdge : usize, //无边标记
}

#[allow(non_snake_case)]
impl Traveling {
    pub fn Backtrack(&mut self,i : usize) {
        let n = self.N;
        let NoEdge = self.NoEdge;
        if i == n {
            if (self.a[self.x[n-1]][self.x[n]] != NoEdge) && 
                (self.cc+self.a[self.x[n-1]][self.x[n]]+
                self.a[self.x[n]][1]<self.bestc || self.bestc == NoEdge) {
                //更新最优解
                for j in 1..n+1 {
                    self.bestx[j] = self.x[j];
                }
                self.bestc = self.cc+self.a[self.x[n-1]][self.x[n]]+self.a[self.x[n]][1];
            }
        } else {
            for j in i..n+1 {
                //是否可以进入x[j]子树
                if self.a[self.x[i-1]][self.x[j]] != NoEdge && 
                    (self.cc+self.a[self.x[i-1]][self.x[j]]<self.bestc || 
                    self.bestc == NoEdge) {
                    //搜索子树
                    self.x.swap(i,j);
                    self.cc += self.a[self.x[i-1]][self.x[i]];
                    Self::Backtrack(self,i+1);
                    self.cc -= self.a[self.x[i-1]][self.x[i]];
                    self.x.swap(i,j);
                }
            }
        }
    }
}

#[allow(non_snake_case)]
fn TSP(a:Vec<Vec<usize>>,v:Vec<usize>,n:usize,NoEdge:usize) -> usize{
    let mut Y:Traveling = Traveling {
        N : n,
        x : (0..n+1).collect::<Vec<usize>>(),
        a : a,
        bestc : NoEdge,
        bestx : v,
        cc : 0,
        NoEdge : NoEdge,
    };
    Y.Backtrack(2);
    println!("最短距离 : {:?}",Y.bestc);
    println!("路线规划 : {:?}",&Y.bestx[1..]);
    Y.bestc
}

#[allow(non_snake_case)]
fn main(){
    //获取图结点个数
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<usize>().unwrap();
    let NoEdge:usize = 1000000;//无法通行
    //获取图的信息
    let mut a : Vec<Vec<usize>> = vec![vec![NoEdge;n+1]];
    for _ in 0..n {
        line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line = "0 ".to_string() + &line;
        a.push(line.split_whitespace()
                .map(|ch| ch.parse::<usize>().unwrap())
                .collect::<Vec<usize>>());
    }
    //保存最优路径结果
    let v : Vec<usize> = vec![NoEdge;n+1];
    //TSP回溯法实现
    TSP(a,v,n,NoEdge);
}


/*
5
1000000 5 61 34 12
57 1000000 43 20 7
39 42 1000000 8 21
6 50 42 1000000 8
41 26 10 35 1000000
*/




