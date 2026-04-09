# Audit Report 1

## 1) Verdict
Partial Pass

## 2) Scope
Static code/doc review only. No runtime execution included in this report.

## 3) Highest-Risk Findings
- High: permission enforcement still relies mainly on role hierarchy rather than granular runtime permission checks.
- High: some object/store-level authorization paths need additional regression coverage.
- High: manual runtime verification is still required for QR scanning and backup/restore workflows.
- Medium: session refresh and token lifecycle behavior needs end-to-end runtime confirmation.
- Medium: docs/test-coverage synchronization requires ongoing maintenance as tests evolve.

## 4) Security Summary
- Authentication entry points: Partial Pass
- Route-level authorization: Partial Pass
- Object-level authorization: Partial Pass
- Function-level authorization: Partial Pass
- Tenant/store isolation: Partial Pass
- Admin/internal protection: Partial Pass

## 5) Tests and Logging Summary
- Unit tests: Partial Pass
- API/integration tests: Partial Pass
- Logging/observability: Partial Pass
- Sensitive-data leakage risk: Partial Pass