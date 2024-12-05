CREATE TABLE tbl_auth_logs (
    id SERIAL PRIMARY KEY,

    client_ip VARCHAR(45) NOT NULL,
    client_user_agent TEXT NOT NULL,
    client_device TEXT NOT NULL,
    client_os TEXT NOT NULL,
    client_browser TEXT NOT NULL,

    user_id INT NOT NULL REFERENCES tbl_users(id),
    created_at TIMESTAMP DEFAULT NOW()
)