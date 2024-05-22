use std::fs;

fn main() -> std::io::Result<()> {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["protos/user.proto"], &["protos"])?;

    println!("cargo:rerun-if-changed=protos/user.proto");
        
    Ok(())
}
