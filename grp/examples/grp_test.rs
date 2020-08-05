#![allow(unused)]

extern crate dirs;
extern crate sys_info;

use grp::*;
use whoami::username;
use chrono::prelude::*;
use std::{
    env,
    cmp::max,
    path::Path,
    error::Error,
    io::prelude::*,
    fs::{ File, OpenOptions }
};


fn main() -> Result<(), Box<dyn Error>> {

    let requirement = env::args()
                          .skip(1)
                          .map(|arg| arg.parse::<BigUint>().expect("Should be positive"))
                          .collect::<Vec<_>>();

    if !requirement.is_empty() {

        let (length,                 sbl_cnt,                num_cnt               )
            =
            (requirement[0].clone(), requirement[1].clone(), requirement[2].clone());

        save_to_desktop(&RandomPassword::new(length, sbl_cnt, num_cnt)?.show());

    } else { // Default

        let rp = RandomPassword::new(300, 23, 32)?.show();
        let head = format!("{} - {}", now_time(), username());
        let width = max(head.len(), rp.len());

        println!("\n{}\n{}\n", head, rp);

    }

    Ok(())

}


fn save_to_desktop(rp: &str) -> Result<(), Box<dyn Error>> {

    let _desktop = dirs::desktop_dir().unwrap();

    let mut filepath = String::new();

    match sys_info::os_type()?.as_str() {

        "Darwin" | "Linux" => { filepath = format!("{}/random_password.txt", _desktop.to_str().unwrap()); },

        "Windows" => { filepath = format!("{}\\random_password.txt", _desktop.to_str().unwrap()); },

        _ => ()
    }

    let mut file: File;

    if !Path::new(filepath.as_str()).exists() {

        file = File::create(filepath.as_str())?;
    }

    file = OpenOptions::new()
                       .append(true)
                       .open(filepath.as_str())?;


    let head = format!("{} - {}", now_time(), username()).to_owned();
    let width = max(head.len(), rp.len());
    let result = writeln!(&mut file, "\n{}\n{}\n", head, rp).is_ok();

    if result {

        println!("Password is saved to {}", filepath.as_str());

    } else {

        println!("Failed to save the password to {}", filepath.as_str());

    }

    Ok(())

}


#[inline]
fn now_time() -> String {

    Local::now()
          .time()
          .to_string()
          .chars()
          .into_iter()
          .map(|c| c.to_string())
          .collect::<Vec<_>>()[..8]
          .join("")

}
