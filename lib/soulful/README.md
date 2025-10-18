# Soulful Crate

The `soulful` crate implements the core application logic, providing a high-level API for interacting with music services like MusicBrainz and Soulseek. It is responsible for searching, fetching, and managing music data.

## Modules

### `musicbrainz`

This module provides a client for the MusicBrainz API, allowing the application to search for and retrieve detailed information about artists, albums, and tracks.

- **`search(artist: &Option<String>, query: &str, search_type: SearchType, limit: u8) -> Result<Vec<SearchResult>, musicbrainz_rs::Error>`**:

  - **Description**: Performs a search on MusicBrainz for tracks or albums. It refines the results to prioritize official releases and avoid duplicates.
  - **Parameters**:
    - `artist`: An optional artist name to narrow down the search.
    - `query`: The search query (e.g., track or album title).
    - `search_type`: An enum (`SearchType`) specifying whether to search for a `Track` or an `Album`.
    - `limit`: The maximum number of results to return.
  - **Returns**: A `Vec<SearchResult>` containing the search results.

- **`find_album(release_id: &str) -> Result<AlbumWithTracks, musicbrainz_rs::Error>`**:
  - **Description**: Fetches a specific album by its MusicBrainz ID, returning detailed information about the album and its full tracklist.
  - **Parameters**:
    - `release_id`: The MusicBrainz ID of the album to retrieve.
  - **Returns**: An `AlbumWithTracks` struct containing the album's metadata and its tracks.

### `slskd`

This module contains the client for interacting with a `slskd` (Soulseek daemon) instance. It handles searching for files on the Soulseek network and managing downloads.

- **`SoulseekClient`**:
  - **Description**: A client to communicate with the `slskd` API. It is configured with the API key, base URL, and download path.
  - **Methods**:
    - **`search(artist: String, album: String, tracks: Vec<Track>, timeout: Duration) -> Result<Vec<AlbumResult>, SoulfulError>`**: Searches for albums on Soulseek that match the provided metadata. It scores and ranks the results based on quality and availability.
    - **`download(tracks: Vec<TrackResult>) -> Result<Vec<DownloadResponse>, SoulfulError>`**: Initiates the download of a list of selected tracks.

### `beets`

This module is intended for integration with the [Beets](https://beets.io/) music tagger. (Functionality not detailed as per user request).
