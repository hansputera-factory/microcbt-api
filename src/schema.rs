// @generated automatically by Diesel CLI.

diesel::table! {
    tbl_auth_logs (id) {
        id -> Int4,
        #[max_length = 45]
        client_ip -> Varchar,
        client_user_agent -> Text,
        client_device -> Text,
        client_os -> Text,
        client_browser -> Text,
        user_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tbl_class (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        grade -> Int4,
        students_count -> Int4,
        created_at -> Nullable<Timestamp>,
        major_id -> Int4,
        semester_id -> Int4,
    }
}

diesel::table! {
    tbl_logs (id) {
        id -> Int4,
        message -> Text,
        user_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tbl_majors (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        code -> Varchar,
        created_at -> Nullable<Timestamp>,
        semester_id -> Int4,
    }
}

diesel::table! {
    tbl_roles (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        can_insert_student -> Nullable<Bool>,
        can_update_student -> Nullable<Bool>,
        can_delete_student -> Nullable<Bool>,
        can_insert_teacher -> Nullable<Bool>,
        can_update_teacher -> Nullable<Bool>,
        can_delete_teacher -> Nullable<Bool>,
        can_insert_class -> Nullable<Bool>,
        can_update_class -> Nullable<Bool>,
        can_delete_class -> Nullable<Bool>,
        can_insert_semester -> Nullable<Bool>,
        can_update_semester -> Nullable<Bool>,
        can_delete_semester -> Nullable<Bool>,
        is_moderator -> Nullable<Bool>,
        is_teacher -> Nullable<Bool>,
        is_student -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tbl_semester (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        code -> Varchar,
        created_at -> Nullable<Timestamp>,
        is_active -> Nullable<Bool>,
    }
}

diesel::table! {
    tbl_settings (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        school_name -> Varchar,
        school_address -> Text,
        #[max_length = 255]
        village -> Varchar,
        #[max_length = 255]
        district -> Varchar,
        #[max_length = 255]
        city -> Varchar,
        #[max_length = 255]
        province -> Varchar,
        #[max_length = 255]
        postal_code -> Nullable<Varchar>,
        #[max_length = 255]
        fax -> Nullable<Varchar>,
        #[max_length = 255]
        registered_id -> Varchar,
        #[max_length = 255]
        website -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        phone -> Nullable<Varchar>,
        #[max_length = 255]
        headmaster_name -> Varchar,
        #[max_length = 255]
        headmaster_id -> Nullable<Varchar>,
        #[max_length = 255]
        signature_photo_url -> Nullable<Varchar>,
        #[max_length = 255]
        school_logo_url -> Nullable<Varchar>,
        #[max_length = 255]
        app_logo_url -> Nullable<Varchar>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tbl_students (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        national_student_id -> Varchar,
        #[max_length = 255]
        school_student_id -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        user_id -> Nullable<Int4>,
        class_id -> Int4,
        semester_id -> Int4,
    }
}

diesel::table! {
    tbl_subject_groups (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        is_universal -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        semester_id -> Int4,
    }
}

diesel::table! {
    tbl_subjects (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        code -> Varchar,
        is_active -> Nullable<Bool>,
        subject_group_id -> Int4,
        created_at -> Nullable<Timestamp>,
        semester_id -> Int4,
    }
}

diesel::table! {
    tbl_teachers (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        registered_id -> Nullable<Varchar>,
        user_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        semester_id -> Int4,
    }
}

diesel::table! {
    tbl_users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        role_id -> Int4,
        semester_id -> Int4,
    }
}

diesel::joinable!(tbl_auth_logs -> tbl_users (user_id));
diesel::joinable!(tbl_class -> tbl_majors (major_id));
diesel::joinable!(tbl_class -> tbl_semester (semester_id));
diesel::joinable!(tbl_logs -> tbl_users (user_id));
diesel::joinable!(tbl_majors -> tbl_semester (semester_id));
diesel::joinable!(tbl_students -> tbl_class (class_id));
diesel::joinable!(tbl_students -> tbl_semester (semester_id));
diesel::joinable!(tbl_students -> tbl_users (user_id));
diesel::joinable!(tbl_subject_groups -> tbl_semester (semester_id));
diesel::joinable!(tbl_subjects -> tbl_semester (semester_id));
diesel::joinable!(tbl_subjects -> tbl_subject_groups (subject_group_id));
diesel::joinable!(tbl_teachers -> tbl_semester (semester_id));
diesel::joinable!(tbl_teachers -> tbl_users (user_id));
diesel::joinable!(tbl_users -> tbl_roles (role_id));
diesel::joinable!(tbl_users -> tbl_semester (semester_id));

diesel::allow_tables_to_appear_in_same_query!(
    tbl_auth_logs,
    tbl_class,
    tbl_logs,
    tbl_majors,
    tbl_roles,
    tbl_semester,
    tbl_settings,
    tbl_students,
    tbl_subject_groups,
    tbl_subjects,
    tbl_teachers,
    tbl_users,
);
