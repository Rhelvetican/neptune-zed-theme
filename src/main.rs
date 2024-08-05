#[macro_use]
mod color;
mod model;
mod pallette;

use std::collections::HashMap;

use color::Color;
use libjsonutils::file::write_json;
use model::{IntoStyle, PlayerColor, Style, SyntaxStyle, Theme};

const DIST: &str = "./themes/neptune_theme.json";

const THEMES_INDEX: &[&str] = &[
    "border",
    "border.variant",
    "border.focused",
    "border.selected",
    "border.transparent",
    "border.disabled",
    "elevated_surface.background",
    "surface.background",
    "background",
    "element.background",
    "element.hover",
    "element.active",
    "element.selected",
    "element.disabled",
    "drop_target.background",
    "ghost_element.background",
    "ghost_element.hover",
    "ghost_element.active",
    "ghost_element.selected",
    "ghost_element.disabled",
    "text",
    "text.muted",
    "text.placeholder",
    "text.disabled",
    "text.accent",
    "icon",
    "icon.muted",
    "icon.disabled",
    "icon.placeholder",
    "icon.accent",
    "status_bar.background",
    "title_bar.background",
    "title_bar.inactive_background",
    "toolbar.background",
    "tab_bar.background",
    "tab.inactive_background",
    "tab.active_background",
    "search.match_background",
    "panel.background",
    "panel.focused_border",
    "pane.focused_border",
    "scrollbar.thumb.background",
    "scrollbar.thumb.hover_background",
    "scrollbar.thumb.border",
    "scrollbar.track.background",
    "scrollbar.track.border",
    "editor.foreground",
    "editor.background",
    "editor.gutter.background",
    "editor.subheader.background",
    "editor.active_line.background",
    "editor.highlighted_line.background",
    "editor.line_number",
    "editor.active_line_number",
    "editor.invisible",
    "editor.wrap_guide",
    "editor.active_wrap_guide",
    "editor.document_highlight.read_background",
    "editor.document_highlight.write_background",
    "terminal.background",
    "terminal.foreground",
    "terminal.dim_foreground",
    "terminal.bright_foreground",
    "terminal.ansi.black",
    "terminal.ansi.red",
    "terminal.ansi.green",
    "terminal.ansi.yellow",
    "terminal.ansi.blue",
    "terminal.ansi.magenta",
    "terminal.ansi.cyan",
    "terminal.ansi.white",
    "terminal.ansi.bright_black",
    "terminal.ansi.bright_red",
    "terminal.ansi.bright_green",
    "terminal.ansi.bright_yellow",
    "terminal.ansi.bright_blue",
    "terminal.ansi.bright_magenta",
    "terminal.ansi.bright_cyan",
    "terminal.ansi.bright_white",
    "terminal.ansi.dim_black",
    "terminal.ansi.dim_red",
    "terminal.ansi.dim_green",
    "terminal.ansi.dim_yellow",
    "terminal.ansi.dim_blue",
    "terminal.ansi.dim_magenta",
    "terminal.ansi.dim_cyan",
    "terminal.ansi.dim_white",
    "link_text.hover",
    "conflict",
    "conflict.border",
    "conflict.background",
    "created",
    "created.border",
    "created.background",
    "deleted",
    "deleted.border",
    "deleted.background",
    "error",
    "error.border",
    "error.background",
    "hidden",
    "hidden.border",
    "hidden.background",
    "hint",
    "hint.border",
    "hint.background",
    "ignored",
    "ignored.border",
    "ignored.background",
    "info",
    "info.border",
    "info.background",
    "modified.border",
    "modified.background",
    "predictive",
    "predictive.border",
    "predictive.background",
    "renamed",
    "renamed.border",
    "renamed.background",
    "success",
    "success.border",
    "success.background",
    "unreachable",
    "unreachable.border",
    "unreachable.background",
    "warning",
    "warning.border",
    "warning.background",
];

const SYNTAXES_INDEX: &[&str] = &[
    "attribute",
    "boolean",
    "comment",
    "comment.doc",
    "constant",
    "constructor",
    "embedded",
    "emphasis",
    "emphasis.strong",
    "enum",
    "function",
    "hint",
    "keyword",
    "link_text",
    "link_uri",
    "number",
    "operator",
    "predictive",
    "predoc",
    "primary",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.list_marker",
    "punctuation.special",
    "punctuation.special.symbol",
    "string",
    "string.escape",
    "string.regex",
    "string.special",
    "string.special.symbol",
    "tag",
    "text.literal",
    "title",
    "type",
    "type.interface",
    "type.super",
    "variable",
    "variable.member",
    "variable.parameter",
    "variable.special",
    "variant",
];

fn main() {
    let syntaxes = set_syntax_color();
    let players = set_player_colors();
    let mut style = HashMap::<&str, Style>::new();

    style.insert("players", players.into_style());
    style.insert("syntax", syntaxes.into_style());

    let theme = Theme::new(String::from("x"), String::from("x"), style);

    write_json(DIST, theme).unwrap();
}

fn set_syntax_color() -> HashMap<&'static str, SyntaxStyle> {
    let mut syntax = HashMap::<&str, SyntaxStyle>::new();
    for syntax in SYNTAXES_INDEX {}
    syntax
}

fn set_player_colors() -> Vec<PlayerColor> {
    let mut players = Vec::<PlayerColor>::new();

    players
}
