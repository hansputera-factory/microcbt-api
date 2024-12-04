CREATE TABLE tbl_semester (
    id SERIAL PRIMARY KEY,

    name VARCHAR(255) NOT NULL,
    code VARCHAR(255) NOT NULL,

    created_at TIMESTAMP DEFAULT NOW(),
    
    is_active BOOLEAN DEFAULT FALSE
);
