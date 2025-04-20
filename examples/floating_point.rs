fn main() {
    let a: f32 = 42.42;
    let franken_type: u32 = unsafe {
        // 주어진 비트에 영향을 주지 않고 (f32 u32) 변환한다
        std::mem::transmute(a)
    };

    // 십진 정수로 표현
    println!("{}", franken_type);

    // 32자리 비트로 표현
    println!("{:032b}", franken_type);

    let b: f32 = unsafe {
        // 주어진 비트에 영향을 주지 않고 (f32 u32) 변환한다
        std::mem::transmute(franken_type)
    };

    println!("{}" , b);
    assert_eq!(a,b);

    // 부동 소수점은 1비트의 부호비트, 8비트의 지수부, 23비트의 가수부로 이루어져 있다
    // 01000010001010011010111000010100(42.42 f32의 비트 표현)
    // S(ign)= 0, M(antissa)= 0.01010011010111000010100, E(xponent)= 10000100(132)
    // (-1)^S × (1 + M) × 2^(E - 127)
    // (+1) × (1.01010011010111000010100)₂ × 2^5
    // 1.325625 × 2^5 = 1.325625 × 32 ≈ 42.42
    // bias는 127이며, 음수의 지수를 표현하기 위해 해당 값이 존재한다


    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    // 부호 비트
    let sign_bit = n_bits >> 31;
    println!("sign_bit : {:032b}", sign_bit);
    // result
    // sign_bit : 00000000000000000000000000000000

    let exponent_ = n_bits >> 23;

    // 연산 대상 타입(u32, i32 등)으로 변환됨
    // 0으로 채워지는 zero-extension 발생
    let exponent_ = exponent_ & 0xff;

    // bias 제거, 제거 후에는 sign존재
    let exponent_ = (exponent_ as i32) - 127;

    println!("exponent : {:032b}", exponent_);

    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }
    println!("mantissa = {:.6}", mantissa); 

}