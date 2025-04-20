fn main() {

    // 부동소수점(Floating-point)은 소수점의 위치가 고정되지 않고, 숫자와 소수점 사이의 거리가 동적으로 변하는 방식으로 표현되는 실수의 표기법이다.
    // 컴퓨터에서 실수를 표현할 때 IEEE 754 표준을 기반으로 부동소수점 형식이 사용된다.
    // 지수에 의해 숫자의 크기가 조절되고, 가수에 의해 정확한 값을 나타낸다
    // 지수/가수의 비트를 조절해서 정확도를 조절할 수 있기에 소수점 사이의 거리가 동적으로 변할 수 있다는 것이다.

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