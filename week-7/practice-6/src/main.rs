fn main() {
    let mut num:i32 = 5;
    num_num(&mut num);
    println!("The value of num is:{}",num);
}

fn num_num(param_num:&mut i32){
    *param_num = *param_num*0;
    println!("param_numvalue is :{}",param_num);
}
