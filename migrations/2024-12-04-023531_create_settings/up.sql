CREATE TABLE tbl_settings (
    id SERIAL PRIMARY KEY,

    name VARCHAR(255) NOT NULL,
    school_name VARCHAR(255) NOT NULL,
    school_address TEXT NOT NULL,
    village VARCHAR(255) NOT NULL,
    district VARCHAR(255) NOT NULL,
    city VARCHAR(255) NOT NULL,
    province VARCHAR(255) NOT NULL,
    postal_code VARCHAR(255),

    fax VARCHAR(255),
    registered_id VARCHAR(255) NOT NULL,
    website VARCHAR(255),
    email VARCHAR(255),
    phone VARCHAR(255),

    headmaster_name VARCHAR(255) NOT NULL,
    headmaster_id VARCHAR(255),

    signature_photo_url VARCHAR(255),
    school_logo_url VARCHAR(255),
    app_logo_url VARCHAR(255),

    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);