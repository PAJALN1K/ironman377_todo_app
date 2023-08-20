// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 20]
        state -> Varchar,
        #[max_length = 20]
        importance -> Varchar,
        #[max_length = 20]
        lifesphere -> Varchar,
    }
}
