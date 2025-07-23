CREATE TABLE users (
  user_id CHAR(32),
  role    CHAR(16) NOT NULL,
  last_logged_in TIMESTAMP,
  is_deleted boolean DEFAULT FALSE NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY(user_id)
);

COMMENT ON TABLE users IS 'ユーザー';

COMMENT ON COLUMN users.user_id IS 'ユーザーID';
COMMENT ON COLUMN users.role IS 'ロール';
COMMENT ON COLUMN users.last_logged_in IS '最終ログイン';
