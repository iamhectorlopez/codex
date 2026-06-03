use codex_app_server_protocol::AppInfo;
use codex_mcp::CODEX_APPS_MCP_SERVER_NAME;
use codex_protocol::protocol::APPS_INSTRUCTIONS_CLOSE_TAG;
use codex_protocol::protocol::APPS_INSTRUCTIONS_OPEN_TAG;

use crate::connectors::AppAccountSelectionGuidance;

use super::ContextualUserFragment;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AppsInstructions {
    account_guidance: Vec<AppAccountSelectionGuidance>,
}

impl AppsInstructions {
    pub(crate) fn from_connectors_with_account_guidance(
        connectors: &[AppInfo],
        account_guidance: Vec<AppAccountSelectionGuidance>,
    ) -> Option<Self> {
        connectors
            .iter()
            .any(|connector| connector.is_accessible && connector.is_enabled)
            .then_some(Self { account_guidance })
    }
}

impl ContextualUserFragment for AppsInstructions {
    fn role(&self) -> &'static str {
        "developer"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        (APPS_INSTRUCTIONS_OPEN_TAG, APPS_INSTRUCTIONS_CLOSE_TAG)
    }

    fn body(&self) -> String {
        let mut body = format!(
            "\n## Apps (Connectors)\nApps (Connectors) can be explicitly triggered in user messages in the format `[$app-name](app://{{connector_id}})`. Apps can also be implicitly triggered as long as the context suggests usage of available apps.\nAn app is equivalent to a set of MCP tools within the `{CODEX_APPS_MCP_SERVER_NAME}` MCP.\nAn installed app's MCP tools are either provided to you already, or can be lazy-loaded through the `tool_search` tool. If `tool_search` is available, the apps that are searchable by `tools_search` will be listed by it.\nDo not additionally call list_mcp_resources or list_mcp_resource_templates for apps.\n"
        );

        if !self.account_guidance.is_empty() {
            body.push_str(
                "Some apps have configured account aliases. Treat these aliases as user-provided routing guidance for the connected account to use; they do not create new credentials or OAuth sessions. If the user names one of these aliases, follow it. If the user does not specify an account and the rule says to ask, ask a brief clarification before using the app.\n",
            );
            body.push_str("Configured app accounts:\n");
            for app in &self.account_guidance {
                let accounts = app
                    .accounts
                    .iter()
                    .map(|account| {
                        let mut account_text = format!("`{}`", account.name);
                        if account.key != account.name {
                            account_text.push_str(&format!(" (alias `{}`)", account.key));
                        }
                        if account.is_default {
                            account_text.push_str(" (default)");
                        }
                        if let Some(description) = account.description.as_deref() {
                            account_text.push_str(&format!(" - {description}"));
                        }
                        account_text
                    })
                    .collect::<Vec<_>>()
                    .join("; ");
                body.push_str(&format!(
                    "- `{}` (`{}`): {}.\n",
                    app.app_name, app.app_id, accounts
                ));

                if app.ask_when_unspecified {
                    body.push_str(
                        "  Rule: if the user does not specify one of these account aliases, ask which account to use before calling this app.\n",
                    );
                } else if let Some(default_account) =
                    app.accounts.iter().find(|account| account.is_default)
                {
                    body.push_str(&format!(
                        "  Rule: if the user does not specify an account, use `{}`.\n",
                        default_account.name
                    ));
                }
            }
        }

        body
    }
}
