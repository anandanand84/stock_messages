fn main() {
    prost_build::compile_protos(&["src/StockMessages.proto"],
                                &["src/"]).unwrap();
}