use rust_i18n::t;
use std::borrow::Cow;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

use super::release_notes::release_notes_close_button_rect;
use super::scrollbar::{release_notes_scrollbar_rect, render_scrollbar};
use super::widgets::{
    modal_stack_areas, panel_contrast_fg, render_action_button, render_modal_header,
    render_modal_shell,
};
use crate::app::AppState;

pub(super) type HelpEntry = (String, Cow<'static, str>);
pub(super) type HelpGroup = (Cow<'static, str>, Vec<HelpEntry>);

fn help_entry(key: impl Into<String>, label: Cow<'static, str>) -> HelpEntry {
    (key.into(), label)
}

fn keybind_label(bindings: &crate::config::ActionKeybinds) -> String {
    bindings.label().unwrap_or_else(|| t!("unset").to_string())
}

fn indexed_label(bindings: &[crate::config::IndexedKeybind]) -> String {
    if bindings.is_empty() {
        t!("unset").to_string()
    } else if bindings.len() == 9 {
        let first = &bindings[0].label;
        if first.ends_with('1') {
            format!("{}1..9", first.trim_end_matches('1'))
        } else {
            bindings
                .iter()
                .map(|binding| binding.label.clone())
                .collect::<Vec<_>>()
                .join(" / ")
        }
    } else {
        bindings
            .iter()
            .map(|binding| binding.label.clone())
            .collect::<Vec<_>>()
            .join(" / ")
    }
}

pub(super) fn keybind_help_groups(app: &AppState) -> Vec<HelpGroup> {
    let kb = &app.keybinds;
    let mut groups = Vec::new();

    groups.push((
        t!("global"),
        vec![
            help_entry(
                crate::config::format_key_combo((app.prefix_code, app.prefix_mods)),
                t!("prefix mode"),
            ),
            help_entry(keybind_label(&kb.help), t!("keybinds")),
            help_entry(keybind_label(&kb.settings), t!("settings")),
            help_entry(keybind_label(&kb.detach), t!("detach")),
            help_entry(keybind_label(&kb.reload_config), t!("reload config")),
            help_entry(
                keybind_label(&kb.open_notification_target),
                t!("open notification target"),
            ),
        ],
    ));

    groups.push((
        t!("navigation"),
        vec![
            help_entry("esc", t!("back")),
            help_entry(
                format!(
                    "{} / {}",
                    keybind_label(&kb.navigate.workspace_up),
                    keybind_label(&kb.navigate.workspace_down)
                ),
                t!("workspace list"),
            ),
            help_entry(
                format!(
                    "{} / {} / {} / {} / left / right",
                    keybind_label(&kb.navigate.pane_left),
                    keybind_label(&kb.navigate.pane_down),
                    keybind_label(&kb.navigate.pane_up),
                    keybind_label(&kb.navigate.pane_right)
                ),
                t!("move focus"),
            ),
            help_entry("tab / shift+tab", t!("cycle pane")),
            help_entry("enter", t!("open workspace")),
            help_entry("1..9", t!("switch workspace")),
        ],
    ));

    let workspace_tab = vec![
        help_entry(keybind_label(&kb.workspace_picker), t!("workspace navigation")),
        help_entry(keybind_label(&kb.goto), t!("session navigator")),
        help_entry(keybind_label(&kb.new_workspace), t!("new workspace")),
        help_entry(keybind_label(&kb.new_worktree), t!("new worktree")),
        help_entry(keybind_label(&kb.open_worktree), t!("open worktree")),
        help_entry(
            keybind_label(&kb.remove_worktree),
            t!("delete worktree checkout"),
        ),
        help_entry(keybind_label(&kb.rename_workspace), t!("rename workspace")),
        help_entry(keybind_label(&kb.close_workspace), t!("close workspace")),
        help_entry(keybind_label(&kb.previous_workspace), t!("previous workspace")),
        help_entry(keybind_label(&kb.next_workspace), t!("next workspace")),
        help_entry(indexed_label(&kb.switch_workspace), t!("switch workspace 1-9")),
        help_entry(keybind_label(&kb.previous_agent), t!("previous agent")),
        help_entry(keybind_label(&kb.next_agent), t!("next agent")),
        help_entry(indexed_label(&kb.focus_agent), t!("focus agent 1-9")),
        help_entry(keybind_label(&kb.new_tab), t!("new tab")),
        help_entry(keybind_label(&kb.rename_tab), t!("rename tab")),
        help_entry(keybind_label(&kb.previous_tab), t!("previous tab")),
        help_entry(keybind_label(&kb.next_tab), t!("next tab")),
        help_entry(indexed_label(&kb.switch_tab), t!("switch tab 1-9")),
        help_entry(keybind_label(&kb.close_tab), t!("close tab")),
    ];
    groups.push((t!("workspaces / tabs"), workspace_tab));

    let panes = vec![
        help_entry(keybind_label(&kb.split_vertical), t!("split vertical")),
        help_entry(keybind_label(&kb.split_horizontal), t!("split horizontal")),
        help_entry(keybind_label(&kb.close_pane), t!("close pane")),
        help_entry(keybind_label(&kb.rename_pane), t!("rename pane")),
        help_entry(keybind_label(&kb.edit_scrollback), t!("edit scrollback")),
        help_entry(keybind_label(&kb.copy_mode), t!("copy mode")),
        help_entry(keybind_label(&kb.zoom), t!("zoom pane")),
        help_entry(keybind_label(&kb.resize_mode), t!("resize mode")),
        help_entry(keybind_label(&kb.toggle_sidebar), t!("toggle sidebar")),
        help_entry(keybind_label(&kb.focus_pane_left), t!("focus pane left")),
        help_entry(keybind_label(&kb.focus_pane_down), t!("focus pane down")),
        help_entry(keybind_label(&kb.focus_pane_up), t!("focus pane up")),
        help_entry(keybind_label(&kb.focus_pane_right), t!("focus pane right")),
        help_entry(keybind_label(&kb.cycle_pane_next), t!("cycle pane next")),
        help_entry(
            keybind_label(&kb.cycle_pane_previous),
            t!("cycle pane previous"),
        ),
        help_entry(keybind_label(&kb.last_pane), t!("last pane")),
    ];
    groups.push((t!("panes"), panes));

    if !kb.custom_commands.is_empty() {
        groups.push((
            t!("custom"),
            kb.custom_commands
                .iter()
                .map(|binding| {
                    (
                        binding.label.clone(),
                        binding
                            .description
                            .clone()
                            .map(Cow::Owned)
                            .unwrap_or(t!("custom command")),
                    )
                })
                .collect(),
        ));
    }

    groups
}

pub(crate) fn keybind_help_lines(app: &AppState) -> Vec<(usize, Line<'static>)> {
    let heading_style = Style::default()
        .fg(app.palette.accent)
        .add_modifier(Modifier::BOLD);
    let key_style = Style::default()
        .fg(app.palette.mauve)
        .add_modifier(Modifier::BOLD);
    let label_style = Style::default().fg(app.palette.text);

    let groups = keybind_help_groups(app);
    let key_width = groups
        .iter()
        .flat_map(|(_, entries)| entries.iter().map(|(key, _)| key.chars().count()))
        .max()
        .unwrap_or(8);

    let mut lines = Vec::new();

    for (group, entries) in groups {
        lines.push((
            group.len() + 1,
            Line::from(vec![Span::styled(format!(" {group}"), heading_style)]),
        ));
        for (key, label) in entries {
            let padded_key = format!(" {:<width$} ", key, width = key_width);
            let width = padded_key.chars().count() + label.chars().count();
            lines.push((
                width,
                Line::from(vec![
                    Span::styled(padded_key, key_style),
                    Span::styled(label.into_owned(), label_style),
                ]),
            ));
        }
        lines.push((0, Line::raw("")));
    }

    lines
}

pub(super) fn render_keybind_help_overlay(app: &AppState, frame: &mut Frame) {
    super::dim_background(frame, frame.area());

    let Some(inner) = render_modal_shell(frame, frame.area(), 76, 22, &app.palette) else {
        return;
    };
    if inner.height < 6 || inner.width < 20 {
        return;
    }

    let stack = modal_stack_areas(inner, 2, 1, 0, 1);
    let header_rows =
        Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas::<2>(stack.header);

    render_modal_header(frame, header_rows[0], &t!("keybinds"), &app.palette);
    render_action_button(
        frame,
        release_notes_close_button_rect(header_rows[0]),
        Some("esc"),
        &t!("close"),
        Style::default()
            .fg(panel_contrast_fg(&app.palette))
            .bg(app.palette.accent)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_widget(
        Paragraph::new(format!(" {}", t!("available commands and configured shortcuts")))
            .style(Style::default().fg(app.palette.overlay1)),
        header_rows[1],
    );

    let body_area = stack.content;
    let metrics = crate::pane::ScrollMetrics {
        offset_from_bottom: app
            .keybind_help_max_scroll()
            .saturating_sub(app.keybind_help.scroll) as usize,
        max_offset_from_bottom: app.keybind_help_max_scroll() as usize,
        viewport_rows: body_area.height.max(1) as usize,
    };
    let track = release_notes_scrollbar_rect(body_area, metrics);
    let text_area = track
        .map(|_| {
            Rect::new(
                body_area.x,
                body_area.y,
                body_area.width.saturating_sub(1),
                body_area.height,
            )
        })
        .unwrap_or(body_area);

    let body = Paragraph::new(
        keybind_help_lines(app)
            .into_iter()
            .map(|(_, line)| line)
            .collect::<Vec<_>>(),
    )
    .wrap(Wrap { trim: false })
    .scroll((app.keybind_help.scroll, 0));
    frame.render_widget(body, text_area);
    if let Some(track) = track {
        render_scrollbar(
            frame,
            metrics,
            track,
            app.palette.overlay0,
            app.palette.overlay1,
            "▐",
        );
    }

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(format!(" {} ", t!("scroll")), Style::default().fg(app.palette.overlay0)),
            Span::styled("wheel ↑↓", Style::default().fg(app.palette.text)),
            Span::styled("  ·  ", Style::default().fg(app.palette.overlay0)),
            Span::styled(t!("jump"), Style::default().fg(app.palette.overlay0)),
            Span::styled(" pgup / pgdn ", Style::default().fg(app.palette.text)),
            Span::styled("  ·  ", Style::default().fg(app.palette.overlay0)),
            Span::styled(t!("close"), Style::default().fg(app.palette.overlay0)),
            Span::styled(" esc / enter ", Style::default().fg(app.palette.text)),
        ])),
        stack.footer.unwrap_or_default(),
    );
}
