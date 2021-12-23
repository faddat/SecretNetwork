fn main() {
    protobuf::cleanup();
    protobuf::build_protobuf_parsers()
}

#[cfg(not(feature = "build-protobuf"))]
mod protobuf {
    pub fn cleanup() {}
    pub fn build_protobuf_parsers() {}
}

#[cfg(feature = "build-protobuf")]
mod protobuf {
    use protoc_rust::Customize;
    use std::{
        fs,
        path::{Path, PathBuf},
    };

    pub fn cleanup() {
        let _ = fs::remove_dir_all(&out_dir());
    }
    fn out_dir() -> PathBuf {
        Path::new("src/proto").to_path_buf()
    }

    pub fn build_protobuf_parsers() {
        let protoc_err_msg = "protoc failed to generate protobuf parsers";

        let directories: &[(&str, &[&str])] = &[
            (
                "src/proto/base",
                &["../../../third_party/proto/cosmos/base/v1beta1/coin.proto"],
            ),
            (
                "src/proto/crypto/multisig",
                &[
                    "../../../third_party/proto/cosmos/crypto/multisig/v1beta1/multisig.proto",
                    "../../../third_party/proto/cosmos/crypto/multisig/keys.proto",
                ],
            ),
            (
                "src/proto/crypto/secp256k1",
                &["../../../third_party/proto/cosmos/crypto/secp256k1/keys.proto"],
            ),
            (
                "src/proto/crypto/secp256r1",
                &["../../../third_party/proto/cosmos/crypto/secp256r1/keys.proto"],
            ),
            (
                "src/proto/crypto/ed25519",
                &["../../../third_party/proto/cosmos/crypto/ed25519/keys.proto"],
            ),
            (
                "src/proto/tx",
                &[
                    "../../../third_party/proto/cosmos/tx/v1beta1/tx.proto",
                    "../../../third_party/proto/cosmos/tx/signing/v1beta1/signing.proto",
                ],
            ),
            (
                "src/proto/cosmwasm",
                &["../../../proto/secret/compute/v1beta1/msg.proto"],
            ),
        ];

        for (out_dir, inputs) in directories {
            let dir_err_msg = format!("failed to generate directory {:?}", out_dir);
            std::fs::create_dir_all(*out_dir).expect(&dir_err_msg);

            protoc_rust::Codegen::new()
                .include("../../../proto/secret/compute/v1beta1") // cosmwasm
                .include("../../../third_party/proto/")
                .out_dir(*out_dir)
                .inputs(*inputs)
                .customize(Customize {
                    ..Default::default()
                })
                .run()
                .expect(protoc_err_msg);

            let mod_dir = Path::new(out_dir).to_path_buf();

            let mods = glob::glob(&mod_dir.join("*.rs").to_string_lossy())
                .expect("glob")
                .filter_map(|p| {
                    p.ok()
                        .map(|p| format!("pub mod {};", p.file_stem().unwrap().to_string_lossy()))
                })
                .collect::<Vec<_>>()
                .join("\n");

            let mod_rs = mod_dir.join("mod.rs");
            fs::write(&mod_rs, format!("// @generated\n{}\n", mods)).expect("write");
        }
    }
}
