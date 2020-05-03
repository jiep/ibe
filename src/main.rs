extern crate core;

use core::rand::RAND;
use core::bn254::big::BIG;
use core::bn254::ecp::ECP;
use core::bn254::ecp2::ECP2;
use core::bn254::pair;
use core::bn254::rom;
use core::hash256::HASH256;

fn print_hash256(digest: [u8; 32]) {
    for i in 0..32 {print!("{:02x}",digest[i])}
    println!();
}

fn setup(P: &ECP2, q: &BIG) -> (BIG, ECP2) {
    let mut raw: [u8;100]=[0;100];
    let mut rng = RAND::new();
    rng.clean();
    for i in 0..100 {raw[i]=i as u8}
    rng.seed(100,&raw);
    for _ in 0..100 {
        rng.getbyte();
    }

    let s = BIG::randomnum(&q, &mut rng);
    // println!("Master key: {}", s.tostring());

    let k_pub = P.mul(&s);
    // println!("Public key: {}", k_pub.tostring());

    (s, k_pub)
}

fn extraction(id: String, s: &BIG, Q: &ECP) -> ECP {
    // let id = String::from("hola mundo");
    let id_bytes = id.into_bytes();
    let mut hasher = HASH256::new();
    for i in 0..id_bytes.len(){
        hasher.process(id_bytes[i]);
    }
    // let digest = hasher.hash();
    // println!("H1(ID)");
    // print_hash256(digest);
    let x = BIG::frombytes(&hasher.hash());
    let q_id = pair::g1mul(&Q, &x);
    println!("q_id: {}", q_id.tostring());
    let d_id = q_id.mul(&s);
    // println!("d_id: {}", d_id.tostring());

    d_id
}

fn encryption(m: String, id: String, k_pub: &ECP2, P: &ECP2, Q:&ECP, q: &BIG) {
    let id_bytes = id.into_bytes();
    let mut hasher = HASH256::new();
    for i in 0..id_bytes.len(){
        hasher.process(id_bytes[i]);
    }
    // let digest = hasher.hash();
    // println!("H1(ID)");
    // print_hash256(digest);
    let x = BIG::frombytes(&hasher.hash());
    let q_id = pair::g1mul(&Q, &x);
    println!("q_id: {}", q_id.tostring());

    let mut raw: [u8;100]=[0;100];
    let mut rng = RAND::new();
    rng.clean();
    for i in 0..100 {raw[i]=i as u8}
    rng.seed(100,&raw);
    for _ in 0..100 {
        rng.getbyte();
    }

    let r = BIG::randomnum(&q, &mut rng);
    println!("r: {}", r.tostring());

    let mut g_id = pair::ate(&k_pub, &q_id);
    g_id = pair::fexp(&g_id);
    g_id = pair::gtpow(&g_id, &r);

    println!("Message {:?}", m.as_bytes());
    let mut w: [u8];
    println!("g_id {:?}", w);

    println!(g_id^m);

    // let c = (P.mul(&r), m.into_bytes()^g_id.tobytes());
    //
    //
    //
    // return c;


}

fn main() {
    let P = ECP2::generator();
    println!("Generator of G2: {}", P.tostring());

    let Q = ECP::generator();
    println!("Generator of G1: {}", Q.tostring());

    let q = BIG::new_ints(&rom::CURVE_ORDER);
    println!("Order: {}", q.tostring());

    let (s, k_pub) = setup(&P, &q);
    println!("Master key: {}", s.tostring());
    println!("Public key: {}", k_pub.tostring());

    let id = String::from("pepe@gmail.com");
    let d_id = extraction(id.clone(), &s, &Q);
    println!("d_id: {}", d_id.tostring());

    let m = String::from("message");
    let c = encryption(m, id, &k_pub, &P, &Q, &q);
}
