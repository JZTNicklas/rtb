use askama::Template;
use clap::Parser;
use std::path::PathBuf;

struct PathBufDisplay(PathBuf);

impl std::fmt::Display for PathBufDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self.0.to_str().unwrap();
        write!(f, "{}", path)
    }
}

impl std::clone::Clone for PathBufDisplay {
    fn clone(&self) -> Self {
        PathBufDisplay(self.0.clone())
    }
}

impl core::convert::From<PathBuf> for PathBufDisplay {
    fn from(path: PathBuf) -> Self {
        PathBufDisplay(path)
    }
}

impl core::convert::From<String> for PathBufDisplay {
    fn from(path: String) -> Self {
        PathBufDisplay(PathBuf::from(path))
    }
}

#[derive(Parser)]
struct CLI {
    /// Where to create the component
    filepath: String,
    #[arg(default_value_t = PathBufDisplay::from(PathBuf::from("./src/components")))]
    /// Base directory from where <FILEPATH> starts
    base_dir: PathBufDisplay,
    /// Overwrite file even if it already exists
    #[arg(short, long)]
    force: bool,
}
#[derive(Template)]
#[template(path = "component.html")]
struct ComponentTemplate<'a> {
    name: &'a String,
}

fn write_template(name: String, base_dir: PathBufDisplay, force: bool) {
    // Create entire path with file name
    // TODO: src/components is hard coded
    let mut path = PathBuf::from(base_dir.0);
    path.push(format!("{}.tsx", &name));

    // Clone without the file name, to create directory
    let mut dir = path.clone();
    dir.pop();

    // Create directory
    let dir_created = std::fs::create_dir_all(dir);
    match dir_created {
        Ok(_) => (),
        Err(_) => println!("Error while creating directory"),
    }

    // File already exists in folder. Required -f or --force to continue
    if path.exists() && !force {
        println!("File already exists. Run with -f or --force to overwrite.");
        return;
    }

    // Create template and write to file
    let component = ComponentTemplate { name: &name }; // Create template
    let res = std::fs::write(path, component.render().unwrap());
    match res {
        Ok(_) => println!("OK"),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    let args = CLI::parse();

    write_template(args.filepath, args.base_dir, args.force)
}
