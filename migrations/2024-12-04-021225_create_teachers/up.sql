CREATE TABLE tbl_teachers (
    id SERIAL PRIMARY KEY,

    name VARCHAR(255) NOT NULL,
    registered_id VARCHAR(255),

    user_id INTEGER REFERENCES tbl_users(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    semester_id INTEGER NOT NULL REFERENCES tbl_semester(id)
);