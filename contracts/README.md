# Sigstore Attestation Verifier Contracts

Smart contracts for verifying ZK proofs of Sigstore bundle attestations on-chain. Supports multiple ZK co-processors: RiscZero, Succinct (SP1), and Pico.

## Prerequisites

- **Rust** - Required for ZK proof generation (host programs)
- **Foundry** - Smart contract development framework
  ```bash
  curl -L https://foundry.paradigm.xyz | bash
  foundryup
  ```

## API

### Input Format

The contract accepts ZK proofs generated from Sigstore bundle verification. See test fixtures for examples:

**RiscZero proof** (`test/fixtures/boundless-public.json`):
```json
{
  "zkvm": "risc0",
  "program_id": "0xddcec7db184cde2e6d8419f795308f6cf849626434be292e2adff357efaee0ef",
  "circuit_version": "3.0.3",
  "journal": "0x...",
  "proof": "0x..."
}
```

**SP1 proof** (`test/fixtures/sp1-github.json`):
```json
{
  "zkvm": "sp1",
  "program_id": "0x0081d74e3b06e31064884f3441929c5279eaae8e1dcf9a51874af1262b6c11eb",
  "circuit_version": "v5.0.0",
  "journal": "0x...",
  "proof": "0x..."
}
```

### `verifyAndAttestWithZKProof()`

```solidity
function verifyAndAttestWithZKProof(
    bytes calldata output,           // Journal/public output from ZK proof
    ZkCoProcessorType zkCoProcessor, // 1=RiscZero, 2=Succinct, 3=Pico
    bytes calldata proofBytes        // ZK proof bytes
) external returns (VerificationResult memory)
```

### Output

The function returns a `VerificationResult` struct containing verified attestation data:

```solidity
struct VerificationResult {
    uint64 timestamp;                      // Unix timestamp of signing
    TimestampProofType timestampProofType; // None, Rfc3161, or Rekor
    bytes32[] certificateHashes;           // [leaf, ...intermediates, root]
    bytes subjectDigest;                   // Artifact hash (build artifact digest)
    DigestAlgorithm subjectDigestAlgorithm;// SHA256 or SHA384
    string oidcIssuer;                     // e.g., "https://token.actions.githubusercontent.com"
    string oidcSubject;                    // OIDC subject identity
    string oidcWorkflowRef;                // GitHub workflow reference
    string oidcRepository;                 // e.g., "owner/repo"
    string oidcEventName;                  // e.g., "push", "schedule", "pull_request"
    bytes32[] tsaChainHashes;              // TSA certificate chain (RFC3161)
    DigestAlgorithm messageImprintAlgorithm;
    bytes messageImprint;
    bytes32 rekorLogId;                    // Rekor transparency log ID
    uint64 rekorLogIndex;                  // Merkle tree leaf index
    uint64 rekorEntryIndex;                // API entry index
}
```

### Use Cases

Applications can use the verified output for:

- **Supply Chain Security**: Verify that a binary/artifact was built from a specific repository and workflow
- **On-chain Artifact Registry**: Store verified build attestations as on-chain records
- **DAO Governance**: Require ZK-verified attestations before accepting code deployments
- **Trustless Build Verification**: Prove artifact provenance without trusting centralized authorities
- **Compliance Auditing**: Maintain immutable, verifiable records of software builds

## Commands

### Build

```bash
cd contracts
forge build
```

### Test

```bash
forge test
```

With verbose output:
```bash
forge test -vvv
```

### Deploy

**Environment setup:**
```bash
export OWNER="<owner-address>"
export RISC_ZERO_IMAGE_ID="0xddcec7db184cde2e6d8419f795308f6cf849626434be292e2adff357efaee0ef"
export SP1_VKEY="0x0081d74e3b06e31064884f3441929c5279eaae8e1dcf9a51874af1262b6c11eb"
```

**Single chain deployment:**
```bash
forge script script/Deploy.s.sol --sig "runSingle()" \
  --rpc-url <RPC_URL> \
  --broadcast
```

**Multi-chain deployment:**
Configure chains in `script/config/deployment.toml`, then:
```bash
forge script script/Deploy.s.sol --sig "run()" --broadcast
```

**Update ZK configuration on existing deployment:**

The config script reads the contract address from `deployments/{chainId}.json` automatically.

```bash
# Configure specific ZK processor
forge script script/Config.s.sol --sig "configureRiscZero()" --rpc-url <RPC_URL> --broadcast
forge script script/Config.s.sol --sig "configureSp1()" --rpc-url <RPC_URL> --broadcast
forge script script/Config.s.sol --sig "configurePico()" --rpc-url <RPC_URL> --broadcast

# Configure all enabled processors
forge script script/Config.s.sol --sig "configureAll()" --rpc-url <RPC_URL> --broadcast
```

## Deployment Info

### ZK Program IDs

| ZK System | Program ID | Version |
|-----------|------------|---------|
| RiscZero | `0xddcec7db184cde2e6d8419f795308f6cf849626434be292e2adff357efaee0ef` | 3.0.3 |
| Succinct (SP1) | `0x0081d74e3b06e31064884f3441929c5279eaae8e1dcf9a51874af1262b6c11eb` | v5.0.0 |
| Pico | `0x00f34a5c62d04c190c43cbfa40f2e9c94a8514a3292cd0798e28f58c286c72fc` | v1.1.8 |

### Deployed Networks

#### ATA Sepolia Testnet (Chain ID: 1398243)

| Contract | Address |
|----------|---------|
| SigstoreAttestationVerifier | `0x1B1c7e34aF05bE9D8c93c1A0c1e1056b5272Bb6a` |
| RiscZero Verifier | `0xaE7F7EC735b6A90366e55f87780b36e7e6Ec3c65` |
| SP1 Verifier | `0x7291752B7c1e0E69adF9801865b25435b0bE4Fc6` |
| Pico Verifier | `0x961B849FEc59c355868F4ac0490c16F112DFA4F3` |

**Network Details:**
- RPC URL: `https://1rpc.io/ata/testnet`
- Owner: `0x3D089C2f2CB86d4EfDe153C81cAbD4579784430b`