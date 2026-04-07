# FleetReserve Operations Suite - Test Coverage Plan

## Coverage Strategy

Risk-first approach: tests prioritize flows where failure has the highest business or security impact.

## Coverage Map

| # | Requirement / Risk Point | Mapped Test Case(s) | Key Assertion / Fixture | Coverage | Gap | Min Test Addition |
|---|-------------------------|---------------------|------------------------|----------|-----|-------------------|
| 1 | Auth success | `test_login_success` | Valid credentials return token + CSRF | Unit + API | None | - |
| 2 | Auth failure | `test_login_invalid_password`, `test_login_unknown_user` | Returns 401, no token | Unit + API | None | - |
| 3 | Route authorization | `test_customer_cannot_access_admin`, `test_photographer_cannot_manage_vehicles` | Returns 403 for insufficient role | Unit + API | Full matrix coverage | Add per-role per-route matrix test |
| 4 | Object-level authorization | `test_store_isolation_vehicles`, `test_store_isolation_reservations` | MerchantStaff from store A gets 403 on store B | API | Cross-store reservation | Add cross-store reservation test |
| 5 | Store isolation | `test_merchant_sees_only_own_store`, `test_merchant_cannot_query_other_store` | Query filters enforce store_id | Unit + API | None | - |
| 6 | Photographer assignment isolation | `test_photographer_sees_only_assignments`, `test_photographer_no_unassigned_access` | Only assigned jobs returned | Unit + API | None | - |
| 7 | Reservation concurrency conflict | `test_overlapping_reservation_detected`, `test_concurrent_reservation_retry` | Conflict returned with reasons | Unit + API | Load testing | Manual: concurrent HTTP clients |
| 8 | Deterministic conflict reason | `test_conflict_reasons_stable_order` | Same input produces same reason set | Unit | None | - |
| 9 | Alternative slot generation | `test_two_nearest_slots_returned`, `test_slots_within_business_hours` | Exactly 2 slots, all in business hours | Unit | Edge: end of day | Add last-slot-of-day test |
| 10 | Alternate vehicle/bay suggestion | `test_alternate_vehicle_suggested`, `test_no_alternate_when_all_busy` | At least 1 alternate or empty list | Unit | None | - |
| 11 | Ticket generation | `test_ticket_number_format`, `test_ticket_qr_contains_data` | FR-XXXXXXXX format, valid JSON QR | Unit | None | - |
| 12 | Ticket redemption | `test_redeem_valid_ticket` | Marks redeemed, records timestamp | Unit + API | None | - |
| 13 | Duplicate redemption blocked | `test_double_redeem_rejected` | Second redeem returns 400 | Unit + API | None | - |
| 14 | Supervised undo within 2 min | `test_undo_within_window` | Undo succeeds, ticket redeemable again | Unit + API | None | - |
| 15 | Undo after 2 min rejected | `test_undo_after_window_rejected` | Returns 400 with window-expired reason | Unit + API | None | - |
| 16 | Undo reason required | `test_undo_without_reason_rejected`, `test_undo_empty_reason_rejected` | Returns 400 | Unit + API | None | - |
| 17 | Vehicle status transition perms | `test_valid_transition_succeeds`, `test_invalid_transition_rejected`, `test_decommission_admin_only` | Permission + transition validation | Unit + API | All transition combinations | Add full transition matrix |
| 18 | Upload MIME validation | `test_valid_jpeg_accepted`, `test_valid_png_accepted`, `test_gif_rejected` | Accept JPEG/PNG, reject others | Unit + API | None | - |
| 19 | Upload magic-byte validation | `test_fake_jpeg_header_rejected`, `test_real_jpeg_magic_bytes` | Magic bytes checked, not just extension | Unit | None | - |
| 20 | Duplicate upload fingerprint | `test_duplicate_fingerprint_rejected` | 409 on same SHA-256 | Unit + API | None | - |
| 21 | VIN/plate/username masking | `test_vin_masked_in_list`, `test_plate_masked`, `test_username_masked` | Only last N chars visible | Unit | None | - |
| 22 | Recovery code expiry | `test_recovery_code_valid_within_30min`, `test_recovery_code_expired_after_30min` | Time-based validation | Unit + API | None | - |
| 23 | Backup authorization | `test_backup_admin_only`, `test_backup_non_admin_rejected` | 403 for non-admin | API | None | - |
| 24 | Restore authorization | `test_restore_admin_only` | 403 for non-admin | API | None | - |
| 25 | Audit chain append-only | `test_audit_chain_hash_integrity`, `test_audit_entry_links_previous` | Hash chain verifiable | Unit | Tamper detection | Add hash-tamper-detect test |
| 26 | CSRF rejection | `test_post_without_csrf_rejected`, `test_post_with_valid_csrf_accepted` | 403 without token | API | None | - |
| 27 | Encryption round-trip | `test_encrypt_decrypt_roundtrip`, `test_ciphertext_not_plaintext` | Decrypt(Encrypt(x)) == x | Unit | None | - |
| 28 | Session idle timeout | `test_expired_token_rejected`, `test_idle_timeout_window`, `test_refreshed_token_has_newer_iat` | Token exp = iat + 12h; /me reissues with fresh iat | Unit + Integration | None | - |
| 29 | Ticket validity window | `test_ticket_validity_window_enforcement` | Redemption outside valid_from..valid_until rejected | Integration | None | - |
| 30 | Audit append-only | `test_audit_log_update_blocked_by_trigger`, `test_audit_log_delete_blocked_by_trigger`, `test_audit_log_append_only_triggers` | SQLite triggers block UPDATE/DELETE | Unit + Integration | None | - |
| 31 | Backup encryption | `test_backup_encryption_roundtrip` | encrypt_bytes/decrypt_bytes round-trip | Integration | Full file cycle | Manual: Docker backup path |
| 32 | CSRF session binding | `test_csrf_token_session_binding` | Tokens validated against server-side store | Integration | HTTP-level CSRF flow | Manual: CSRF from different session |
| 33 | Store isolation (reservations) | `test_store_isolation_enforcement` | MerchantStaff cannot access other store's assets | Integration | None | - |
| 34 | Ticket lookup by number | `resolve_ticket()` in tickets handler | Resolves by FR-XXXXXXXX or UUID | Code path | Manual: check-in UI | - |

## Test Organization

- **Unit tests**: `repo/backend/src/` inline `#[cfg(test)]` modules (password, session, csrf, encryption, masking, audit chain, reservation engine, ticket engine, uploads)
- **Integration tests**: `repo/backend/tests/integration_tests.rs` (full cycles: reservation+ticket, conflicts, undo, status transitions, audit, encryption, store isolation, validity window, CSRF)
- **Standalone test references**: `repo/unit_tests/` describes test mapping to inline modules
- **API test specifications**: `repo/API_tests/` documents expected HTTP-level behavior per endpoint

## Honest Gaps

1. **True concurrency testing**: SQLite under concurrent HTTP load cannot be fully tested in unit tests. Requires manual testing with parallel HTTP clients.
2. **QR code scanning**: Frontend QR rendering uses a deterministic SVG pattern, not a spec-compliant QR encoder. Camera-based scanning requires a real QR library (e.g. `qrcode` crate compiled to WASM) and a real device.
3. **Real browser rendering**: Leptos WASM rendering requires a browser or headless browser for full integration testing.
4. **Docker deployment**: Docker build and compose require Docker daemon, tested in CI or manually.
5. **HTTP-level CSRF flow**: CSRF is validated server-side per session in handlers, but the full HTTP request/response cycle (login -> store token -> POST with X-CSRF-Token) requires axum-test or manual testing.
6. **Full backup/restore cycle**: `encrypt_bytes`/`decrypt_bytes` round-trip is tested. Full backup via SQLite backup API -> encrypt -> write -> read -> decrypt -> restore is tested at the function level but requires filesystem access for the Docker path.
