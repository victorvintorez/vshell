main_info_starting-vshell = Starting vShell {$pkgversion}...
main_info_vshell-running = vShell is already running
main_err_style-path-fail = Failed to get style path
main_debug_style-file-load = Using style.{$extension} from: {$path}/
main_err_style-file-fail = Failed to find style.sass, style.scss, or style.css in {$path}/
main_expect_style-parent-dir = to have parent directory
main_expect_style-current-dir = to have current directory
main_expect_rx-receive-signal = to recieve signal on channel
main_info_vshell-shutdown = Shutting down vShell...
main_expect_tx-send-signal = to send signal on channel
main_expect_ctrl-c-handler = to set ctrl-c handler
main_expect_tokio-runtime = to create tokio runtime

config_error_config-parent-dir = Failed to get parent directory of config file
config_error_config-file-fail = Failed to load config {$error}
config_warn_using-default-config = Using default config
config_debug_config-file-load = Using config.toml from: {$path}/

logging_error_panic = Panic: {$error}

macros_error_channel-send-fail = Failed to send message on channel

architecture-theme-style_expect_style-path-current-dir = current dir to exist
architecture-theme-style_expect_gdk-display-default = to return default display
architecture-theme-style_debug_style-watch-event = Style Watcher Event: {$event}
architecture-theme-style_error_style-watch-fail = Error watching style file: {$error}
architecture-theme-style_expect_build-style-watcher = to build style file watcher
architecture-theme-style_expect_style-path-parent-dir = parent dir to exist
architecture-theme-style_expect_start-style-watcher = to start style file watcher
architecture-theme-style_debug_style-file-watching = Watching style file: {$path}
architecture-theme-style_info_style-file-reloading = Reloading style file: {$path}
architecture-theme-style_error_style-file-load-sass = Failed to load style.sass: {$error}
architecture-theme-style_error_style-file-load-scss = Failed to load style.scss: {$error}
architecture-theme-style_error_style-file-load-css = Failed to load style.css: {$error}

architecture-ipc_warn_ipc-socket-length = The IPC socket file's absolute path is too long. This may cause issues with some systems

architecture-ipc-server_warn_ipc-socket-exists = The IPC socket file already exists. Removing it
architecture-ipc-server_info_ipc-socket-starting = Starting IPC socket on {$path}
architecture-ipc-server_error_ipc-socket-bind-fail = Failed to bind IPC socket: {$error}
architecture-ipc-server_error_handle-connection-fail = Failed to handle connection: {$error}
architecture-ipc-server_error_ipc-stream-accept-fail = IPC stream connection failure: {$error}
architecture-ipc-server_debug_ipc-received-request = IPC Received Request: {$request}
architecture-ipc-server_error_ipc-shutdown-fail = Failed to remove IPC socket file: {$error}

architecture-ipc-server-debug_string_pong = Pong

architecture-ipc-client_error_ipc-socket-connect-fail = Failed to connect to IPC socket
architecture-ipc-client_error_ipc-socket-connect-suggestion = Make sure the main vShell instance is running
architecture-ipc-client_debug_ipc-sent-request = IPC Sent Request: {$request}

architecture-storage-redb_debug_database-opened = Opened database: {$path}
architecture-storage-redb_error_database-open-fail = Failed to open database: {$error}
architecture-storage-redb_debug_database-created = Created database: {$path}
architecture-storage-redb_error_database-create-fail = Failed to create database: {$error}