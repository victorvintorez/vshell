main_info_starting-vshell = Starter vShell {$pkgversion}...
main_info_vshell-running = vShell kjører allerede
main_err_style-path-fail = Kunne ikke få stilbane
main_debug_style-file-load = Bruker style.{$extension} fra: {$path}/
main_err_style-file-fail = Kunne ikke finne style.sass, style.scss, eller style.css i {$path}/
main_expect_style-parent-dir = å ha overordnet katalog
main_expect_style-current-dir = for å ha gjeldende katalog
main_expect_rx-receive-signal = for å motta signal på kanalen
main_info_vshell-shutdown = Slår av vShell...
main_expect_tx-send-signal = for å sende signal på kanalen
main_expect_ctrl-c-handler = for å sette ctrl-c handler
main_expect_tokio-runtime = for å lage tokio runtime

config_error_config-parent-dir = Kunne ikke hente overordnet katalog for konfigurasjonsfilen
config_error_config-file-fail = Kunne ikke laste inn konfigurasjonen {$error}
config_warn_using-default-config = Bruker standard konfig
config_debug_config-file-load = Bruker config.toml fra: {$path}/

logging_error_panic = Panikk: {$error}

macros_error_channel-send-fail = Kunne ikke sende melding på kanalen

architecture-theme-style_expect_style-path-current-dir = gjeldende dir til å eksistere
architecture-theme-style_expect_gdk-display-default = for å returnere standardvisning
architecture-theme-style_debug_style-watch-event = Stil Overvåker-arrangement: {$event}
architecture-theme-style_error_style-watch-fail = Feil under visning av stilfil: {$error}
architecture-theme-style_expect_build-style-watcher = å bygge stilfilovervåker
architecture-theme-style_expect_style-path-parent-dir = overordnet dir til å eksistere
architecture-theme-style_expect_start-style-watcher = for å starte stilfilovervåkeren
architecture-theme-style_debug_style-file-watching = Ser på stilfil: {$path}
architecture-theme-style_info_style-file-reloading = Laster inn stilfil på nytt: {$path}
architecture-theme-style_error_style-file-load-sass = Kunne ikke laste inn style.sass: {$error}
architecture-theme-style_error_style-file-load-scss = Kunne ikke laste inn style.scss: {$error}
architecture-theme-style_error_style-file-load-css = Kunne ikke laste inn style.css: {$error}

architecture-ipc_warn_ipc-socket-length = IPC-socketfilens absolutte bane er for lang. Dette kan forårsake problemer med enkelte systemer

architecture-ipc-server_warn_ipc-socket-exists = IPC-socket-filen eksisterer allerede. Fjerner den
architecture-ipc-server_info_ipc-socket-starting = Starter IPC-kontakten på {$path}
architecture-ipc-server_error_ipc-socket-bind-fail = Kunne ikke binde IPC-socket: {$error}
architecture-ipc-server_error_handle-connection-fail = Kunne ikke håndtere tilkoblingen: {$error}
architecture-ipc-server_error_ipc-stream-accept-fail = IPC-strømtilkoblingsfeil: {$error}
architecture-ipc-server_debug_ipc-received-request = IPC mottatt forespørsel: {$request}
architecture-ipc-server_error_ipc-shutdown-fail = Kunne ikke fjerne IPC-socket-filen: {$error}

architecture-ipc-server-debug_string_pong = Pong

architecture-ipc-client_error_ipc-socket-connect-fail = Kunne ikke koble til IPC-kontakten
architecture-ipc-client_error_ipc-socket-connect-suggestion = Sørg for at hoved-vShell-forekomsten kjører
architecture-ipc-client_debug_ipc-sent-request = IPC sendt forespørsel: {$request}