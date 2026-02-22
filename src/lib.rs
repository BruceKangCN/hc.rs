use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[clap(about = "HTTP heath check", version)]
pub struct Args {
    #[arg(short, long)]
    pub count: Option<usize>,

    #[arg(short, long)]
    pub interval: Option<u64>,

    pub end_point: String,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Statistics {
    /// successful response count
    pub success: usize,

    /// error response count
    pub error: usize,

    /// connection failure count
    pub failure: usize,
}

impl Statistics {
    pub fn total(&self) -> usize {
        self.success + self.error + self.failure
    }

    pub fn health_rate(&self) -> f64 {
        (self.success as f64) / (self.total() as f64)
    }

    pub fn fail_rate(&self) -> f64 {
        (self.failure as f64) / (self.total() as f64)
    }
}
