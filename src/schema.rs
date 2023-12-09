// @generated automatically by Diesel CLI.

diesel::table! {
    lists (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        status -> Varchar,
        date -> Timestamp,
        theme -> Nullable<Varchar>,
        due_date -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    subtask_mapping (task_id, dependent_id) {
        task_id -> Uuid,
        dependent_id -> Uuid,
    }
}

diesel::table! {
    task_list_mapping (task_id, list_id) {
        task_id -> Uuid,
        list_id -> Uuid,
    }
}

diesel::table! {
    tasks (id) {
        id -> Uuid,
        user_id -> Uuid,
        task -> Varchar,
        done -> Bool,
        status -> Varchar,
        task_type -> Varchar,
        details -> Text,
        priority -> Nullable<Varchar>,
        progress -> Nullable<Float8>,
        tags -> Array<Nullable<Text>>,
        theme -> Nullable<Varchar>,
        due_date -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        hashed_password -> Varchar,
        timezone -> Varchar,
        role -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::joinable!(lists -> users (user_id));
diesel::joinable!(task_list_mapping -> lists (list_id));
diesel::joinable!(task_list_mapping -> tasks (task_id));
diesel::joinable!(tasks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    lists,
    subtask_mapping,
    task_list_mapping,
    tasks,
    users,
);
