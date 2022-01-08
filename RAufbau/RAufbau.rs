//-------------------------//
#![allow(non_snake_case)]
//-------------------------//
#![allow(dead_code)]
//-------------------------//
#![allow(unused_variables)]
//-------------------------//

use std::io;
use std::collections::HashMap;

fn input() -> String {
    let mut input = String::new();
    println!("Electron number, bitte!");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
                return input.to_string();
            },
        Err(e) => { 
            return e.to_string();
        }
    }
}


fn main()
{
    let mut mld = HashMap::new();
    mld.insert("s", 0);
    mld.insert("p", 1);
    mld.insert("d", 2);
    mld.insert("f", 3); 

    let mut exceptions = HashMap::new();
    exceptions.insert(24, "1s^2 2s^2 2p^6 3s^2 3p^6 4s^1 3d^5");
    exceptions.insert(29, "1s^2 2s^2 2p^6 3s^2 3 4s^1 3d^10");

    let orbitals = vec!["1s^", "2s^", "2p^", "3s^", "3p^", "4s^", "3d^", "4p^", "5s^", "4d^", "5p^", "6s^", "4f^", "5d^", "6p^", "7s^", "5f^", "6d^", "7p^"];

    let mut configuration = String::new();
    let element = String::new();
    let mut configuration_step = 0;
    let mut e = 0;
    let loop_key = true;
    let result = "Configuration of Element ".to_owned() + &element + " is: ";

    let input = input();
    let electron_str = input.trim_end();
    let mut electron_num: i32 = electron_str.parse().unwrap();

    if electron_num == 29 || electron_num == 24 {
         println!("{} {:#?}", result, exceptions[&electron_num]);
    }else{
        while loop_key
        {
            let l = &orbitals[configuration_step][1..2];
            let ml = mld[&l];
            let mln = (2*ml+1)*2;

            if electron_num > mln{
                e = mln;
                let orbital = orbitals[configuration_step];
                configuration = format!("{} {}{}", configuration, orbital, e);
                configuration_step += 1;
                electron_num -= mln;
            }else if electron_num == mln{
                e = mln;
                let orbital = orbitals[configuration_step];
                configuration = format!("{} {}{}", configuration, orbital, e);
                println!("{} {}", result, configuration);
                break;
            }else if electron_num < mln{
                e = mln;
                let orbital = orbitals[configuration_step];
                configuration = format!("{} {}{}", configuration, orbital, e);
                println!("{} {}", result, configuration);
                break;
            }
        }
    }
}

