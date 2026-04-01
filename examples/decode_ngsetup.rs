//! Build an NGSetupResponse, encode it to APER, then decode and inspect it.
//!
//! Demonstrates round-trip APER encoding/decoding of NGAP PDUs.

use asn1_codecs::PerCodecData;
use asn1_codecs::aper::AperCodec;
use oxirush_ngap::ngap::*;

fn main() {
    // Build an NGSetupResponse programmatically using auto-generated types.
    // No raw hex — everything is constructed from typed NGAP structs.
    use bitvec::prelude::*;

    // AMF Region ID = 1 (8 bits), AMF Set ID = 1 (10 bits), AMF Pointer = 0 (6 bits)
    let mut region_bv = BitVec::<u8, Msb0>::from_slice(&[0x01]); // 8 bits: 00000001
    region_bv.truncate(8);
    let mut set_bv = BitVec::<u8, Msb0>::from_slice(&[0x00, 0x40]); // 10 bits: 0000000001
    set_bv.truncate(10);
    let mut ptr_bv = BitVec::<u8, Msb0>::from_slice(&[0x00]); // 6 bits: 000000
    ptr_bv.truncate(6);

    let guami = GUAMI {
        plmn_identity: PLMNIdentity(vec![0x02, 0xF8, 0x39]), // MCC=208, MNC=93
        amf_region_id: AMFRegionID(region_bv),
        amf_set_id: AMFSetID(set_bv),
        amf_pointer: AMFPointer(ptr_bv),
        ie_extensions: None,
    };

    let ies = vec![
        NGSetupResponseProtocolIEs_Entry {
            id: ProtocolIE_ID(ID_AMF_NAME),
            criticality: Criticality(Criticality::REJECT),
            value: NGSetupResponseProtocolIEs_EntryValue::Id_AMFName(AMFName(
                "OxiRushAMF".to_string(),
            )),
        },
        NGSetupResponseProtocolIEs_Entry {
            id: ProtocolIE_ID(ID_SERVED_GUAMI_LIST),
            criticality: Criticality(Criticality::REJECT),
            value: NGSetupResponseProtocolIEs_EntryValue::Id_ServedGUAMIList(ServedGUAMIList(
                vec![ServedGUAMIItem {
                    guami,
                    backup_amf_name: None,
                    ie_extensions: None,
                }],
            )),
        },
        NGSetupResponseProtocolIEs_Entry {
            id: ProtocolIE_ID(ID_RELATIVE_AMF_CAPACITY),
            criticality: Criticality(Criticality::IGNORE),
            value: NGSetupResponseProtocolIEs_EntryValue::Id_RelativeAMFCapacity(
                RelativeAMFCapacity(255),
            ),
        },
        NGSetupResponseProtocolIEs_Entry {
            id: ProtocolIE_ID(ID_PLMN_SUPPORT_LIST),
            criticality: Criticality(Criticality::REJECT),
            value: NGSetupResponseProtocolIEs_EntryValue::Id_PLMNSupportList(PLMNSupportList(
                vec![PLMNSupportItem {
                    plmn_identity: PLMNIdentity(vec![0x02, 0xF8, 0x39]),
                    slice_support_list: SliceSupportList(vec![SliceSupportItem {
                        s_nssai: S_NSSAI {
                            sst: SST(vec![1]),
                            sd: Some(SD(vec![0x00, 0x00, 0x01])),
                            ie_extensions: None,
                        },
                        ie_extensions: None,
                    }]),
                    ie_extensions: None,
                }],
            )),
        },
    ];

    let pdu = NGAP_PDU::SuccessfulOutcome(SuccessfulOutcome {
        procedure_code: ProcedureCode(ID_NG_SETUP),
        criticality: Criticality(Criticality::REJECT),
        value: SuccessfulOutcomeValue::Id_NGSetup(NGSetupResponse {
            protocol_i_es: NGSetupResponseProtocolIEs(ies),
        }),
    });

    // Encode to APER
    let mut output = PerCodecData::new_aper();
    pdu.aper_encode(&mut output).expect("APER encode failed");
    let aper_bytes = output.into_bytes();
    println!("Encoded NGSetupResponse: {} bytes", aper_bytes.len());
    println!("APER hex: {}", hex::encode(&aper_bytes));

    // Decode back from APER
    let mut codec_data = PerCodecData::from_slice_aper(&aper_bytes);
    let decoded = NGAP_PDU::aper_decode(&mut codec_data).expect("APER decode failed");

    println!("\n=== Decoded NGAP PDU ===\n");

    match &decoded {
        NGAP_PDU::SuccessfulOutcome(outcome) => {
            println!("Type:           SuccessfulOutcome");
            println!("Procedure code: {}", outcome.procedure_code.0);
            println!("Criticality:    {:?}", outcome.criticality.0);

            if let SuccessfulOutcomeValue::Id_NGSetup(ref resp) = outcome.value {
                for ie in &resp.protocol_i_es.0 {
                    match &ie.value {
                        NGSetupResponseProtocolIEs_EntryValue::Id_AMFName(name) => {
                            println!("AMF Name:       {}", name.0);
                        }
                        NGSetupResponseProtocolIEs_EntryValue::Id_RelativeAMFCapacity(cap) => {
                            println!("AMF Capacity:   {}", cap.0);
                        }
                        NGSetupResponseProtocolIEs_EntryValue::Id_ServedGUAMIList(list) => {
                            println!("Served GUAMIs:  {} entries", list.0.len());
                            for guami_item in &list.0 {
                                let plmn = &guami_item.guami.plmn_identity.0;
                                println!(
                                    "  PLMN: {}",
                                    plmn.iter().map(|b| format!("{b:02x}")).collect::<String>()
                                );
                            }
                        }
                        NGSetupResponseProtocolIEs_EntryValue::Id_PLMNSupportList(list) => {
                            println!("PLMN Support:   {} entries", list.0.len());
                            for item in &list.0 {
                                let plmn = &item.plmn_identity.0;
                                println!(
                                    "  PLMN: {}, slices: {}",
                                    plmn.iter().map(|b| format!("{b:02x}")).collect::<String>(),
                                    item.slice_support_list.0.len()
                                );
                            }
                        }
                        _ => {
                            println!("  (other IE: id={})", ie.id.0);
                        }
                    }
                }
            }
        }
        _ => println!("Unexpected PDU type"),
    }

    // Verify round-trip
    let mut re_output = PerCodecData::new_aper();
    decoded
        .aper_encode(&mut re_output)
        .expect("re-encode failed");
    let re_encoded = re_output.into_bytes();
    assert_eq!(aper_bytes, re_encoded, "Round-trip mismatch!");
    println!("\nRound-trip OK ({} bytes)", re_encoded.len());
}
