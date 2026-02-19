# WINDSURF EXECUTION FILE — SHA × vlayer ZKP Integration
# =========================================================
# READ BEFORE EXECUTING:
#
# - Execute ONE phase at a time.
# - Do NOT proceed to the next phase until the user says "execute phase N".
# - After completing each phase, print exactly:
#   ✅ PHASE N COMPLETE — STANDBY FOR PHASE N+1 PROMPT
# - Do not modify main branch. All work is in feat/zkp-vlayer-integration.
# - All file paths are relative to repo root.
# =========================================================

# =============================================================
# PHASE 1 — Branch Scaffold + Architecture + Interface Design
# STATUS: EXECUTE NOW
# =============================================================
#
# Goal: Create the ZKP branch, full doc suite, scaffolded file
# tree, and Rust interface stubs. No circuit logic yet.
# This shows vlayer grant reviewers: branch exists, architecture
# is designed, interfaces are defined, roadmap is clear.

# ── STEP 1: Git branch ───────────────────────────────────────

git checkout main
git pull origin main
git checkout -b feat/zkp-vlayer-integration

# ── STEP 2: Directory scaffold ───────────────────────────────

mkdir -p zkp/circuits
mkdir -p zkp/contracts
mkdir -p zkp/prover
mkdir -p zkp/scripts
mkdir -p zkp/tests
mkdir -p docs/zkp

touch zkp/circuits/.gitkeep
touch zkp/prover/.gitkeep
touch zkp/tests/.gitkeep

# ── STEP 3: Create all files listed below ────────────────────
# Windsurf: create each file with the exact content specified.

# FILE LIST FOR PHASE 1:
# 1.  zkp/README.md
# 2.  zkp/contracts/IZkVerifier.rs
# 3.  zkp/contracts/sha_v2_interface.rs
# 4.  zkp/scripts/prove_and_submit.py
# 5.  docs/zkp/ARCHITECTURE.md
# 6.  docs/zkp/CIRCUIT_SPEC.md
# 7.  docs/zkp/INTEGRATION.md
# 8.  docs/zkp/ZK_ROADMAP.md
# 9.  PROGRESS.md  (repo root)
# 10. Update existing README.md to add ZKP section

# ── STEP 4: Commit ───────────────────────────────────────────

git add -A
git commit -m "feat(zkp): phase 1 scaffold — architecture, interfaces, docs

- Add feat/zkp-vlayer-integration branch
- Define 4-phase ZK integration roadmap
- Stub IZkVerifier trait and SHA v2 contract interface
- Add circuit spec, architecture doc, integration guide
- Add prove_and_submit.py prover script scaffold
- Add PROGRESS.md tracking vlayer grant momentum
- No circuit logic yet — Phase 2 deliverable

vlayer grant applied: 2026-02-19
SHA deployed on Sepolia: 0xD661a1aB8CEFaaCd78F4B968670C3bC438415615"

git push origin feat/zkp-vlayer-integration

# ── AFTER COMMIT: Print ──────────────────────────────────────
# ✅ PHASE 1 COMPLETE — STANDBY FOR PHASE 2 PROMPT

# =============================================================
# PHASE 2 — ZK Circuit + vlayer Prover + SHA v2 Contract
# STATUS: STANDBY — await "execute phase 2" from user
# =============================================================
#
# Goal: Real vlayer circuit for execution correctness.
# SHA v2 contract with verify_receipt_with_zk() deployed to Sepolia.
# End-to-end prove → submit flow working on testnet.

# FILE LIST FOR PHASE 2 (do not create yet):
# 1.  zkp/circuits/execution_proof.nr       (Noir circuit)
# 2.  zkp/contracts/sha_v2.rs               (full Stylus contract)
# 3.  zkp/prover/generate_proof.py          (vlayer CLI wrapper)
# 4.  zkp/tests/test_zk_verify.rs           (integration tests)
# 5.  zkp/scripts/deploy_sha_v2.sh          (deployment script)
# 6.  docs/zkp/BENCHMARKS_ZK.md            (gas benchmarks for ZK path)

# =============================================================
# PHASE 3 — Batch Proof Aggregation
# STATUS: STANDBY — await "execute phase 3" from user
# =============================================================

# =============================================================
# PHASE 4 — Recursive ZK / DePIN Scale
# STATUS: STANDBY — await "execute phase 4" from user
# =============================================================
