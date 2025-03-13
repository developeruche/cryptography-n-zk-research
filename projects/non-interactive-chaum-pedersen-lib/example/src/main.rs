use library::*;
use library::utils::{exponentiate, generate_random_32_bytes};


fn main() {
    let system_default = NICP::new();


    let x = BigUint::from(300u32);
    let y1 = exponentiate(&system_default.alpha, &x, &system_default.modulus);
    let y2 = exponentiate(&system_default.beta, &x, &system_default.modulus);

    let k = BigUint::from(10u32);

    let r1 = exponentiate(&system_default.alpha, &k, &system_default.modulus);
    let r2 = exponentiate(&system_default.beta, &k, &system_default.modulus);

    let c = gen_challenge(
        &y1,
        &y2,
        &r1,
        &r2
    );


    let solution = solve_challenge(
        &k,
        &x,
        &c,
        &system_default.order
    );


    let verify = verify_challenge(&system_default.alpha, &system_default.beta, &solution, &c, &y1, &y2, &system_default.modulus);



    dbg!("==============================");
    println!(" Here come the verification:::------->   {}", verify);
    dbg!("==============================");
}