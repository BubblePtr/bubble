use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};

#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub cwd: PathBuf,
    pub enabled_skills: Vec<String>,
}

impl Config {
    pub fn from_env_and_args() -> Result<Self> {
        let mut args = env::args().skip(1);
        let mut enabled_skills = Vec::new();
        let mut cwd = env::current_dir().context("failed to read current directory")?;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--skill" | "-s" => {
                    let value = args.next().context("missing value after --skill")?;
                    enabled_skills.push(value);
                }
                "--cwd" => {
                    let value = args.next().context("missing value after --cwd")?;
                    cwd = PathBuf::from(value);
                }
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                other => bail!("unknown argument: {other}"),
            }
        }

        if enabled_skills.is_empty() {
            enabled_skills = env::var("BUBBLE_SKILLS")
                .unwrap_or_default()
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(ToOwned::to_owned)
                .collect();
        }

        Ok(Self {
            base_url: env::var("OPENAI_BASE_URL")
                .context("OPENAI_BASE_URL is required, for example https://api.openai.com/v1")?,
            api_key: env::var("OPENAI_API_KEY").context("OPENAI_API_KEY is required")?,
            model: env::var("OPENAI_MODEL").context("OPENAI_MODEL is required")?,
            cwd,
            enabled_skills,
        })
    }
}

fn print_help() {
    println!("Usage: bubble [--skill <name>]... [--cwd <path>]");
}
