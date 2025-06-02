use crate::cli::repositories::Repositories;

pub fn execute(index: Option<&String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nClone repo");
    let framework = Repositories::new();
    match index {
        Some(idx) => {
            let idx_num = idx.parse::<usize>()?;
            framework.clone_by_index(idx_num)?;
        },
        None => {
            let results = framework.clone_all();
            for result in results {
                if let Err(e) = result {
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)));
                }
            }
        },
    }
    Ok(())
}


