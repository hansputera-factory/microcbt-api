CREATE TABLE tbl_subject_groups (
    id SERIAL PRIMARY KEY,

    name VARCHAR(255) NOT NULL,
    is_universal BOOLEAN DEFAULT FALSE,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    semester_id INTEGER NOT NULL REFERENCES tbl_semester(id)
);