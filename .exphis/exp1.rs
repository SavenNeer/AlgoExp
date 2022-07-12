//main.rs
//循环二分搜索
fn search1<E:PartialOrd>(data: &Vec<E>,target: E) -> i32{
    //区间初始化
    let mut l: i32 = 0;
    let mut r = data.len() as i32 - 1;
    //二分搜索
    while l <= r {
        //选取中点
        let mid = (l + r) / 2;
        //比较中点
        if data[mid as usize] == target{
            return mid;
        }
        //区间二分
        if data[mid as usize] < target {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }
    return -1;
}

//递归二分搜索
fn recursion_search<E:PartialOrd>(data:&Vec<E>,l:i32,r:i32,target:E) -> i32{
    if l > r { return -1; }
    //区间中点判断
    let mid = l + (r - l) / 2;
    if data[mid as usize] == target {
        return mid;
    }
    //区间二分
    match data[mid as usize] < target {
        true => recursion_search(data, mid+1, r, target),
        false => recursion_search(data, l, mid-1, target),
    }
}

//递归二分搜索
fn search2<E:PartialOrd>(data:&Vec<E>,target:E) -> i32 {
    //调用递归搜索函数
    return recursion_search(data, 0, data.len() as i32 - 1, target)
}

//主函数
fn main(){
    let search_num = 21;
    let lis: Vec<i32> = vec![3,6,8,10,17,21,35,37,46,59];
    println!("原数组:{:?}",lis);
    println!("数组长度:{}",lis.len());
    //中间元素的搜索
    println!("搜索存在于数组中间的数字:{}",search_num);
    let res = search1(&lis, search_num);
    println!("循环二分搜索结果:{}",res);
    let res = search2(&lis, search_num);
    println!("递归二分搜索结果:{}",res);
    //首尾元素的搜索
    let head_num = lis[0];
    println!("搜索数组开头的数字得到结果:(循环={},递归={})"
    ,search1(&lis, head_num),search2(&lis, head_num));
    let tail_num = lis[lis.len()-1];
    println!("搜索数组结尾的数字得到结果:(循环={},递归={})"
    ,search1(&lis, tail_num),search2(&lis, tail_num));
    //不在数组中的元素的搜索
    println!("搜索不在数组中的数字19得到的结果:(循环={},递归={})"
    ,search1(&lis, 19),search2(&lis, 19))
}

