use bio::io::fastq;
use csv;
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn extract_bases(fastq_path: &Path, csv_path: &Path) -> Result<(), Box<dyn Error>> {
    // Open the FASTQ file
    let reader = fastq::Reader::from_file(fastq_path)?;

    // Create a CSV writer
    let mut writer = csv::Writer::from_path(csv_path)?;

    // Iterate over the records
    for result in reader.records() {
        let record = result?;
        let sequence = record.seq();

        // Extract the first 10 bases
        let bases = &sequence[0..10];

        // Write the bases to the CSV file
        writer.write_record(&[std::str::from_utf8(bases)?])?;
    }

    // Flush the writer to ensure all data is written to the file
    writer.flush()?;

    Ok(())
}