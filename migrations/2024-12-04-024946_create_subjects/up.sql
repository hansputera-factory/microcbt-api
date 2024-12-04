CREATE TABLE tbl_subjects (
    id SERIAL PRIMARY KEY,

    name VARCHAR(255) NOT NULL,
    code VARCHAR(255) NOT NULL UNIQUE,
    is_active BOOLEAN DEFAULT TRUE,

    subject_group_id INTEGER NOT NULL REFERENCES tbl_subject_groups(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    semester_id INTEGER NOT NULL REFERENCES tbl_semester(id)
);