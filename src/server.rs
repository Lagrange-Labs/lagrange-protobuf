pub mod lagrange {
    pub mod preprocessing {
        pub mod v1 {
            tonic::include_proto!("lagrange.preprocessing.v1");
        }
    }

    tonic::include_proto!("lagrange");

    pub mod groth16 {
        pub mod v1 {
            tonic::include_proto!("lagrange.groth16.v1");
        }
    }

    pub mod query {
        pub mod v1 {
            tonic::include_proto!("lagrange.query.v1");
        }
    }
    pub mod experimental {
        pub mod rec_proof {
            pub mod v1 {
                tonic::include_proto!("lagrange.experimental.rec_proof.v1");
            }
        }
        pub mod tx_trie {
            pub mod v1 {
                tonic::include_proto!("lagrange.experimental.rec_proof.v1");
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
