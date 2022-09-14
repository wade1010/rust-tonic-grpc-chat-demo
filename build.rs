fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["./abi.proto"], &["./"])
        .unwrap();
    println!("cargo:return-if-changed=./abi.proto");
    println!("cargo:return-if-changed=./build.rs");
}
