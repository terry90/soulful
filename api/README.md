# API Crate

This crate exposes server-side functionality to the frontend using `dioxus` server functions. It acts as a bridge between the UI components and the core application logic implemented in the `soulful` crate.

## Server Functions

The following server functions are exposed to the frontend:

- **`search_album(input: SearchQuery) -> Result<Vec<SearchResult>, ServerFnError>`**:

  - **Description**: Searches for albums on MusicBrainz based on the provided query.
  - **Parameters**:
    - `input`: A `SearchQuery` struct containing the artist and album name.
  - **Returns**: A `Vec<SearchResult>` containing a list of matching albums.

- **`search_track(input: SearchQuery) -> Result<Vec<SearchResult>, ServerFnError>`**:

  - **Description**: Searches for tracks on MusicBrainz.
  - **Parameters**:
    - `input`: A `SearchQuery` struct with the artist and track name.
  - **Returns**: A `Vec<SearchResult>` with the search results.

- **`find_album(id: String) -> Result<AlbumWithTracks, ServerFnError>`**:

  - **Description**: Retrieves detailed information for a specific album from MusicBrainz, including its tracklist.
  - **Parameters**:
    - `id`: The MusicBrainz ID of the album.
  - **Returns**: An `AlbumWithTracks` struct containing the album details.

- **`search_downloads(data: DownloadQuery) -> Result<Vec<AlbumResult>, ServerFnError>`**:

  - **Description**: Searches for available downloads on Soulseek for a given album and its tracks.
  - **Parameters**:
    - `data`: A `DownloadQuery` struct with the album and track information.
  - **Returns**: A `Vec<AlbumResult>` with potential download candidates.

- **`download(data: Vec<TrackResult>) -> Result<Vec<AlbumResult>, ServerFnError>`**:
  - **Description**: Initiates the download of the selected tracks from Soulseek.
  - **Parameters**:
    - `data`: A `Vec<TrackResult>` containing the tracks to be downloaded.
  - **Returns**: A `Vec<DownloadResponse>` indicating the status of each download.

## Data Structures

- **`SearchQuery`**: A struct used for sending search queries to the server. It includes optional fields for `artist` and the main `query`.
