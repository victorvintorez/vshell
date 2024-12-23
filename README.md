# Project Architecture

## Clients Folder
Contains anything that hooks into an external resource or service. This includes interactions with wayland, ollama, etc.

## Components Folder
This contains custom components or meta-components (components made out of base components) that are not a window or module.

## Modules Folder
These are entire modules that have a single explicit purpose i.e. a clock, workspace switcher, etc. These are not windows or components.

## Windows Folder
These are full windows, ie topbar, applauncher, notification view, etc. These are not components or modules.
