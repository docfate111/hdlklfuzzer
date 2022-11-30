use HDprogrammutator::ProgramMutator;
use HDprogrammutator::rand_string;
use std::process::Command;

fn main() -> Result<(), std::io::Error> {
    let mut p = ProgramMutator::new();
    p.add_n_random_syscalls(15);
    let mut serialized_filename = String::from("serialized");
    serialized_filename.push_str(&rand_string(10));
    p.to_path(&serialized_filename)?;
    let image_path = String::from("target/rootfs.img");
    let _output = Command::new("./hdexecutor")
                    .args([serialized_filename, image_path, "btrfs".to_string()])
                    .env("LD_LIBRARY_PATH", ".")
                    .output()
                    .expect("failed to execute process");
    //io.write_all(output.stderr);
    //io.write_all(output.stdout);
    Ok(())
}
