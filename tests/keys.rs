//! Guards the pairing between each `UnionMessage::KEY` const and the type's own `name()`.
//! Stage 2's round-trip test compares the emitted key against the same const it was built from,
//! so it stays green even if an impl is wired to the wrong const (e.g. `InitData` -> `PEGIN_REQUEST`).
//! `name()` is derived independently, so this is the only mechanical guard against that mis-pairing.

use bitvmx_client_types::{
    AdvanceFundsRequest, Committee, DisputeCoreData, FullPenalizationData, FundsAdvanceSPV,
    FundsAdvanced, InitData, PegInRequest, PegOutAccepted, PegOutRequest, RejectPeginData,
    UnionMessage, UnionSPVNotification, UnionSettings,
};

#[test]
fn key_matches_name() {
    assert_eq!(Committee::KEY, Committee::name());
    assert_eq!(DisputeCoreData::KEY, DisputeCoreData::name());
    assert_eq!(InitData::KEY, InitData::name());
    assert_eq!(PegInRequest::KEY, PegInRequest::name());
    assert_eq!(RejectPeginData::KEY, RejectPeginData::name());
    assert_eq!(PegOutRequest::KEY, PegOutRequest::name());
    assert_eq!(PegOutAccepted::KEY, PegOutAccepted::name());
    assert_eq!(AdvanceFundsRequest::KEY, AdvanceFundsRequest::name());
    assert_eq!(FundsAdvanced::KEY, FundsAdvanced::name());
    assert_eq!(FundsAdvanceSPV::KEY, FundsAdvanceSPV::name());
    assert_eq!(UnionSPVNotification::KEY, UnionSPVNotification::name());
    assert_eq!(FullPenalizationData::KEY, FullPenalizationData::name());
    assert_eq!(UnionSettings::KEY, UnionSettings::name());
}
