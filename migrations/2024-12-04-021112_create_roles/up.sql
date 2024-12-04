CREATE TABLE tbl_roles (
    id SERIAL PRIMARY KEY,

    name VARCHAR(255) NOT NULL,

    can_insert_student BOOLEAN DEFAULT FALSE,
    can_update_student BOOLEAN DEFAULT FALSE,
    can_delete_student BOOLEAN DEFAULT FALSE,

    can_insert_teacher BOOLEAN DEFAULT FALSE,
    can_update_teacher BOOLEAN DEFAULT FALSE,
    can_delete_teacher BOOLEAN DEFAULT FALSE,

    can_insert_class BOOLEAN DEFAULT FALSE,
    can_update_class BOOLEAN DEFAULT FALSE,
    can_delete_class BOOLEAN DEFAULT FALSE,

    can_insert_semester BOOLEAN DEFAULT FALSE,
    can_update_semester BOOLEAN DEFAULT FALSE,
    can_delete_semester BOOLEAN DEFAULT FALSE,

    is_moderator BOOLEAN DEFAULT FALSE,
    is_teacher BOOLEAN DEFAULT FALSE,
    is_student BOOLEAN DEFAULT TRUE,

    created_at TIMESTAMP DEFAULT NOW()
);