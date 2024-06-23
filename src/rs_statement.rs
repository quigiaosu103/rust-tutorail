use std::{array, fmt::Debug, num::ParseIntError};
use std::thread;
use std::time::Duration;


// struct Part<'a> {
//     part: &'a str
// }
// impl<'a> Part<'a> {
//     fn get_part(& self, str: &str)  -> &str{
//         println!("{}", str);
//         self.part
//     }
// }

// impl  Debug for SumationError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("SumationError!!!").finish()
//     }
// }

// struct SumationError;
// fn to_i32(str: &str) -> Result<u32, ParseIntError> {
//     str.parse()
// }

// fn sum(vec: &Vec<String>) -> Result<u32, SumationError> {
//     let mut sum: u32 = 0;
//     for item in vec {
//         sum += to_i32(&item).map_err(|_| SumationError)?; 
//     }
//     Ok(sum)
// }



// fn main() {
//     let longest: &str;
//     {
//         let str1: & str = "String::from";    
//         let str2: & str = "str2 static striong"; 
//         longest = longest_f(str1, str2);
//     }
//     println!("{}", longest);   
//     let part = Part {
//         part: "hello"
//     };

//     let new_vec = vec![String::from("10a"), String::from("19")];
//     let sum_str = sum(&new_vec);
//     println!("{:?}", new_vec);
//     println!("sum: {:?}", sum_str);
//     // let inner_part = part.get_part(str1.as_str());
//     // println!("{}", inner_part);
       
// }

// fn longest_f<'a>(str1: & str, str2: &'a str) -> &'a str { //'a de thong bao rang lifetime cua return value se co mqh voi str1 va str2 
//     let s: &str = "static string ref";
//     s
// } 

//neu chi co 1 ref thi output ref se sd lifetime cua input ref 
#[tokio::main]
async fn main() {
    my_f().await;
}


async fn my_f() {
    let job_handle_1 = tokio::spawn(
        get_from_db()
    );
    let job_handle_2 = tokio::spawn(
        get_from_db()
    );

    tokio::join!(
        job_handle_1,
        job_handle_2
    );
    // let result = s.0.unwrap();
    // let r2 = s.1.unwrap();
    // println!("r1: {}, r2: {}", result, r2);

    
    // println!("Call async function!");
    // let data1 = get_from_db().await;
    // println!("read data 1: {}", data1);
    // let mut rt = tokio::runtime::Runtime::new().unwrap();
    // rt.block_on(
    //     async {
    //         println!("blocking");
    //     }
    // );
    
    // let data2 = get_from_db().await;
    // println!("read data 2: {}", data2);
}

async fn get_from_db() -> String {
    println!("Start call data!");
    let mut data = String::from("Some data in database!");
    thread::sleep(Duration::from_secs(1));
    println!("End call data!");

    data
}
