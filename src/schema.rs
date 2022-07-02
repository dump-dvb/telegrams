
table! {
    r09_save_telegram (id) {
        id -> BigSerial,
        time -> Timestamp,
        station -> Uuid,
        region -> BigInt,
        telegram_type -> TinyInt,
        delay -> Nullable<Integer>,
        reporting_point -> Integer,
        junction -> Integer,
        direction -> TinyInt,
        request_status -> TinyInt,
        priority -> Nullable<TinyInt>,
        direction_request -> Nullable<TinyInt>,
        line -> Nullable<Integer>,
        run_number -> Nullable<Integer>,
        destination_number -> Nullable<Integer>,
        train_length -> Nullable<TinyInt>,
        vehicle_number -> Nullable<Integer>,
        operator -> Nullable<TinyInt>,
    }
}
