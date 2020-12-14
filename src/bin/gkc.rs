use std::{
    error::Error,
    fmt::Display,
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

use glimmerkit_build::{package::Package, stats::CollectedStats};
use glob::glob;
use pathdiff::diff_paths;
use yaml_rust::YamlLoader;

fn main() -> Result<(), Box<dyn Error>> {
    let root = git_root()?;

    let time = Instant::now();
    let mut stats = CollectedStats::new();

    for pkg in workspace(&root)? {
        stats.concat(pkg.vanilla()?);
    }

    let elapsed = time.elapsed();
    println!(
        "total    : {}.{} seconds",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );
    println!("");

    let total = stats.total();
    println!("compiled : {} files", total.files);
    println!(
        "elapsed  : {}.{} seconds",
        total.elapsed.as_secs(),
        total.elapsed.subsec_millis()
    );

    Ok(())
}

fn workspace(root: &Path) -> Result<Vec<Package>, Box<dyn Error>> {
    let file = fs::read_to_string(root.join("pnpm-workspace.yaml"))?;

    let yml = YamlLoader::load_from_str(&file)?;
    let pkgs = yml[0]["packages"].as_vec().ok_or(YamlError::new(
        "pnpm-workspace.yaml",
        "Couldn't find packages",
    ))?;

    let list = match yml[0]["skip-ts"].as_vec() {
        None => vec![],
        Some(v) => v.clone(),
    };

    let skip = list
        .iter()
        .map(|y| {
            y.as_str().ok_or(YamlError::new(
                "pnpm-workspace.yaml",
                "item in `skip-ts` key wasn't a string",
            ))
        })
        .collect::<Result<Vec<&str>, YamlError>>()?;

    let mut out = vec![];

    for pkg in pkgs {
        let pkg = match pkg.as_str() {
            None => {
                return Err(YamlError::new(
                    "pnpm-workspace.yaml",
                    "item in `packages` key wasn't a string",
                ))?
            }
            Some(v) => v,
        };

        if skip.contains(&pkg) {
            // println!("skipping {} (found in skip-ts)", pkg);
            continue;
        }

        // let walker = GlobWalkerBuilder::from_patterns(&root, &[pkg]).file_type(FileType::DIR);

        // println!("searching {}", root.join(pkg).display());

        for dir in glob(&format!("{}", root.join(pkg).display()))?.filter_map(Result::ok) {
            let path = match diff_paths(&dir, root) {
                None => {
                    return Err(UnexpectedErr::new(format!(
                        "{} wasn't nested in {}",
                        dir.display(),
                        root.display()
                    )))?
                }
                Some(p) => p,
            };

            out.push(Package::new(root, path.to_string_lossy().into_owned()));
        }
    }

    Ok(out)
}

#[derive(Debug)]
struct UnexpectedErr {
    string: String,
}

impl UnexpectedErr {
    fn new(string: impl Into<String>) -> UnexpectedErr {
        UnexpectedErr {
            string: string.into(),
        }
    }
}

impl Display for UnexpectedErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unexpected error: {}", self.string)
    }
}

impl Error for UnexpectedErr {}

#[derive(Debug)]
struct YamlError {
    file: String,
    problem: String,
}

impl YamlError {
    fn new(file: impl Into<String>, problem: impl Into<String>) -> YamlError {
        YamlError {
            file: file.into(),
            problem: problem.into(),
        }
    }
}

impl Display for YamlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "yaml error in {}: {}", self.file, self.problem)
    }
}

impl Error for YamlError {}

fn git_root() -> Result<PathBuf, Box<dyn Error>> {
    // println!(
    //     "{}",
    //     std::env::current_dir().expect("Couldn't get cwd").display()
    // );
    let root = std::env::current_dir()?;

    // let root = root
    //     .parent()
    //     .expect("Couldn't get parent directory")
    //     .canonicalize()
    //     .unwrap_or_else(|_| panic!("couldn't canonicalize {}", root.display()));

    for ancestor in root.ancestors() {
        if ancestor.join(".git").is_dir() {
            return Ok(ancestor.to_path_buf());
        }
    }

    Err(Box::new(NoRootError))
}

#[derive(Debug)]
struct NoRootError;

impl Display for NoRootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No git root found")
    }
}

impl Error for NoRootError {}
