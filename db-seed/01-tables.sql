-- ===============================================
-- 01‐tables.sql  (compatible with MariaDB/MySQL & PostgreSQL)
-- ===============================================

-- ------------------------------------------------
-- 1) users table
-- ------------------------------------------------
CREATE TABLE users (
    id           VARCHAR(36)    PRIMARY KEY,
    username     VARCHAR(64)    NOT NULL UNIQUE,
    email        VARCHAR(128)   NOT NULL,
    created_by   VARCHAR(36),
    created_at   TIMESTAMP      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified_by  VARCHAR(36),
    modified_at  TIMESTAMP      NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Separate index for email lookup
CREATE INDEX idx_users_email ON users(email);


-- ------------------------------------------------
-- 2) devices table
-- ------------------------------------------------
CREATE TABLE devices (
    id           VARCHAR(36)    PRIMARY KEY,
    user_id      VARCHAR(36)    NOT NULL,
    name         VARCHAR(128)   NOT NULL,
    status       VARCHAR(32)    NOT NULL,
    device_os    VARCHAR(16)    NOT NULL,
    registered_at TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by   VARCHAR(36),
    created_at   TIMESTAMP      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified_by  VARCHAR(36),
    modified_at  TIMESTAMP      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- enforce unique (user_id, name)
    UNIQUE (user_id, name),

    -- foreign key → users.id
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Index to speed up lookups by user_id
CREATE INDEX idx_devices_user_id ON devices(user_id);


-- ------------------------------------------------
-- 3) uploaded_files table
-- ------------------------------------------------
CREATE TABLE uploaded_files (
    id                VARCHAR(36)  PRIMARY KEY,
    user_id           VARCHAR(36)  NOT NULL,
    file_name         VARCHAR(128) NOT NULL,  -- stored/generated file name
    origin_file_name  VARCHAR(128) NOT NULL,  -- original filename from upload
    file_relative_path VARCHAR(256) NOT NULL,
    file_url          VARCHAR(256) NOT NULL,
    content_type      VARCHAR(64)  NOT NULL,
    file_size         BIGINT       NOT NULL,  -- use BIGINT for “unsigned” 
    file_type         VARCHAR(16)  NOT NULL,

    created_by        VARCHAR(36),
    created_at        TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified_by       VARCHAR(36),
    modified_at       TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- foreign key → users.id
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- If you want a composite key (e.g. one file_name per user), uncomment and adjust:
--   UNIQUE (user_id, file_name);


-- ------------------------------------------------
-- 4) user_auth table
-- ------------------------------------------------
CREATE TABLE user_auth (
    user_id       VARCHAR(36)  PRIMARY KEY,
    password_hash VARCHAR(255) NOT NULL,
    created_at    TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified_at   TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- FK to users.id
    FOREIGN KEY (user_id) REFERENCES users(id)
);

