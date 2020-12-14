use std::{error::Error, fs, path::PathBuf};

use relative_path::{RelativePath, RelativePathBuf};

use crate::stats::Stat;
use crate::{module::ModuleFile, stats::StatStart};

pub struct Package {
    repo_root: PathBuf,
    pkg_root: RelativePathBuf,
}

impl Package {
    pub fn new(repo_root: impl Into<PathBuf>, pkg_root: impl Into<RelativePathBuf>) -> Package {
        Package {
            repo_root: repo_root.into(),
            pkg_root: pkg_root.into(),
        }
    }

    pub fn vanilla(&self) -> Result<Vec<Stat>, Box<dyn Error>> {
        let Self {
            repo_root,
            pkg_root,
        } = self;

        let mut stats: Vec<Stat> = vec![];

        let out = pkg_root.join("dist");

        let pkg_path = pkg_root.to_path(&repo_root);
        let out_path = out.to_path(&repo_root);

        if out_path.is_dir() {
            fs::remove_dir_all(out_path)?;
        } else if out_path.exists() {
            panic!("{} exists, but is not a directory", out_path.display());
        }

        let walker = globwalk::GlobWalkerBuilder::from_patterns(&pkg_path, &["**/*.ts"]);

        for file in walker.build()?.filter_map(Result::ok) {
            let file_name = file.path();

            let stat = StatStart::new(file_name);

            let relative =
                pathdiff::diff_paths(file_name, &pkg_path).expect("Expected relative path");

            let out_file = out
                .join(RelativePath::from_path(&relative).unwrap())
                .to_path(&repo_root)
                .with_extension("js");

            let module = ModuleFile::new(file_name);
            let buf = module.parse().emit();

            fs::create_dir_all(out_file.parent().unwrap())?;

            fs::write(out_file, &buf)?;

            stats.push(stat.done());
        }

        Ok(stats)
    }
}
