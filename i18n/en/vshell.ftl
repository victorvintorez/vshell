main_info_starting-vshell = Starting vShell {$pkgversion}...
main_info_vshell-running = vShell is already running
main_err_style-path-fail = Failed to get style path
main_debug_style-file-load = Using style.{$extension} from: {$path}/
main_err_style-file-fail = Failed to find style.sass, style.scss, or style.css in {$path}/
main_expect_style-parent-dir = to have parent directory
main_expect_style-current-dir = to have current directory
main_expect_rx-receive-signal = to receive signal on channel
main_info_vshell-shutdown = Shutting down vShell...
main_expect_tx-send-signal = to send signal on channel
main_expect_ctrl-c-handler = to set ctrl-c handler
main_expect_tokio-runtime = to create tokio runtime
main_info_theme-initial-load = Theme Loaded: Warnings: {$warn}
main_error_theme-load-fail = Failed to load theme: {$error}

config_error_config-parent-dir = Failed to get parent directory of config file
config_error_config-file-fail = Failed to load config {$error}
config_warn_using-default-config = Using default config
config_debug_config-file-load = Using config.toml from: {$path}/

logging_error_panic = Panic: {$error}

macros_error_channel-send-fail = Failed to send message on channel

shell_info_running-command = Running command: {$cmd}
shell_expect_command-success-result = to return output
shell_info_command-success = Command succeeded: {$cmd}, Result: {$result}
shell_expect_command-fail-result = to return error
shell_warn_command-fail = Command failed: {$cmd}, Result: {$result}
shell_warn_command-not-run = Command could not be run: {$cmd}, Error: {$error}

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

architecture-theme-md3_expect_wallpaper-read = to read wallpaper file
architecture-theme-md3_expect_image-read = to parse image file
architecture-theme-template_info_templates-loading = Loading {$count} templates.
architecture-theme-template_warn_template-not-found = Template for {$name} not found at {$path}
architecture-theme-template_expect_template-target-dir = target to have parent directory
architecture-theme-template_error_template-target-dir-fail = Failed to get target directory for template {$name}: {$path}
architecture-theme-template_info_pre-hook-running = Running pre-hook for template {$name}: {$hook}
architecture-theme-template_error_pre-hook-output-fail = Failed to get output from pre-hook for template {$name}: {$hook}: {$error}
architecture-theme-template_warn_pre-hook-result-success = Pre-hook for template {$name}: {$hook} returned success: {$output}
architecture-theme-template_error_pre-hook-result-fail = Pre-hook for template {$name}: {$hook} returned failure: {$output}
architecture-theme-template_info_post-hook-running = Running post-hook for template {$name}: {$hook}
architecture-theme-template_error_post-hook-output-fail = Failed to get output from post-hook for template {$name}: {$hook}: {$error}
architecture-theme-template_warn_post-hook-result-success = Post-hook for template {$name}: {$hook} returned success: {$output}
architecture-theme-template_error_post-hook-result-fail = Post-hook for template {$name}: {$hook} returned failure: {$output}