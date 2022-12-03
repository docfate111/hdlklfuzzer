use std::fs;
use std::process::Command;
use HDprogrammutator::rand_string;
use HDprogrammutator::ProgramMutator;
//use std::io::{self, Write};

fn main() -> Result<(), std::io::Error> {
    fs::create_dir("/tmp/hdlklfuzzer")?;
    fs::create_dir("/tmp/hdlklfuzzer/crashes")?;
    fs::create_dir("/tmp/hdlklfuzzer/staging")?;
    fs::create_dir("/tmp/hdlklfuzzer/panics")?;

    let mut check = 0;
    let mut panics = 0;
    loop {
        let hash = rand_string(12);
        // store serialized program into serialized_filename
        let mut full_serialized_filename = String::from("/tmp/hdlklfuzzer/staging/");
        let mut serialized_filename = String::from("serialized");
        serialized_filename.push_str(&hash);
        full_serialized_filename.push_str(&serialized_filename);

        //make a copy of image to /tmp/hdlklfuzzer/staging
        let mut full_image_path = String::from("/tmp/hdlklfuzzer/staging/");
        let mut image_path = String::from("image");
        image_path.push_str(&hash);
        full_image_path.push_str(&image_path);
        fs::copy("target/rootfs.img", full_image_path.clone())?;

        // loop
        let mut p = ProgramMutator::new();
        p.add_n_random_syscalls(17);
        p.to_path(&full_serialized_filename)?;
        let output = Command::new("./hdexecutor")
            .args([
                full_serialized_filename.clone(),
                full_image_path.clone(),
                "btrfs".to_string(),
            ])
            .output()
            .expect("failed to execute process");
        //io::stdout().write_all(&output.stderr)?;
        //io::stdout().write_all(&output.stdout)?;
        let exit_code = output.status.code().unwrap();
        if exit_code == 101 {
            let panic_path = String::from("/tmp/hdlklfuzzer/panics/");
            let mut image_panic_path = panic_path.clone();
            image_panic_path.push_str(&image_path);
            fs::copy(full_image_path.clone(), image_panic_path)?;

            let mut serialized_panic_path = panic_path.clone();
            serialized_panic_path.push_str(&serialized_filename);
            fs::copy(full_serialized_filename.clone(), serialized_panic_path)?;
            panics += 1;
        } else if exit_code != 0 {
            // assuming it is a crash
            let panic_path = String::from("/tmp/hdlklfuzzer/crashes/");
            let mut image_panic_path = panic_path.clone();
            image_panic_path.push_str(&image_path);
            fs::copy(full_image_path.clone(), image_panic_path.clone())?;

            let mut serialized_panic_path = panic_path.clone();
            serialized_panic_path.push_str(&serialized_filename);
            fs::copy(
                full_serialized_filename.clone(),
                serialized_panic_path.clone(),
            )?;
            let mut cprog = serialized_panic_path.clone();
            cprog.push_str(".c");
            p.cprogram_to_file(&mut cprog)?;
            check += 1;
        }
        println!("panics {panics}\n check {check} ");
        // remove files from staging
        fs::remove_file(full_image_path)?;
        fs::remove_file(full_serialized_filename)?;
    }
    Ok(())
}
