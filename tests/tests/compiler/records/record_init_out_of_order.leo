/*
namespace: Compile
expectation: Pass
*/

// TODO: This should be made into an integration test.

program test.aleo {
    record Token {
        // The token owner.
        owner: address,
        // The Aleo balance (in gates).
        gates: u64,
        // The token amount.
        amount: u64,
    }

    transition main(owner: address, a: u64) -> Token {
        let amount: u64 = a * a;
        return Token {
            amount,
            gates: 0u64,
            owner,
        };
    }
}