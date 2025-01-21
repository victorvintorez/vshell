#[macro_export]
macro_rules! glib_recv_mpsc {
    ($rx:expr, $func:ident) => { glib_recv_mpsc!($rx, ev => $func(ev)) };

    ($rx:expr, $val:ident => $expr:expr) => {{
        glib::spawn_future_local(async move {
            let mut rx = $rx;
            while let Some($val) = rx.recv().await {
                $expr
            }
        });
    }};
}

#[macro_export]
macro_rules! try_send {
    ($tx:expr, $msg:expr) => {
        $tx.try_send($msg).expect(&*fl!("macros_error_channel-send-fail"))
    };
}

#[macro_export]
macro_rules! send {
    ($tx:expr, $msg:expr) => {
        $tx.send($msg).expect(&*fl!("macros_error_channel-send-fail"))
    };
}

#[macro_export]
macro_rules! send_async {
    ($tx:expr, $msg:expr) => {
        $tx.send($msg).await.expect(&*fl!("macros_error_channel-send-fail"))
    };
}

#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {
        ::i18n_embed_fl::fl!($crate::architecture::i18n::LANGUAGE_LOADER, $message_id)
    };
    ($message_id:literal, $($args:expr),*) => {
        ::i18n_embed_fl::fl!($crate::architecture::i18n::LANGUAGE_LOADER, $message_id, $($args),*)
    };
}