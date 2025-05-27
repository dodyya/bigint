mod big_int;
use big_int::BigInt;
fn main() {
    let mut f_n2 = BigInt::try_from("0").unwrap();
    let mut f_n1 = BigInt::try_from("1").unwrap();

    //fib
    for i in 0..2500 {
        f_n1 = f_n2.clone() + f_n1;
        f_n2 = f_n1.clone() + f_n2;
        // print!("{}:{},{} ", i, f_n2, f_n1);
    }
    print!("{},{} ", f_n2, f_n1);

    //Test division implementation using random values

    //9278135210
    //259701

    // println!("{} / {} = {}", a.clone(), b.clone(), a.clone() / b.clone());
    // println!("{} % {} = {}", a.clone(), b.clone(), a.clone() % b.clone());
}

//5357543035931336604742125245300009052807024058527668037218751941851755255624680612465991894078479290637973364587765734125935726428461570217992288787349287401967283887412115492710537302531185570938977091076523237491790970633699383779582771973038531457285598238843271083830214915826312193418602834034688
