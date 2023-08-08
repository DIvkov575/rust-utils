use futures::{stream, StreamExt};
use std::time::Duration;
use std::time::Instant;
use std::error::Error;
use std::fs::DirEntry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() >= 2);

    let paths = std::fs::read_dir(&args[1])?.map(|x| x.unwrap());
    let concurrency = 5;

    // concurrency starts here
    let start = Instant::now();
    stream::iter(paths)
        .for_each_concurrent(concurrency, |path| async move {
            let result = compute_task(path).await;
        })
        .await;
    let elapsed = start.elapsed();
    // concurrency ends


    println!("Execution time {:#?}", elapsed);
    println!("{}", args[1]);
    Ok(())
}


async fn compute_task(path: DirEntry) -> Result<(), Box<dyn Error>> {
    let file_base= path
        .path()
        .file_name()
        .unwrap()
        .to_owned()
        .into_string()
        .unwrap();

    if file_base.starts_with("Copy of ") {
        let output_filename = path
            .path()
            .to_str()
            .unwrap()
            .replace("Copy of ", "");
        std::fs::rename(path.path(), output_filename)?;
    }

    Ok(())
}


// list directories
// let a = std::env::current_dir().unwrap();
// let paths = std::fs::read_dir(a).unwrap();
// for path in paths {
// println!("{:#?}", path.unwrap().path())


// fn main() -> Result<(), std::io::Error> {
