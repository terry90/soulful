use dioxus::prelude::*;
use shared::slskd::{AlbumResult, TrackResult};
use std::collections::HashSet;

use crate::Checkbox;

#[derive(Props, PartialEq, Clone)]
pub struct Props {
    pub results: Vec<AlbumResult>,
    #[props(into)]
    pub on_download: EventHandler<Vec<TrackResult>>,
}

#[derive(Props, Clone, PartialEq)]
struct AlbumResultItemProps {
    album: AlbumResult,
    selected_tracks: Signal<HashSet<String>>,
    on_album_select_all: EventHandler<AlbumResult>,
    on_track_toggle: EventHandler<String>,
}

#[component]
fn AlbumResultItem(props: AlbumResultItemProps) -> Element {
    let album = props.album.clone();

    rsx! {
        div { key: "{album.album_path}", class: "bg-gray-700 p-4 rounded-md",
            div { class: "flex justify-between items-center mb-2",
                div {
                    h4 { class: "text-lg font-semibold", "{album.album_title}" }
                    p { class: "text-sm text-gray-400",
                        "by {album.artist.clone().unwrap_or_default()}"
                    }
                    p { class: "text-sm text-gray-400",
                        "Quality: {album.dominant_quality}, Score: {album.score:.2}"
                    }
                }
                button {
                    class: "bg-teal-500 hover:bg-teal-600 text-white font-bold py-2 px-4 rounded-md transition-colors duration-300",
                    onclick: move |_| props.on_album_select_all.call(album.clone()),
                    "Select All"
                }
            }
            ul { class: "list-disc pl-5 space-y-1",
                for TrackResult { base , title , .. } in props.album.tracks {
                    li {
                        key: "{base.filename}",
                        class: "flex items-center gap-2",
                        onclick: move |_| props.on_track_toggle.call(base.filename.clone()),

                        Checkbox { is_selected: props.selected_tracks.read().contains(&base.filename) }

                        label { "{title}" }
                    }
                }
            }
        }
    }
}

/// Main component responsible for displaying all download options.
#[component]
pub fn DownloadResults(props: Props) -> Element {
    let mut selected_tracks = use_signal(|| HashSet::<String>::new());
    let results = props.results.clone();

    let handle_album_select_all = move |album_result: AlbumResult| {
        let mut selected = selected_tracks.write();
        let all_selected = album_result
            .tracks
            .iter()
            .all(|t| selected.contains(&t.base.filename));

        if all_selected {
            for track in &album_result.tracks {
                selected.remove(&track.base.filename);
            }
        } else {
            for track in &album_result.tracks {
                selected.insert(track.base.filename.clone());
            }
        }
    };

    let handle_track_toggle = move |filename: String| {
        let mut selected = selected_tracks.write();
        if selected.contains(&filename) {
            selected.remove(&filename);
        } else {
            selected.insert(filename);
        }
    };

    let handle_download = move |_| {
        let selected_filenames = selected_tracks.read();
        let tracks_to_download: Vec<TrackResult> = props
            .results
            .iter()
            .flat_map(|album_result| album_result.tracks.iter())
            .filter(|track| selected_filenames.contains(&track.base.filename))
            .cloned()
            .collect();
        props.on_download.call(tracks_to_download);
    };

    rsx! {
        div { class: "bg-gray-800 text-white p-6 rounded-lg shadow-xl max-w-4xl mx-auto my-10",
            h3 { class: "text-2xl font-bold mb-6 text-center text-teal-400", "Download Options" }
            div { class: "space-y-4",
                for album in results {
                    AlbumResultItem {
                        album,
                        selected_tracks,
                        on_album_select_all: handle_album_select_all.clone(),
                        on_track_toggle: handle_track_toggle.clone(),
                    }
                }
            }
            div { class: "flex justify-end mt-6",
                button {
                    class: "bg-indigo-500 hover:bg-indigo-600 text-white font-bold py-2 px-4 rounded-md transition-colors duration-300 disabled:bg-gray-600 disabled:cursor-not-allowed",
                    disabled: selected_tracks.read().is_empty(),
                    onclick: handle_download,
                    "Download Selected"
                }
            }
        }
    }
}
