#[allow(unused_imports)]
pub mod proto {

    pub mod base;
    pub mod cosmwasm {
        pub mod msg;
        use crate::proto::base::coin;
    }
    pub mod tx {
        pub mod signing;
        pub mod tx;
        use crate::proto::base::coin;
        use crate::proto::crypto::multisig::multisig;
    }

    pub mod crypto {
        pub mod ed25519;
        pub mod multisig;
        pub mod secp256k1;
        pub mod secp256r1;

        use crate::proto::base::coin;
    }
}
