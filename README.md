# Project Architecture

## Architecture:
The architecture module contains all base modules required throughout the project. It contains the following modules:
**CLI**: This module contains the command line interface for the project. It is responsible for parsing the command line arguments and executing the appropriate commands.
**i18n**: This module contains the internationalization module for the project. It is responsible for translating the project into different languages.
**IPC**: This module contains the inter-process communication module for the project. It is responsible for handling communication between different processes.
**Logging**: This module contains the logging module for the project. It is responsible for logging the project's activities.
**Storage**: This module contains the storage module for the project. It is responsible for storing and retrieving data from the project's database and datafiles.
**Theme**: This module contains the theme module for the project. It is responsible for storing, managing, and setting the project's theme.

## Components:
The components module contains custom GTK components following the Material Design 3 guidelines. It contains the following components:

## Modules:
The modules module contains all the basic sections for each UI screen. It contains the following modules:

## Services:
The services module contains all the services required for each module/screen the project. It contains the following modules:

## Windows:
The windows module contains all the top level windows for the project. It contains the following modules:
**Applications**: This module contains all standalone windows, and contains the following Applications:
- **dbus-viewer**: This application is responsible for viewing the D-Bus services and objects on the system.
- **icon-viewer**: This application is responsible for viewing the icons available on the system.
- **keybind-viewer**: This application is responsible for viewing the keybindings set by the user of the system.
- **settings-app**: This application is responsible for viewing and changing the settings of the application and system.
**Layers**: This module contains all layers for the project, and contains the following Layers:
- **applauncher**: This layer is responsible for application launching as well as many other features.
- **sidebar**: This layer is responsible for the sidebar tools panel.
- **topbar**: This layer is responsible for the main desktop bar.