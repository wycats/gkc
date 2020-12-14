use derive_new::new;
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

pub struct StatStart {
    file: PathBuf,
    start: Instant,
}

impl StatStart {
    pub fn new(path: impl Into<PathBuf>) -> StatStart {
        StatStart {
            file: path.into(),
            start: Instant::now(),
        }
    }

    pub fn done(self) -> Stat {
        Stat {
            file: self.file,
            elapsed: self.start.elapsed(),
        }
    }
}

pub struct Stat {
    file: PathBuf,
    elapsed: Duration,
}

impl Stat {
    pub fn new(path: impl Into<PathBuf>, elapsed: impl Into<Duration>) -> Stat {
        Stat {
            file: path.into(),
            elapsed: elapsed.into(),
        }
    }
}

#[derive(new)]
pub struct CollectedStats {
    #[new(default)]
    emitted: Vec<Stat>,
}

impl CollectedStats {
    pub fn add(&mut self, stat: Stat) {
        self.emitted.push(stat);
    }

    pub fn concat(&mut self, stats: Vec<Stat>) {
        self.emitted.extend(stats);
    }

    pub fn total(&self) -> Stats {
        let mut elapsed: Duration = Duration::from_secs(0);

        for stat in &self.emitted {
            elapsed = elapsed + stat.elapsed;
        }

        Stats {
            files: self.emitted.len(),
            elapsed,
        }
    }
}

pub struct Stats {
    pub files: usize,
    pub elapsed: Duration,
}
