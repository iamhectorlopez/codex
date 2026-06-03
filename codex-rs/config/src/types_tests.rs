use super::*;
use pretty_assertions::assert_eq;
use std::collections::HashMap;

#[test]
fn deserialize_skill_config_with_name_selector() {
    let cfg: SkillConfig = toml::from_str(
        r#"
            name = "github:yeet"
            enabled = false
        "#,
    )
    .expect("should deserialize skill config with name selector");

    assert_eq!(cfg.name.as_deref(), Some("github:yeet"));
    assert_eq!(cfg.path, None);
    assert!(!cfg.enabled);
}

#[test]
fn deserialize_skill_config_with_path_selector() {
    let tempdir = tempfile::tempdir().expect("tempdir");
    let skill_path = tempdir.path().join("skills").join("demo").join("SKILL.md");
    let cfg: SkillConfig = toml::from_str(&format!(
        r#"
            path = {path:?}
            enabled = false
        "#,
        path = skill_path.display().to_string(),
    ))
    .expect("should deserialize skill config with path selector");

    assert_eq!(
        cfg,
        SkillConfig {
            path: Some(
                AbsolutePathBuf::from_absolute_path(&skill_path)
                    .expect("skill path should be absolute"),
            ),
            name: None,
            enabled: false,
        }
    );
}

#[test]
fn memories_config_clamps_count_limits_to_nonzero_values() {
    let config = MemoriesConfig::from(MemoriesToml {
        max_raw_memories_for_consolidation: Some(0),
        max_rollouts_per_startup: Some(0),
        ..Default::default()
    });

    assert_eq!(
        config,
        MemoriesConfig {
            max_raw_memories_for_consolidation: 1,
            max_rollouts_per_startup: 1,
            ..MemoriesConfig::default()
        }
    );
}

#[test]
fn memories_config_clamps_rate_limit_remaining_threshold() {
    let config = MemoriesConfig::from(MemoriesToml {
        min_rate_limit_remaining_percent: Some(101),
        ..Default::default()
    });
    assert_eq!(
        config,
        MemoriesConfig {
            min_rate_limit_remaining_percent: 100,
            ..MemoriesConfig::default()
        }
    );

    let config = MemoriesConfig::from(MemoriesToml {
        min_rate_limit_remaining_percent: Some(-1),
        ..Default::default()
    });
    assert_eq!(
        config,
        MemoriesConfig {
            min_rate_limit_remaining_percent: 0,
            ..MemoriesConfig::default()
        }
    );
}

#[test]
fn deserialize_apps_config_with_account_aliases() {
    let cfg: AppsConfigToml = toml::from_str(
        r#"
            [connector_gmail]
            default_account = "personal"
            ask_account_when_unspecified = false

            [connector_gmail.accounts.personal]
            name = "Personal"
            description = "Use for family and personal email."

            [connector_gmail.accounts.work]
            name = "Work"
            description = "Use for company mail."
            default = true
        "#,
    )
    .expect("should deserialize app account aliases");

    assert_eq!(
        cfg,
        AppsConfigToml {
            default: None,
            apps: HashMap::from([(
                "connector_gmail".to_string(),
                AppConfig {
                    enabled: true,
                    default_account: Some("personal".to_string()),
                    ask_account_when_unspecified: Some(false),
                    accounts: HashMap::from([
                        (
                            "personal".to_string(),
                            AppAccountConfig {
                                name: Some("Personal".to_string()),
                                description: Some("Use for family and personal email.".to_string()),
                                default: false,
                            },
                        ),
                        (
                            "work".to_string(),
                            AppAccountConfig {
                                name: Some("Work".to_string()),
                                description: Some("Use for company mail.".to_string()),
                                default: true,
                            },
                        ),
                    ]),
                    ..Default::default()
                },
            )]),
        }
    );
}
