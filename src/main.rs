use HDprogrammutator::ProgramMutator;
use HDprogrammutator::rand_string;
use std::process::Command;
use std::fs;
//use std::io::{self, Write};

fn main() -> Result<(), std::io::Error> {
    fs::create_dir("/tmp/hdlklfuzzer")?;
    fs::create_dir("/tmp/hdlklfuzzer/crashes")?;
    fs::create_dir("/tmp/hdlklfuzzer/staging")?;
    fs::create_dir("/tmp/hdlklfuzzer/panics")?;

    let hash = rand_string(12);
    // store serialized program into serialized_filename
    let mut serialized_filename = String::from("/tmp/hdlklfuzzer/staging/serialized");
    serialized_filename.push_str(&hash);
    
    //make a copy of image to /tmp/hdlklfuzzer/staging
    let mut image_path = String::from("/tmp/hdlklfuzzer/staging/");
    image_path.push_str("image");
    image_path.push_str(&hash);
    fs::copy("target/rootfs.img", image_path.clone())?;
    
    // loop
    let mut p = ProgramMutator::new();
    p.add_n_random_syscalls(15);
    p.to_path(&serialized_filename)?;
    let output = Command::new("./hdexecutor")
                    .args([serialized_filename, image_path, "btrfs".to_string()])
                    .env("LD_LIBRARY_PATH", ".")
                    .output()
                    .expect("failed to execute process");
    //io::stdout().write_all(&output.stderr)?;
    //io::stdout().write_all(&output.stdout)?;
    let exit_code = output.status.code().unwrap();
    if exit_code == 101 {
        
    } else if exit_code != 0 {

    }

    // remove files from staging
    Ok(())
}
