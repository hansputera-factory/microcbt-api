CREATE TABLE tbl_students (
    id SERIAL PRIMARY KEY,

    name VARCHAR(255) NOT NULL,
    national_student_id VARCHAR(255) NOT NULL,
    school_student_id VARCHAR(255),

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    user_id INTEGER REFERENCES tbl_users(id),
    class_id INTEGER NOT NULL REFERENCES tbl_class(id),
    semester_id INTEGER NOT NULL REFERENCES tbl_semester(id)
);