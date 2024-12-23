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
        $tx.try_send($msg).expect($crate::error::ERR_CHANNEL_SEND)
    };
}