# UI Crate

This crate contains all the shared UI components for the application, built using the `dioxus` framework. These components are designed to be reusable and modular, forming the building blocks of the user interface.

## Core Components

### `Navbar`

- **Description**: A simple container component that wraps the navigation bar.
- **Usage**: It takes children elements to be rendered inside the navbar.

### `Button`

- **Description**: A customizable button component with support for different visual styles and states.
- **Props**:
  - `children`: The content to be displayed inside the button.
  - `onclick`: An event handler for click events.
  - `variant`: An enum (`ButtonVariant`) that defines the button's style (`Primary` or `Secondary`).
  - `disabled`: A boolean to disable the button.

### `CoverArt`

- **Description**: A component to display album cover art. It gracefully handles image loading errors by showing a fallback icon.
- **Props**:
  - `src`: The URL of the image to display.
  - `alt`: The alternative text for the image.

### `Modal`

- **Description**: A modal dialog component that can be used to display content in a layer above the main interface.
- **Props**:
  - `on_close`: An event handler that is called when the modal is requested to be closed.
  - `children`: The content to be displayed inside the modal.
  - `header`: The header content of the modal.

## Album Components

The `album` module contains components related to displaying album information and handling track selection.

### `Album`

- **Description**: The main component for displaying an album's details, including its tracklist. It allows users to select tracks for download.
- **Props**:
  - `data`: An `AlbumWithTracks` struct containing the album and track information.
  - `on_select`: An event handler that is called when the user confirms their track selection. It passes a `DownloadQuery` with the selected tracks.

### Sub-components

- **`AlbumHeader`**: Displays the album's title, artist, and other metadata.
- **`TrackList`**: Renders the list of tracks for an album, allowing for individual track selection and a "select all" option.
- **`AlbumFooter`**: Contains the actions related to the album, such as the download button, which is enabled only when at least one track is selected.

## Search Components

The `search` module provides components for searching for music.

- **`AlbumResult`**: Displays a single album in a search result list.
- **`TrackResult`**: Displays a single track in a search result list.
