CREATE TABLE tbl_majors (
    id SERIAL PRIMARY KEY,

    name VARCHAR(255) NOT NULL,
    code VARCHAR(255) NOT NULL,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    semester_id INTEGER NOT NULL REFERENCES tbl_semester(id)
);