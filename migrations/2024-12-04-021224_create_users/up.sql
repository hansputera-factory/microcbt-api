CREATE TABLE tbl_users (
    id SERIAL PRIMARY KEY,

    name VARCHAR(255) NOT NULL,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,

    is_active BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    role_id INTEGER NOT NULL REFERENCES tbl_roles(id),
    semester_id INTEGER NOT NULL REFERENCES tbl_semester(id)
);