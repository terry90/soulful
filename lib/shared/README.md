# Shared Crate

The `shared` crate contains the data structures that are used across the entire application, ensuring data consistency between the `api`, `ui`, and `soulful` crates. All models are serializable and deserializable using `serde`.

## Modules

### `musicbrainz`

This module defines the data structures for interacting with the MusicBrainz API.

- **`SearchResult`**: An enum that represents a search result, which can be either a `Track` or an `Album`.
- **`Track`**: A struct containing detailed information about a track, including its title, artist, and album details.
- **`Album`**: A struct holding information about an album, such as its title, artist, and release date.
- **`AlbumWithTracks`**: A composite struct that combines `Album` information with a list of its `Track`s.

### `slskd`

This module contains the data structures for interacting with the `slskd` (Soulseek daemon) API.

- **`DownloadRequest`**: Represents a request to download a file from a Soulseek user.
- **`DownloadResponse`**: Contains the response from a download request, including the download ID.
- **`SearchResult`**: Holds information about a file found on the Soulseek network, including details about the user, file size, and quality metrics.
- **`TrackResult`**: A struct that combines a `SearchResult` with matched metadata like artist, title, and album.
- **`AlbumResult`**: Represents a potential album download, containing a list of `TrackResult`s and aggregated metadata.

### `download`

This module defines the structure for download queries.

- **`DownloadQuery`**: A struct that encapsulates a download request, containing the `Album` and a list of `Track`s to be downloaded.
