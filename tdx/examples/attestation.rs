use dcap_rs::types::quotes::version_4::QuoteV4;
use tdx::Tdx;

fn main() {
    // Initialise a TDX object
    let tdx = Tdx::new();

    // Retrieve an attestation report with default options passed to the hardware device
    let raw_report = tdx.get_attestation_report_raw().unwrap();
    let report = QuoteV4::from_bytes(&raw_report);
    println!(
        "Attestation Report raw bytes: 0x{}",
        hex::encode(raw_report)
    );
    println!("Attestation Report : {:?}", report);
}
