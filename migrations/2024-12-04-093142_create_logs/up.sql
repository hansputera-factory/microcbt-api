CREATE TABLE tbl_logs (
    id SERIAL PRIMARY KEY,
    
    message TEXT NOT NULL,

    user_id INT REFERENCES tbl_users(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT NOW()
)