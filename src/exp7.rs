
fn backtrack(k:usize,n:usize,c:usize,
    w:&Vec<i32>,v:&Vec<i32>,x:&mut Vec<i32>,
    cw:&mut i32,cv:&mut i32,bestp:&mut i32,
    bestx:&mut Vec<i32>
) {
    if k > n {
        if cv > bestp {
            //更新最优解
            *bestp = *cv;
            for i in 1..n+1 {
                bestx[i] = x[i];
            }
        }
    }else {
        //装入与不装入
        for i in 0..2 {
            x[k] = i;
            if i == 0 {
                //不装入
                backtrack(k+1,n,c,&w,&v,x, cw, cv,bestp,bestx);
            }else {
                //判断能否装入
                if *cw + w[k] <= c as i32 {
                    //记录相关状态
                    *cw += w[k];
                    *cv += v[k];
                    backtrack(k+1,n,c,w,v,x,cw,cv,bestp,bestx);
                    *cw -= w[k];
                    *cv -= v[k];
                }
            }
        }
    }
}

fn main() {
    let c = 14;
    println!("背包最大容量:{}",c);
    let mut cw = 0;
    let mut cv = 0;
    let mut bestp = 0;
    let n:usize = 5;
    println!("物品个数:{}",n);
    let w:Vec<i32> = vec![0,3,2,5,7,4];
    println!("物品重量:{:?}",&w[1..]);
    let v:Vec<i32> = vec![0,3,9,2,8,4];
    println!("物品重量:{:?}",&v[1..]);
    let mut x:Vec<i32> = vec![0;n+1];
    let mut bestx:Vec<i32> = vec![0;n+1];
    //回溯搜索
    backtrack(1,n,c,&w,&v,
            &mut x,&mut cw,&mut cv,
            &mut bestp,&mut bestx);
    println!("最大可装入价值 = {}",bestp);
    println!("物品选择:{:?}",&bestx[1..]);
}

