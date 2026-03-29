use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct SkillMeta {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skill {
    pub meta: SkillMeta,
    pub body: String,
}

pub fn load_skills(dir: &Path) -> Result<Vec<Skill>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut skills = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        let content = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        skills.push(
            parse_skill(&content).with_context(|| format!("invalid skill {}", path.display()))?,
        );
    }

    skills.sort_by(|a, b| a.meta.name.cmp(&b.meta.name));
    Ok(skills)
}

pub fn selected_skill_prompts(skills: &[Skill], names: &[String]) -> Result<Vec<String>> {
    let mut prompts = Vec::new();
    for name in names {
        let skill = skills
            .iter()
            .find(|skill| skill.meta.name == *name)
            .with_context(|| format!("skill not found: {name}"))?;
        prompts.push(format!("Skill: {}\n{}", skill.meta.name, skill.body.trim()));
    }
    Ok(prompts)
}

pub fn parse_skill(input: &str) -> Result<Skill> {
    let mut parts = input.splitn(3, "---");
    let prefix = parts.next().unwrap_or_default();
    if !prefix.trim().is_empty() {
        bail!("skill file must start with YAML frontmatter");
    }

    let frontmatter = parts.next().context("missing YAML frontmatter")?;
    let body = parts.next().context("missing Markdown body")?;
    let meta: SkillMeta =
        serde_yaml::from_str(frontmatter).context("failed to parse YAML frontmatter")?;
    if meta.name.trim().is_empty() {
        bail!("skill name cannot be empty");
    }

    Ok(Skill {
        meta,
        body: body.trim().to_string(),
    })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{load_skills, parse_skill, selected_skill_prompts};

    #[test]
    fn parses_valid_skill() {
        let skill = parse_skill(
            "---\nname: rust\ndescription: Safe Rust\n---\n\nPrefer small functions.\n",
        )
        .unwrap();
        assert_eq!(skill.meta.name, "rust");
        assert_eq!(skill.meta.description.as_deref(), Some("Safe Rust"));
        assert_eq!(skill.body, "Prefer small functions.");
    }

    #[test]
    fn allows_missing_description() {
        let skill = parse_skill("---\nname: rust\n---\n\nPrefer std.\n").unwrap();
        assert_eq!(skill.meta.description, None);
    }

    #[test]
    fn rejects_missing_name() {
        let err = parse_skill("---\ndescription: Missing\n---\n\nBody\n").unwrap_err();
        assert!(err.to_string().contains("failed to parse YAML frontmatter"));
    }

    #[test]
    fn loads_directory_and_selects_prompt() {
        let dir = temp_path("skills");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("rust.md"),
            "---\nname: rust\n---\n\nPrefer safe Rust.\n",
        )
        .unwrap();

        let skills = load_skills(&dir).unwrap();
        let prompts = selected_skill_prompts(&skills, &[String::from("rust")]).unwrap();
        assert_eq!(
            prompts,
            vec![String::from("Skill: rust\nPrefer safe Rust.")]
        );

        fs::remove_dir_all(dir).unwrap();
    }

    fn temp_path(name: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("bubble-{name}-{nanos}"))
    }
}
