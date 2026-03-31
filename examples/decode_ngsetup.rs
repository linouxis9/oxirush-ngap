//! Decode an NGSetupResponse from APER bytes, inspect its contents,
//! and re-encode it back to APER.

use asn1_codecs::aper::AperCodec;
use asn1_codecs::PerCodecData;
use oxirush_ngap::ngap::*;

fn main() {
    // A captured NGSetupResponse (APER-encoded)
    // Contains: AMF Name, Served GUAMI List, Relative AMF Capacity, PLMN Support List
    let aper_hex = concat!(
        "2015",     // SuccessfulOutcome, procedure code 21 (NGSetup)
        "0026",     // length
        "000004",   // 4 IEs
        "0001", "0009", "0008", "80", "4f70656e354753", // AMF Name = "Open5GS"
        "0060", "0006", "0005", "00", "02f839", "4000", // Served GUAMI: PLMN=208/93, AMF Region=0x40, Set=0, Pointer=0
        "0056", "0001", "00", "ff",                     // Relative AMF Capacity = 255
        "005e", "0008", "0007", "00", "02f839", "0001", "0801", // PLMN Support: PLMN=208/93, S-NSSAI (SST=1, SD=000001)
    );
    let aper_bytes = hex::decode(aper_hex).expect("invalid hex");

    // Decode
    let mut codec_data = PerCodecData::from_slice_aper(&aper_bytes);
    let pdu = NGAP_PDU::aper_decode(&mut codec_data).expect("APER decode failed");

    println!("=== Decoded NGAP PDU ===\n");

    match &pdu {
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
                        }
                        _ => {
                            println!("  (other IE: id={})", ie.id.0);
                        }
                    }
                }
            }
        }
        NGAP_PDU::InitiatingMessage(_) => println!("InitiatingMessage"),
        NGAP_PDU::UnsuccessfulOutcome(_) => println!("UnsuccessfulOutcome"),
    }

    // Re-encode to APER
    let mut output = PerCodecData::new_aper();
    pdu.aper_encode(&mut output).expect("APER encode failed");
    let re_encoded = output.into_bytes();

    println!(
        "\nRe-encoded: {} bytes (original: {} bytes)",
        re_encoded.len(),
        aper_bytes.len()
    );
}
