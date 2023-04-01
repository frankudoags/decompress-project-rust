use std::time::Instant;
#[allow(unused_imports)]
use std::{
    fs,
    io::{self, Write},
};

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    // get args from command line
    let args: Vec<_> = std::env::args().collect();
    //  if there are no args, print usage and exit
    if args.len() != 2 {
        eprintln!("Usage: cargo run <filename to be decompressed>");
        return 1;
    };
    // extract the filename from the args
    let fname = std::path::Path::new(&args[1]);
    // open the file
    let file = fs::File::open(&fname).unwrap();

    //start timing;
    let start = Instant::now();
    // create a zip archive from the file
    let mut archive = zip::ZipArchive::new(file).unwrap();

    // iterate over the files in the archive
    for i in 0..archive.len() {
        // get the file
        let mut file = archive.by_index(i).unwrap();

        // ensure the file path is safe to use as a path, and if not, skip it and continue
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        // grab comments
        let comment = file.comment();
        // if there is a comment, print it
        if !comment.is_empty() {
            println!("File {} comment: {}", i, comment);
        }

        // if the file is a directory, create it
        if (file.name()).ends_with('/') {
            // create the directory
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );

            // ensure that for the current file, the parent directory exists
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            // create the file
            let mut outfile = fs::File::create(&outpath).unwrap();
            // copy the contents of the file
            io::copy(&mut file, &mut outfile).unwrap();
        }
        // print the time it took to decompress the file
    }
    println!("Time elapsed: {} secs", start.elapsed().as_secs());

    0
}
