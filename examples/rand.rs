

fn main(){
    println!("mock1 : {}", mock_rand_1(123));

    println!("max input : {:08b} -> {:?}", 0xff, mock_rand_2(0xff));
    println!("mid input : {:08b} -> {:?}", 0x7f, mock_rand_2(0x7f));
    println!("min input : {:08b} -> {:?}", 0x0f, mock_rand_2(0x0f));
}

// 나누기는 속도가 느린 연산이다
fn mock_rand_1(n: u8) -> f32 {
    (n as f32) / 255.0
}


// 나누기없이 랜덤 생성
fn mock_rand_2(n: u8) -> f32 {
    let base: u32 = 0b0_01111110_00000000_00000000_0000000;

    let large_n = (n as u32) << 15;

    let f32_bits = base | large_n;

    let m = f32::from_bits(f32_bits);

    2.0*(m-0.5)
}