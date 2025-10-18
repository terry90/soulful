# Soulful Music Application: Technical Documentation

## 1. Project Overview

This document provides a comprehensive overview of the Soulful Music Application, a Rust-based project designed for searching, discovering, and downloading music. The application is built with a modern, modular architecture, leveraging the `dioxus` framework for the user interface and interacting with external services like MusicBrainz and Soulseek.

The project is divided into several key crates, each with a specific responsibility:

- **`soulful`**: The core logic crate, responsible for all business logic and interactions with external APIs.
- **`api`**: The API layer that exposes server-side functionality to the frontend using `dioxus` server functions.
- **`ui`**: The user interface crate, containing all the `dioxus` components that make up the frontend.
- **`shared`**: A crate for common data structures used across the entire application to ensure data consistency.

---

## 2. `soulful` Crate

The `soulful` crate implements the core application logic, providing a high-level API for interacting with music services like MusicBrainz and Soulseek. It is responsible for searching, fetching, and managing music data.

### Modules

#### `musicbrainz`

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

#### `slskd`

This module contains the client for interacting with a `slskd` (Soulseek daemon) instance. It handles searching for files on the Soulseek network and managing downloads.

- **`SoulseekClient`**:
  - **Description**: A client to communicate with the `slskd` API. It is configured with the API key, base URL, and download path.
  - **Methods**:
    - **`search(artist: String, album: String, tracks: Vec<Track>, timeout: Duration) -> Result<Vec<AlbumResult>, SoulfulError>`**: Searches for albums on Soulseek that match the provided metadata. It scores and ranks the results based on quality and availability.
    - **`download(tracks: Vec<TrackResult>) -> Result<Vec<DownloadResponse>, SoulfulError>`**: Initiates the download of a list of selected tracks.

#### `beets`

This module is intended for integration with the [Beets](https://beets.io/) music tagger. (Functionality not detailed as per user request).

---

## 3. `api` Crate

This crate exposes server-side functionality to the frontend using `dioxus` server functions. It acts as a bridge between the UI components and the core application logic implemented in the `soulful` crate.

### Server Functions

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

### Data Structures

- **`SearchQuery`**: A struct used for sending search queries to the server. It includes optional fields for `artist` and the main `query`.

---

## 4. `ui` Crate

This crate contains all the shared UI components for the application, built using the `dioxus` framework. These components are designed to be reusable and modular, forming the building blocks of the user interface.

### Core Components

#### `Navbar`

- **Description**: A simple container component that wraps the navigation bar.
- **Usage**: It takes children elements to be rendered inside the navbar.

#### `Button`

- **Description**: A customizable button component with support for different visual styles and states.
- **Props**:
  - `children`: The content to be displayed inside the button.
  - `onclick`: An event handler for click events.
  - `variant`: An enum (`ButtonVariant`) that defines the button's style (`Primary` or `Secondary`).
  - `disabled`: A boolean to disable the button.

#### `CoverArt`

- **Description**: A component to display album cover art. It gracefully handles image loading errors by showing a fallback icon.
- **Props**:
  - `src`: The URL of the image to display.
  - `alt`: The alternative text for the image.

#### `Modal`

- **Description**: A modal dialog component that can be used to display content in a layer above the main interface.
- **Props**:
  - `on_close`: An event handler that is called when the modal is requested to be closed.
  - `children`: The content to be displayed inside the modal.
  - `header`: The header content of the modal.

### Album Components

The `album` module contains components related to displaying album information and handling track selection.

#### `Album`

- **Description**: The main component for displaying an album's details, including its tracklist. It allows users to select tracks for download.
- **Props**:
  - `data`: An `AlbumWithTracks` struct containing the album and track information.
  - `on_select`: An event handler that is called when the user confirms their track selection. It passes a `DownloadQuery` with the selected tracks.

#### Sub-components

- **`AlbumHeader`**: Displays the album's title, artist, and other metadata.
- **`TrackList`**: Renders the list of tracks for an album, allowing for individual track selection and a "select all" option.
- **`AlbumFooter`**: Contains the actions related to the album, such as the download button, which is enabled only when at least one track is selected.

### Search Components

The `search` module provides components for searching for music.

- **`AlbumResult`**: Displays a single album in a search result list.
- **`TrackResult`**: Displays a single track in a search result list.

---

## 5. `shared` Crate

The `shared` crate contains the data structures that are used across the entire application, ensuring data consistency between the `api`, `ui`, and `soulful` crates. All models are serializable and deserializable using `serde`.

### Modules

#### `musicbrainz`

This module defines the data structures for interacting with the MusicBrainz API.

- **`SearchResult`**: An enum that represents a search result, which can be either a `Track` or an `Album`.
- **`Track`**: A struct containing detailed information about a track, including its title, artist, and album details.
- **`Album`**: A struct holding information about an album, such as its title, artist, and release date.
- **`AlbumWithTracks`**: A composite struct that combines `Album` information with a list of its `Track`s.

#### `slskd`

This module contains the data structures for interacting with the `slskd` (Soulseek daemon) API.

- **`DownloadRequest`**: Represents a request to download a file from a Soulseek user.
- **`DownloadResponse`**: Contains the response from a download request, including the download ID.
- **`SearchResult`**: Holds information about a file found on the Soulseek network, including details about the user, file size, and quality metrics.
- **`TrackResult`**: A struct that combines a `SearchResult` with matched metadata like artist, title, and album.
- **`AlbumResult`**: Represents a potential album download, containing a list of `TrackResult`s and aggregated metadata.

#### `download`

This module defines the structure for download queries.

- **`DownloadQuery`**: A struct that encapsulates a download request, containing the `Album` and a list of `Track`s to be downloaded.
