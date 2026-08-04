//! Pins the nine derived program ids to literal UUIDs. Tier-3 drift (a changed salt string or
//! hash input order) produces no runtime error anywhere in the protocol — the program is simply
//! never found. This test is the only alarm for that class of bug.
//!
//! The literal UUIDs below were captured from this test's own failure output, never computed by
//! hand and never copied from the union repo.

use std::str::FromStr;

use bitcoin::PublicKey;
use bitvmx_client_types::{
    get_accept_pegin_pid, get_advance_funds_pid, get_dispute_aggregated_key_pid,
    get_dispute_channel_pid, get_dispute_core_pid, get_dispute_pair_aggregated_key_pid,
    get_full_penalization_pid, get_take_aggreated_key_pid, get_user_take_pid,
};
use uuid::Uuid;

fn committee_id() -> Uuid {
    Uuid::from_str("00000000-0000-0000-0000-000000000042").unwrap()
}

fn pubkey() -> PublicKey {
    PublicKey::from_str("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")
        .unwrap()
}

#[test]
fn dispute_core_pid_is_pinned() {
    let actual = get_dispute_core_pid(committee_id(), &pubkey());
    assert_eq!(actual.to_string(), "56c7752e-956c-da39-58fb-b68e3a6ef484");
}

#[test]
fn accept_pegin_pid_is_pinned() {
    let actual = get_accept_pegin_pid(committee_id(), 7);
    assert_eq!(actual.to_string(), "60a1d53c-0d8c-1832-e2ac-0b446cc22ef0");
}

#[test]
fn advance_funds_pid_is_pinned() {
    let actual = get_advance_funds_pid(committee_id(), 7);
    assert_eq!(actual.to_string(), "29c4f6fa-a06f-bd4f-aae5-ed1e30a1da8e");
}

#[test]
fn user_take_pid_is_pinned() {
    let actual = get_user_take_pid(committee_id(), 7);
    assert_eq!(actual.to_string(), "a5aa3764-3044-7e54-faf8-fefb23c223d0");
}

#[test]
fn take_aggreated_key_pid_is_pinned() {
    let actual = get_take_aggreated_key_pid(committee_id());
    assert_eq!(actual.to_string(), "f203b47d-f062-1c1d-947f-47669a59b1bb");
}

#[test]
fn dispute_aggregated_key_pid_is_pinned() {
    let actual = get_dispute_aggregated_key_pid(committee_id());
    assert_eq!(actual.to_string(), "2487b8b7-f77f-3a5e-b5e1-d0d28d945fe0");
}

#[test]
fn dispute_pair_aggregated_key_pid_is_pinned() {
    let actual = get_dispute_pair_aggregated_key_pid(committee_id(), 3, 9);
    assert_eq!(actual.to_string(), "05a5559e-90f1-3ba0-a141-e6d8bdc8f1b1");
}

#[test]
fn dispute_channel_pid_is_pinned() {
    let actual = get_dispute_channel_pid(committee_id(), 3, 9);
    assert_eq!(actual.to_string(), "eef6f9eb-bfed-cce5-d570-67fbaf5402ae");
}

#[test]
fn full_penalization_pid_is_pinned() {
    let actual = get_full_penalization_pid(committee_id());
    assert_eq!(actual.to_string(), "c232abbd-92dc-91ab-703a-102d8b0b3b1d");
}
