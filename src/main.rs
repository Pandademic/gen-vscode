mod lisp;

use std::env;
use std::fs;
use std::process::Command;
use execute::Execute;
use pathsearch::find_executable_in_path;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("Hello, world!");
    println!("reading from file:{}",&args[1]);
    let contents = fs::read_to_string(&args[1])
        .expect("something went wrong reading the file");

    let (settings,exts) = lisp::eval(&contents);

    let mut use_insiders = false;

    if args.len() > 2 {
        if &args[2] == "-insiders"{
            println!("recived the -insiders flag , assuming that code-insiders is to be used");
            use_insiders = true;
        }else {
            println!("unknown flag recived at position 2, ignoring");
        }
    }

    for s in settings {
        println!("{}",s);
    }

    for e in exts {
        if use_insiders {
                let e_str:&str = &*e;
                if let Some(vsc_exe) = find_executable_in_path("code-insiders"){
                    let vsc_path_as_str = vsc_exe.as_os_str().to_str().unwrap(); 
                    println!("{}",vsc_path_as_str);
                    let mut exec_cmd = Command::new(vsc_path_as_str.to_owned()+".exe");
                    exec_cmd.arg("--install-extension");
                    exec_cmd.arg(e_str);
                    exec_cmd.arg("--force");
                    if let Some(exit_code) = exec_cmd.execute().unwrap() {
                        if exit_code == 0 {
                            println!("Ok.");
                        } else {
                            eprintln!("Failed.");
                        }
                    } else {
                        eprintln!("Interrupted!");
                    }
                }
        }else {
            let output = Command::new("code")
                .arg("--install-extension")
                .arg(e)
                .arg("--force")
                .output()
                .expect("failed to install extension");

         println!("stdout:{}",String::from_utf8_lossy(&output.stdout));
         println!("stderr: {}",String::from_utf8_lossy(&output.stderr));
        }
    }
}
