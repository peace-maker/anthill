table! {
    team_key_values (team_id, key) {
        team_id -> Int4,
        key -> Text,
        value -> Text,
    }
}

table! {
    teams (id) {
        id -> Int4,
        name -> Nullable<Text>,
        state -> Int2,
    }
}

joinable!(team_key_values -> teams (team_id));

allow_tables_to_appear_in_same_query!(team_key_values, teams,);
