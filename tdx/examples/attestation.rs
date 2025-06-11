use tdx::Tdx;

fn main() {
    // Initialise a TDX object
    let tdx = Tdx::new();

    // Retrieve an attestation report with default options passed to the hardware device
    let (report, _) = tdx.get_attestation_report().unwrap();

    println!("Attestation Report: {:?}", report);

    // Verify the attestation report
    tdx.verify_attestation_report(&report).unwrap();

    println!("Verification successful!");
}
