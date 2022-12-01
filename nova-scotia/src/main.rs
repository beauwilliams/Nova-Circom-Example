fn main() {
    let circuit_file = root.join("examples/bitcoin/circom/bitcoin_benchmark.r1cs");
    let witness_generator_file =
        root.join("examples/bitcoin/circom/bitcoin_benchmark_cpp/bitcoin_benchmark");

    let r1cs = load_r1cs(&circuit_file); // loads R1CS file into memory
}
