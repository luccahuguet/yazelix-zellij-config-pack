use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

pub const RENDERER_SCHEMA_VERSION: u64 = 1;

const GENERATED_LAYOUT_MARKER: &str = "GENERATED ZELLIJ LAYOUT (YAZELIX)";
const GENERATED_LAYOUT_FINGERPRINT_PREFIX: &str = "generation_fingerprint:";
const ZJSTATUS_TAB_TEMPLATE_PLACEHOLDER: &str = "__YAZELIX_ZJSTATUS_TAB_TEMPLATE__";
const PANE_ORCHESTRATOR_PLUGIN_URL_PLACEHOLDER: &str = "__YAZELIX_PANE_ORCHESTRATOR_PLUGIN_URL__";
const HOME_DIR_PLACEHOLDER: &str = "__YAZELIX_HOME_DIR__";
const RUNTIME_DIR_PLACEHOLDER: &str = "__YAZELIX_RUNTIME_DIR__";
const PANE_ORCHESTRATOR_PLUGIN_ALIAS: &str = "yazelix_pane_orchestrator";
const YZPP_PLUGIN_ALIAS: &str = "yzpp";
const BOTTOM_POPUP_COMMAND_KEY: &str = "bottom_popup";
const TOP_POPUP_COMMAND_KEY: &str = "top_popup";
const MENU_POPUP_COMMAND_KEY: &str = "menu";

const REQUIRED_LAYOUT_PLACEHOLDERS: &[&str] = &[
    ZJSTATUS_TAB_TEMPLATE_PLACEHOLDER,
    PANE_ORCHESTRATOR_PLUGIN_URL_PLACEHOLDER,
    HOME_DIR_PLACEHOLDER,
    RUNTIME_DIR_PLACEHOLDER,
    "__YAZELIX_SIDEBAR_COMMAND__",
    "__YAZELIX_SIDEBAR_ARGS__",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZellijConfigPackRenderRequest {
    pub base_config_content: String,
    pub override_keybinds: Vec<String>,
    pub render_plan: ZellijRenderPlanData,
    pub popup_commands: BTreeMap<String, Vec<String>>,
    pub custom_popups: Vec<CustomPopup>,
    #[serde(default)]
    pub layout_templates: Option<Vec<ZellijConfigPackLayoutTemplate>>,
    #[serde(default)]
    pub static_fragments: Option<BTreeMap<String, String>>,
    pub zjstatus_plugin_block: String,
    pub pane_orchestrator_plugin_url: String,
    pub yzpp_plugin_url: String,
    pub home_dir: String,
    pub runtime_dir: String,
    pub generation_fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZellijRenderPlanData {
    pub owned_top_level_setting_names: Vec<String>,
    pub dynamic_top_level_settings: Vec<TopLevelSetting>,
    pub enforced_top_level_settings: Vec<TopLevelSetting>,
    pub rounded_value: String,
    pub popup_width_percent: i64,
    pub popup_height_percent: i64,
    pub screen_saver_enabled: bool,
    pub screen_saver_idle_seconds: i64,
    pub screen_saver_style: String,
    pub right_sidebar_command: String,
    pub right_sidebar_args: Vec<String>,
    pub left_sidebar_command: String,
    pub left_sidebar_args: Vec<String>,
    pub layout_percentages: ZellijLayoutPercentages,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopLevelSetting {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZellijLayoutPercentages {
    pub left_sidebar_width_percent: String,
    pub right_sidebar_width_percent: String,
    pub open_content_width_percent: String,
    pub closed_content_width_percent: String,
    pub left_open_right_open_content_width_percent: String,
    pub left_open_right_closed_content_width_percent: String,
    pub left_closed_right_open_content_width_percent: String,
    pub left_closed_right_closed_content_width_percent: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomPopup {
    pub id: String,
    pub command: Vec<String>,
    #[serde(default)]
    pub keybindings: Vec<String>,
    pub keep_alive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZellijConfigPackLayoutTemplate {
    pub relative_path: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZellijConfigPackRenderedFile {
    pub relative_path: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZellijConfigPackRenderOutput {
    pub renderer_schema_version: u64,
    pub merged_config: String,
    pub layout_files: Vec<ZellijConfigPackRenderedFile>,
    pub generation_fingerprint: String,
}

pub fn render_zellij_config_pack(
    request: &ZellijConfigPackRenderRequest,
) -> Result<ZellijConfigPackRenderOutput, String> {
    Ok(ZellijConfigPackRenderOutput {
        renderer_schema_version: RENDERER_SCHEMA_VERSION,
        merged_config: render_merged_config(request),
        layout_files: render_config_pack_layouts(request)?,
        generation_fingerprint: request.generation_fingerprint.clone(),
    })
}

pub fn bundled_layout_templates() -> Vec<ZellijConfigPackLayoutTemplate> {
    vec![
        ZellijConfigPackLayoutTemplate {
            relative_path: "yzx_side.kdl".to_string(),
            content: include_str!("../layouts/yzx_side.kdl").to_string(),
        },
        ZellijConfigPackLayoutTemplate {
            relative_path: "yzx_side.swap.kdl".to_string(),
            content: include_str!("../layouts/yzx_side.swap.kdl").to_string(),
        },
    ]
}

pub fn bundled_static_fragments() -> BTreeMap<String, String> {
    [
        (
            "__YAZELIX_SWAP_SIDEBAR_OPEN__",
            include_str!("../layouts/fragments/swap_sidebar_open.kdl"),
        ),
        (
            "__YAZELIX_SWAP_SIDEBAR_CLOSED__",
            include_str!("../layouts/fragments/swap_sidebar_closed.kdl"),
        ),
        (
            "__YAZELIX_SWAP_AGENT_OPEN__",
            include_str!("../layouts/fragments/swap_agent_open.kdl"),
        ),
        (
            "__YAZELIX_SWAP_AGENT_CLOSED__",
            include_str!("../layouts/fragments/swap_agent_closed.kdl"),
        ),
    ]
    .into_iter()
    .map(|(placeholder, content)| (placeholder.to_string(), content.to_string()))
    .collect()
}

fn render_config_pack_layouts(
    request: &ZellijConfigPackRenderRequest,
) -> Result<Vec<ZellijConfigPackRenderedFile>, String> {
    let bundled_templates;
    let templates = if let Some(templates) = &request.layout_templates {
        templates
    } else {
        bundled_templates = bundled_layout_templates();
        &bundled_templates
    };
    let bundled_fragments;
    let fragments = if let Some(fragments) = &request.static_fragments {
        fragments
    } else {
        bundled_fragments = bundled_static_fragments();
        &bundled_fragments
    };

    templates
        .iter()
        .map(|template| {
            let rendered = render_layout_template(
                &template.content,
                fragments,
                &request.zjstatus_plugin_block,
                &request.pane_orchestrator_plugin_url,
                &request.home_dir,
                &request.runtime_dir,
                &request.render_plan,
            )?;
            Ok(ZellijConfigPackRenderedFile {
                relative_path: template.relative_path.clone(),
                content: format!(
                    "{}{}",
                    generated_zellij_layout_header(&request.generation_fingerprint),
                    rendered
                ),
            })
        })
        .collect()
}

fn render_merged_config(request: &ZellijConfigPackRenderRequest) -> String {
    let extracted_blocks = extract_semantic_config_blocks(&request.base_config_content);
    let base_config = strip_owned_top_level_settings(
        &extracted_blocks.config_without_semantic_blocks,
        &request.render_plan.owned_top_level_setting_names,
    );
    let merged_keybinds =
        build_merged_keybinds_block(&extracted_blocks.keybind_lines, &request.override_keybinds);
    let merged_ui = build_yazelix_ui_block(
        &extracted_blocks.ui_lines,
        &request.render_plan.rounded_value,
    );
    let plugins_block = build_yazelix_plugins_block(
        &extracted_blocks.plugin_lines,
        request,
        &request.render_plan,
    );
    let load_plugins_block = build_yazelix_load_plugins_block(&extracted_blocks.load_plugin_lines);

    [
        "// ========================================".to_string(),
        "// GENERATED ZELLIJ CONFIG (YAZELIX)".to_string(),
        "// ========================================".to_string(),
        "// Source preference:".to_string(),
        "//   1) ~/.config/yazelix/zellij.kdl (Yazelix-managed override)".to_string(),
        "//   2) ~/.config/zellij/config.kdl (native fallback, read-only)".to_string(),
        "//   3) zellij setup --dump-config (defaults)".to_string(),
        "//".to_string(),
        "// Generated: 1970-01-01 00:00:00".to_string(),
        "// ========================================".to_string(),
        String::new(),
        base_config,
        String::new(),
        merged_keybinds,
        String::new(),
        plugins_block,
        String::new(),
        merged_ui,
        String::new(),
        render_top_level_settings_block(
            "// === YAZELIX DYNAMIC SETTINGS (from settings.jsonc) ===",
            &request.render_plan.dynamic_top_level_settings,
        ),
        String::new(),
        render_top_level_settings_block(
            "// === YAZELIX ENFORCED SETTINGS ===",
            &request.render_plan.enforced_top_level_settings,
        ),
        String::new(),
        "// === YAZELIX BACKGROUND PLUGINS ===".to_string(),
        load_plugins_block,
    ]
    .join("\n")
}

fn build_yazelix_plugins_block(
    existing_lines: &[String],
    request: &ZellijConfigPackRenderRequest,
    render_plan: &ZellijRenderPlanData,
) -> String {
    let mut merged_lines = existing_lines.to_vec();
    let orchestrator_present = merged_lines
        .iter()
        .any(|line| line.contains(&format!("{PANE_ORCHESTRATOR_PLUGIN_ALIAS} location=")));
    if !orchestrator_present {
        merged_lines.extend([
            format!(
                "    {PANE_ORCHESTRATOR_PLUGIN_ALIAS} location=\"{}\" {{",
                request.pane_orchestrator_plugin_url
            ),
            format!("        runtime_dir {}", json_quote(&request.runtime_dir)),
            format!(
                "        screen_saver_enabled \"{}\"",
                render_plan.screen_saver_enabled
            ),
            format!(
                "        screen_saver_idle_seconds \"{}\"",
                render_plan.screen_saver_idle_seconds
            ),
            format!(
                "        screen_saver_style {}",
                json_quote(&render_plan.screen_saver_style)
            ),
            format!(
                "        runtime_config_generation {}",
                json_quote(&request.generation_fingerprint)
            ),
        ]);
        merged_lines.push(format!(
            "        right_sidebar_command {}",
            json_quote(expand_runtime_placeholder(
                &render_plan.right_sidebar_command,
                &request.runtime_dir,
            ))
        ));
        for (index, arg) in render_plan.right_sidebar_args.iter().enumerate() {
            merged_lines.push(format!(
                "        right_sidebar_arg_{} {}",
                index + 1,
                json_quote(expand_runtime_placeholder(arg, &request.runtime_dir))
            ));
        }
        merged_lines.push("    }".to_string());
    }

    let yzpp_present = merged_lines
        .iter()
        .any(|line| line.contains(&format!("{YZPP_PLUGIN_ALIAS} location=")));
    if !yzpp_present {
        merged_lines.extend(render_yzpp_plugin_block(request, render_plan));
    }

    if merged_lines.is_empty() {
        String::new()
    } else {
        block_with_lines("plugins", &merged_lines)
    }
}

fn render_yzpp_plugin_block(
    request: &ZellijConfigPackRenderRequest,
    render_plan: &ZellijRenderPlanData,
) -> Vec<String> {
    let yzx_cli = format!("{}/shells/posix/yzx_cli.sh", request.runtime_dir);
    let bottom_popup_program =
        generated_popup_command(&request.popup_commands, BOTTOM_POPUP_COMMAND_KEY, &yzx_cli);
    let top_popup_program =
        generated_popup_command(&request.popup_commands, TOP_POPUP_COMMAND_KEY, &yzx_cli);
    let menu_program =
        generated_popup_command(&request.popup_commands, MENU_POPUP_COMMAND_KEY, &yzx_cli);
    let mut lines = vec![
        format!(
            "    {YZPP_PLUGIN_ALIAS} location=\"{}\" {{",
            request.yzpp_plugin_url
        ),
        "        popups {".to_string(),
    ];

    append_generated_popup_spec(
        &mut lines,
        "bottom_popup",
        "yzx_bottom_popup",
        Some("yzx_bottom_popup"),
        &bottom_popup_program,
        render_plan.popup_width_percent,
        render_plan.popup_height_percent,
        None,
        Some(&yzx_cli),
    );
    append_generated_popup_spec(
        &mut lines,
        "top_popup",
        "yzx_top_popup",
        Some("yzx_top_popup"),
        &top_popup_program,
        render_plan.popup_width_percent,
        render_plan.popup_height_percent,
        None,
        None,
    );
    append_generated_popup_spec(
        &mut lines,
        "menu",
        "yzx_menu",
        Some("yzx menu"),
        &menu_program,
        render_plan.popup_width_percent,
        render_plan.popup_height_percent,
        None,
        None,
    );
    for custom_popup in &request.custom_popups {
        let custom_popup_program =
            popup_command_argv_for_yazelix_runtime(&custom_popup.command, &yzx_cli);
        let pane_title = format!("yzx_{}", custom_popup.id);
        append_generated_popup_spec(
            &mut lines,
            &custom_popup.id,
            &pane_title,
            Some(&pane_title),
            &custom_popup_program,
            render_plan.popup_width_percent,
            render_plan.popup_height_percent,
            custom_popup.keep_alive.then_some("hide"),
            None,
        );
    }
    lines.extend([
        "            config {".to_string(),
        format!("                command {}", json_quote(&yzx_cli)),
        "                arg_1 \"config\"".to_string(),
        "                arg_2 \"ui\"".to_string(),
        "                pane_title \"yzx_config\"".to_string(),
        "                command_marker \"yzx config ui\"".to_string(),
        format!(
            "                width_percent \"{}\"",
            render_plan.popup_width_percent
        ),
        format!(
            "                height_percent \"{}\"",
            render_plan.popup_height_percent
        ),
        "            }".to_string(),
        "        }".to_string(),
        "    }".to_string(),
    ]);
    lines
}

fn generated_popup_command(
    popup_commands: &BTreeMap<String, Vec<String>>,
    key: &str,
    yzx_cli: &str,
) -> Vec<String> {
    popup_commands
        .get(key)
        .map(|command| popup_command_argv_for_yazelix_runtime(command, yzx_cli))
        .unwrap_or_default()
}

fn popup_command_argv_for_yazelix_runtime(command: &[String], yzx_cli: &str) -> Vec<String> {
    let Some(command_path) = command.first() else {
        return Vec::new();
    };
    if command_path == yzx_cli {
        return command.to_vec();
    }
    if command_path == "yzx" {
        return std::iter::once(yzx_cli.to_string())
            .chain(command.iter().skip(1).cloned())
            .collect();
    }
    std::iter::once(yzx_cli.to_string())
        .chain(std::iter::once("popup_run".to_string()))
        .chain(command.iter().cloned())
        .collect()
}

fn append_generated_popup_spec(
    lines: &mut Vec<String>,
    id: &str,
    pane_title: &str,
    command_marker: Option<&str>,
    popup_argv: &[String],
    popup_width_percent: i64,
    popup_height_percent: i64,
    toggle_close_behavior: Option<&str>,
    on_close_yzx_cli: Option<&str>,
) {
    lines.push(format!("            {id} {{"));
    if let Some(command_path) = popup_argv.first() {
        lines.push(format!(
            "                command {}",
            json_quote(command_path)
        ));
        for (index, arg) in popup_argv.iter().skip(1).enumerate() {
            lines.push(format!(
                "                arg_{} {}",
                index + 1,
                json_quote(arg)
            ));
        }
        let marker = command_marker.unwrap_or(command_path);
        lines.push(format!(
            "                command_marker {}",
            json_quote(marker)
        ));
    }
    lines.extend([
        format!("                pane_title {}", json_quote(pane_title)),
        format!("                width_percent \"{popup_width_percent}\""),
        format!("                height_percent \"{popup_height_percent}\""),
    ]);
    if let Some(toggle_close_behavior) = toggle_close_behavior {
        lines.push(format!(
            "                toggle_close_behavior {}",
            json_quote(toggle_close_behavior)
        ));
    }
    if let Some(yzx_cli) = on_close_yzx_cli {
        lines.extend([
            "                on_close {".to_string(),
            format!("                    command {}", json_quote(yzx_cli)),
            "                    arg_1 \"sidebar\"".to_string(),
            "                    arg_2 \"refresh\"".to_string(),
            "                }".to_string(),
        ]);
    }
    lines.push("            }".to_string());
}

fn build_yazelix_load_plugins_block(existing_lines: &[String]) -> String {
    let mut seen = BTreeSet::new();
    let mut merged_lines = Vec::new();
    for line in existing_lines {
        let trimmed = line.trim();
        if !trimmed.is_empty() && seen.insert(trimmed.to_string()) {
            merged_lines.push(line.clone());
        }
    }
    if merged_lines.is_empty() {
        String::new()
    } else {
        block_with_lines("load_plugins", &merged_lines)
    }
}

fn build_merged_keybinds_block(existing_lines: &[String], override_lines: &[String]) -> String {
    let mut merged = existing_lines.to_vec();
    merged.extend_from_slice(override_lines);
    if merged.is_empty() {
        String::new()
    } else {
        block_with_lines("keybinds", &merged)
    }
}

fn build_yazelix_ui_block(existing_ui_lines: &[String], rounded_value: &str) -> String {
    let existing_ui_text = existing_ui_lines.join("\n");
    let hide_session_name = existing_ui_text.contains("hide_session_name true");
    let mut lines = vec![
        "ui {".to_string(),
        "    pane_frames {".to_string(),
        format!("        rounded_corners {rounded_value}"),
    ];
    if hide_session_name {
        lines.push("        hide_session_name true".to_string());
    }
    lines.extend(["    }".to_string(), "}".to_string()]);
    lines.join("\n")
}

fn render_top_level_settings_block(header: &str, settings: &[TopLevelSetting]) -> String {
    std::iter::once(header.to_string())
        .chain(
            settings
                .iter()
                .map(|setting| format!("{} {}", setting.name, setting.value)),
        )
        .collect::<Vec<_>>()
        .join("\n")
}

fn strip_owned_top_level_settings(content: &str, owned_setting_names: &[String]) -> String {
    content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !owned_setting_names
                .iter()
                .any(|name| trimmed.starts_with(&format!("{name} ")))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_layout_template(
    content: &str,
    static_fragments: &BTreeMap<String, String>,
    zjstatus_plugin_block: &str,
    pane_orchestrator_plugin_url: &str,
    home_dir: &str,
    runtime_dir: &str,
    render_plan: &ZellijRenderPlanData,
) -> Result<String, String> {
    let mut updated = apply_static_fragments(content, static_fragments);
    let replacements = [
        (
            ZJSTATUS_TAB_TEMPLATE_PLACEHOLDER,
            zjstatus_plugin_block.to_string(),
        ),
        (
            PANE_ORCHESTRATOR_PLUGIN_URL_PLACEHOLDER,
            pane_orchestrator_plugin_url.to_string(),
        ),
        (HOME_DIR_PLACEHOLDER, home_dir.to_string()),
        (RUNTIME_DIR_PLACEHOLDER, runtime_dir.to_string()),
        (
            "__YAZELIX_SIDEBAR_COMMAND__",
            json_quote(expand_runtime_placeholder(
                &render_plan.left_sidebar_command,
                runtime_dir,
            )),
        ),
        (
            "__YAZELIX_SIDEBAR_ARGS__",
            render_sidebar_args(&render_plan.left_sidebar_args, runtime_dir),
        ),
        (
            "__YAZELIX_SIDEBAR_WIDTH_PERCENT__",
            render_plan
                .layout_percentages
                .left_sidebar_width_percent
                .clone(),
        ),
        (
            "__YAZELIX_AGENT_WIDTH_PERCENT__",
            render_plan
                .layout_percentages
                .right_sidebar_width_percent
                .clone(),
        ),
        (
            "__YAZELIX_OPEN_CONTENT_WIDTH_PERCENT__",
            render_plan
                .layout_percentages
                .open_content_width_percent
                .clone(),
        ),
        (
            "__YAZELIX_CLOSED_CONTENT_WIDTH_PERCENT__",
            render_plan
                .layout_percentages
                .closed_content_width_percent
                .clone(),
        ),
        (
            "__YAZELIX_OPEN_AGENT_OPEN_CONTENT_WIDTH_PERCENT__",
            render_plan
                .layout_percentages
                .left_open_right_open_content_width_percent
                .clone(),
        ),
        (
            "__YAZELIX_OPEN_AGENT_CLOSED_CONTENT_WIDTH_PERCENT__",
            render_plan
                .layout_percentages
                .left_open_right_closed_content_width_percent
                .clone(),
        ),
        (
            "__YAZELIX_CLOSED_AGENT_OPEN_CONTENT_WIDTH_PERCENT__",
            render_plan
                .layout_percentages
                .left_closed_right_open_content_width_percent
                .clone(),
        ),
        (
            "__YAZELIX_CLOSED_AGENT_CLOSED_CONTENT_WIDTH_PERCENT__",
            render_plan
                .layout_percentages
                .left_closed_right_closed_content_width_percent
                .clone(),
        ),
    ];
    for (placeholder, value) in replacements {
        updated = updated.replace(placeholder, &value);
    }
    for placeholder in REQUIRED_LAYOUT_PLACEHOLDERS {
        if updated.contains(placeholder) {
            return Err(format!(
                "failed to expand Zellij layout placeholder: {placeholder}"
            ));
        }
    }
    Ok(updated)
}

fn generated_zellij_layout_header(generation_fingerprint: &str) -> String {
    format!(
        "// ========================================\n// {GENERATED_LAYOUT_MARKER}\n// {GENERATED_LAYOUT_FINGERPRINT_PREFIX} {generation_fingerprint}\n// ========================================\n"
    )
}

fn apply_static_fragments(content: &str, fragments: &BTreeMap<String, String>) -> String {
    let mut updated = content.to_string();
    for (placeholder, value) in fragments {
        if !updated.contains(placeholder) {
            continue;
        }
        let fragment_lines = value.lines().collect::<Vec<_>>();
        updated = updated
            .lines()
            .map(|line| {
                if line.contains(placeholder) {
                    let indent = line
                        .chars()
                        .take_while(|ch| ch.is_whitespace())
                        .collect::<String>();
                    fragment_lines
                        .iter()
                        .map(|fragment_line| format!("{indent}{fragment_line}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
    }
    updated
}

fn render_sidebar_args(args: &[String], runtime_dir: &str) -> String {
    if args.is_empty() {
        String::new()
    } else {
        format!(
            "args {}",
            args.iter()
                .map(|arg| json_quote(expand_runtime_placeholder(arg, runtime_dir)))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

fn expand_runtime_placeholder(value: &str, runtime_dir: &str) -> String {
    value.replace(RUNTIME_DIR_PLACEHOLDER, runtime_dir)
}

#[derive(Debug, Default)]
struct ExtractedSemanticBlocks {
    config_without_semantic_blocks: String,
    load_plugin_lines: Vec<String>,
    plugin_lines: Vec<String>,
    keybind_lines: Vec<String>,
    ui_lines: Vec<String>,
}

fn extract_semantic_config_blocks(config_content: &str) -> ExtractedSemanticBlocks {
    let mut stripped_lines = Vec::new();
    let mut load_plugin_lines = Vec::new();
    let mut plugin_lines = Vec::new();
    let mut keybind_lines = Vec::new();
    let mut ui_lines = Vec::new();
    let mut active_block = String::new();
    let mut brace_depth: i64 = 0;

    for line in config_content.lines() {
        let trimmed = line.trim();
        let open_braces = line.chars().filter(|ch| *ch == '{').count() as i64;
        let close_braces = line.chars().filter(|ch| *ch == '}').count() as i64;

        if active_block.is_empty() {
            let matched_block = ["load_plugins", "plugins", "keybinds", "ui"]
                .into_iter()
                .find(|block| trimmed.starts_with(block));
            if let Some(block) = matched_block {
                active_block = block.to_string();
                brace_depth = open_braces - close_braces;
                if brace_depth <= 0 {
                    let inline_body = trimmed
                        .trim_start_matches(block)
                        .trim()
                        .trim_start_matches('{')
                        .trim_end_matches('}')
                        .trim();
                    if !inline_body.is_empty() {
                        push_semantic_line(
                            block,
                            inline_body.to_string(),
                            &mut load_plugin_lines,
                            &mut plugin_lines,
                            &mut keybind_lines,
                            &mut ui_lines,
                        );
                    }
                    active_block.clear();
                    brace_depth = 0;
                }
            } else {
                stripped_lines.push(line.to_string());
            }
        } else {
            brace_depth += open_braces - close_braces;
            if brace_depth > 0 {
                push_semantic_line(
                    &active_block,
                    line.to_string(),
                    &mut load_plugin_lines,
                    &mut plugin_lines,
                    &mut keybind_lines,
                    &mut ui_lines,
                );
            } else {
                active_block.clear();
            }
        }
    }

    ExtractedSemanticBlocks {
        config_without_semantic_blocks: stripped_lines.join("\n"),
        load_plugin_lines,
        plugin_lines,
        keybind_lines,
        ui_lines,
    }
}

fn push_semantic_line(
    block: &str,
    line: String,
    load_plugin_lines: &mut Vec<String>,
    plugin_lines: &mut Vec<String>,
    keybind_lines: &mut Vec<String>,
    ui_lines: &mut Vec<String>,
) {
    match block {
        "load_plugins" => load_plugin_lines.push(line),
        "plugins" => plugin_lines.push(line),
        "keybinds" => keybind_lines.push(line),
        "ui" => ui_lines.push(line),
        _ => {}
    }
}

fn block_with_lines(name: &str, lines: &[String]) -> String {
    std::iter::once(format!("{name} {{"))
        .chain(lines.iter().cloned())
        .chain(std::iter::once("}".to_string()))
        .collect::<Vec<_>>()
        .join("\n")
}

fn json_quote(value: impl AsRef<str>) -> String {
    serde_json::to_string(value.as_ref()).unwrap_or_else(|_| "\"\"".to_string())
}

// Test lane: default
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_request() -> ZellijConfigPackRenderRequest {
        ZellijConfigPackRenderRequest {
            base_config_content: "scroll_buffer_size 100\nkeybinds { normal { bind \"Alt h\" { MoveFocusOrTab \"left\"; } } }\n".to_string(),
            override_keybinds: vec![
                r#"    normal { bind "Alt X" { SwitchToMode "Normal"; } }"#.to_string(),
            ],
            render_plan: ZellijRenderPlanData {
                owned_top_level_setting_names: vec!["default_layout".to_string()],
                dynamic_top_level_settings: vec![TopLevelSetting {
                    name: "theme".to_string(),
                    value: "\"default\"".to_string(),
                }],
                enforced_top_level_settings: vec![TopLevelSetting {
                    name: "default_layout".to_string(),
                    value: "\"/tmp/yazelix/layouts/yzx_side.kdl\"".to_string(),
                }],
                rounded_value: "true".to_string(),
                popup_width_percent: 90,
                popup_height_percent: 80,
                screen_saver_enabled: false,
                screen_saver_idle_seconds: 300,
                screen_saver_style: "random".to_string(),
                right_sidebar_command: "__YAZELIX_RUNTIME_DIR__/bin/agent".to_string(),
                right_sidebar_args: vec!["--right".to_string()],
                left_sidebar_command: "__YAZELIX_RUNTIME_DIR__/bin/sidebar".to_string(),
                left_sidebar_args: vec![
                    "--root".to_string(),
                    "__YAZELIX_RUNTIME_DIR__/side".to_string(),
                ],
                layout_percentages: ZellijLayoutPercentages {
                    left_sidebar_width_percent: "20%".to_string(),
                    right_sidebar_width_percent: "40%".to_string(),
                    open_content_width_percent: "80%".to_string(),
                    closed_content_width_percent: "100%".to_string(),
                    left_open_right_open_content_width_percent: "40%".to_string(),
                    left_open_right_closed_content_width_percent: "80%".to_string(),
                    left_closed_right_open_content_width_percent: "60%".to_string(),
                    left_closed_right_closed_content_width_percent: "100%".to_string(),
                },
            },
            popup_commands: BTreeMap::from([
                ("bottom_popup".to_string(), vec!["lazygit".to_string()]),
                ("top_popup".to_string(), vec!["btop".to_string()]),
                ("menu".to_string(), vec!["yzx".to_string(), "menu".to_string()]),
            ]),
            custom_popups: vec![CustomPopup {
                id: "gitui".to_string(),
                command: vec!["gitui".to_string()],
                keybindings: vec!["Alt Shift G".to_string()],
                keep_alive: false,
            }],
            layout_templates: None,
            static_fragments: None,
            zjstatus_plugin_block: r#"plugin location="file:/tmp/zjstatus.wasm" {
    pipe_workspace_format "child-owned-workspace"
}"#
            .to_string(),
            pane_orchestrator_plugin_url: "file:/tmp/pane.wasm".to_string(),
            yzpp_plugin_url: "file:/tmp/yzpp.wasm".to_string(),
            home_dir: "/home/user".to_string(),
            runtime_dir: "/opt/yazelix".to_string(),
            generation_fingerprint: "gen-test".to_string(),
        }
    }

    // Defends: the child render API is deterministic from explicit request data and bundled assets.
    #[test]
    fn renders_bundled_config_pack_without_main_checkout_state() {
        let output = render_zellij_config_pack(&sample_request()).unwrap();

        assert_eq!(output.renderer_schema_version, RENDERER_SCHEMA_VERSION);
        assert!(output.merged_config.contains("scroll_buffer_size 100"));
        assert!(output.merged_config.contains("Alt X"));
        assert!(output.merged_config.contains("gitui"));
        assert!(output.merged_config.contains("file:/tmp/pane.wasm"));
        assert!(output.merged_config.contains("file:/tmp/yzpp.wasm"));
        assert_eq!(output.layout_files.len(), 2);
        let side = output
            .layout_files
            .iter()
            .find(|file| file.relative_path == "yzx_side.kdl")
            .unwrap();
        assert!(
            side.content
                .starts_with(&generated_zellij_layout_header("gen-test"))
        );
        assert!(
            side.content
                .contains(r#"plugin location="file:/tmp/zjstatus.wasm" {"#)
        );
        assert!(side.content.contains(r#"cwd="/home/user""#));
        assert!(
            side.content
                .contains(r#"command "/opt/yazelix/bin/sidebar""#)
        );
        assert!(
            side.content
                .contains(r#"args "--root" "/opt/yazelix/side""#)
        );
        for placeholder in REQUIRED_LAYOUT_PLACEHOLDERS {
            assert!(!side.content.contains(placeholder));
        }
    }

    // Regression: external popup commands are wrapped through the runtime CLI wrapper before yzpp sees them.
    #[test]
    fn wraps_external_popup_commands_through_runtime_cli() {
        assert_eq!(
            popup_command_argv_for_yazelix_runtime(
                &["lazygit".to_string(), "status".to_string()],
                "/opt/yazelix/shells/posix/yzx_cli.sh",
            ),
            vec![
                "/opt/yazelix/shells/posix/yzx_cli.sh".to_string(),
                "popup_run".to_string(),
                "lazygit".to_string(),
                "status".to_string(),
            ]
        );
    }
}
