use bio::io::fastq;
use csv;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 5 {
        eprintln!("Usage: fastq_read -input fastqfile.fastq.gz -output csvfile.csv");
        std::process::exit(1);
    }

    // Get the input and output file paths
    let input_path = Path::new(&args[2]);
    let output_path = Path::new(&args[4]);

    // Call the extract_bases function with the file paths
    extract_bases(input_path, output_path)?;

    Ok(())
}

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