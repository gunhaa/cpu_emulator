fn main(){

    // 고정 소수점은
    // 실수를 정수로 저장하기 위해 일정한 비율(스케일링)을 곱해서 정수로 만든 다음,
    // 나중에 다시 원래 소수로 복원할 수 있게 소수점의 위치를 약속해두는 표현 방식

    // Q7의 경우
    // 소수점이 7비트 뒤에 있다고 약속한 방식
    // 실수 x를 x * 128 해서 정수(i8)에 저장
    // 나중에 정수 / 128로 원래 실수 복원

    // 즉, i8: 부호 있는 8비트 정수 → 표현 가능한 값은 -128 ~ 127
    // 최솟값: -128 / 128 = -1.0
    // 최댓값:  127 / 128 ≈ 0.9921875
    // 즉, 총 표현 가능한 값 = 256개


    // f64 → Q7 변환
    let a = 0.5_f64;
    let q = Q7::from(a);
    println!("Q7 representation of {} is {:?}", a, q);

    // Q7 → f64 변환
    let b = f64::from(q);
    println!("Converted back to f64: {}", b);

    // f32 → Q7 변환
    let a_f32 = -0.25_f32;
    let q_f32 = Q7::from(a_f32);
    println!("Q7 representation of {} is {:?}", a_f32, q_f32);

    // Q7 → f32 변환
    let b_f32 = f32::from(q_f32);
    println!("Converted back to f32: {}", b_f32);

    // 최대/최소값 테스트
    let max_q = Q7::from(1.5);
    let min_q = Q7::from(-2.0); 
    println!("Clamped max Q7: {:?}", max_q);
    println!("Clamped min Q7: {:?}", min_q);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
    fn from (n:f64) -> Self {
        // assert!(n>=-1.0);
        // assert!(n <=1.0);
        if n >= 1.0 {
            Q7(127)
        } else if n <= -1.0 {
            Q7(-128)
        } else {
            Q7((n*128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> f64 {
        (n.0 as f64) * 2f64.powf(-7.0)
    }
}

impl From<f32> for Q7 {
    fn from(n: f32) -> Self {
        Q7::from(n as f64)
    }
}

impl From<Q7> for f32 {
    fn from(n: Q7) -> f32 {
        f64::from(n) as f32
    }
}