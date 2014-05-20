extern crate contract_and_numerical_range_types;
use contract_and_numerical_range_types::make_range_int_type;

make_range_int_type!(RangedIntNFiveToFive RINFiveToFive OutOfRangeNFiveToFive -5 5)

fn main() {
    /*let i = RangedIntNFiveToFive::from_int(1) + RangedIntNFiveToFive::from_int(-1) ;
    let j = RangedIntNFiveToFive::from_int(20) + RangedIntNFiveToFive::from_int(-22) ;
    let k = RangedIntNFiveToFive::from_int(1) + RangedIntNFiveToFive::from_int(-1);
    let l = RangedIntNFiveToFive::from_int(1).unwrap();
    println!("i: {}", i.to_str());                              //prints "i: RINFiveToFive(0)"
    println!("j: {}", j.to_str());                              //prints "j: OutOfRangeNFiveToFive"
    println!("k: {}", k.to_str());                              //prints "k: RINFiveToFive(0)"
    println!("l: {}", l.to_str());                              //prints "l: 1"

    let m = RangedIntNFiveToFive::from_int(20).unwrap();        //causes exit:"task '<main>' failed at 'failed to unwrap $RangedIntName - OutOfRange!', ranged_num.rs:47"
    println!("m: {}", m.to_str());                              //never reached*/
    println!("hello world!");
}