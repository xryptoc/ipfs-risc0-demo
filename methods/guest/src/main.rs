// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

use ipfs_core::IpfsProof;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    let ipfs_proof_request: IpfsProof = env::read();

    println!("Finished in {} cycles", env::cycle_count());

    println!("Current in {} cwycles", env::cycle_count());

    let res = ipfs_proof_request.calculate_proof();

    // // Commit to the journal the verifying key and message that was signed.
    env::commit(&res.hash);
    env::commit(&res.data);

    println!("Finished in {} cycles", env::cycle_count());
}
