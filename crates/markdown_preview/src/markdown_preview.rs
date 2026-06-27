use gpui::{App, actions};
use workspace::Workspace;

pub mod markdown_preview_settings;
pub mod markdown_preview_view;

pub use zed_actions::preview::markdown::{OpenPreview, OpenPreviewToTheSide};

use crate::markdown_preview_view::MarkdownPreviewView;

actions!(
    markdown,
    [
        /// Scrolls up by one page in the markdown preview.
        #[action(deprecated_aliases = ["markdown::MovePageUp"])]
        ScrollPageUp,
        /// Scrolls down by one page in the markdown preview.
        #[action(deprecated_aliases = ["markdown::MovePageDown"])]
        ScrollPageDown,
        /// Scrolls up by approximately one visual line.
        ScrollUp,
        /// Scrolls down by approximately one visual line.
        ScrollDown,
        /// Scrolls up by one markdown element in the markdown preview
        ScrollUpByItem,
        /// Scrolls down by one markdown element in the markdown preview
        ScrollDownByItem,
        /// Scrolls to the top of the markdown preview.
        ScrollToTop,
        /// Scrolls to the bottom of the markdown preview.
        ScrollToBottom,
        /// Opens a following markdown preview that syncs with the editor.
        OpenFollowingPreview,
        /// Opens the current markdown file as a rendered editor in the same pane.
        OpenRenderedEditor,
        /// Reopens the current rendered markdown editor as source.
        OpenSourceEditor,
        /// Toggles the current markdown file between source and rendered edit modes.
        ToggleRenderedEditor,
        /// Toggles strong emphasis around the rendered/source Markdown selection.
        ToggleStrong,
        /// Toggles emphasis around the rendered/source Markdown selection.
        ToggleEmphasis,
        /// Toggles inline code around the rendered/source Markdown selection.
        ToggleInlineCode,
        /// Converts selected Markdown lines to paragraph text.
        SetParagraph,
        /// Converts selected Markdown lines to heading level 1.
        SetHeading1,
        /// Converts selected Markdown lines to heading level 2.
        SetHeading2,
        /// Converts selected Markdown lines to heading level 3.
        SetHeading3,
        /// Converts selected Markdown lines to heading level 4.
        SetHeading4,
        /// Converts selected Markdown lines to heading level 5.
        SetHeading5,
        /// Converts selected Markdown lines to heading level 6.
        SetHeading6
    ]
);

pub fn init(cx: &mut App) {
    workspace::register_serializable_item::<MarkdownPreviewView>(cx);

    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        markdown_preview_view::MarkdownPreviewView::register(workspace, window, cx);
    })
    .detach();
}
