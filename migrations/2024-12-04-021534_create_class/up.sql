CREATE TABLE tbl_class (
    id SERIAL PRIMARY KEY,

    name VARCHAR(255) NOT NULL,
    grade INTEGER NOT NULL,
    students_count INTEGER NOT NULL,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    major_id INTEGER NOT NULL REFERENCES tbl_majors(id),
    semester_id INTEGER NOT NULL REFERENCES tbl_semester(id)
);