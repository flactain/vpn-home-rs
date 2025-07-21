CREATE TABLE terminals (
  terminal_id UUID NOT NULL,
  terminal_name VARCHAR(32) NOT NULL,
  owner_user_id CHAR(32) NOT NULL,
  os VARCHAR(32),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (terminal_id)
);

COMMENT ON TABLE terminals IS '端末';

COMMENT ON COLUMN terminals.terminal_id   IS '端末ID';
COMMENT ON COLUMN terminals.terminal_name IS '端末名';
COMMENT ON COLUMN terminals.owner_user_id IS '所有者ユーザーID';
COMMENT ON COLUMN terminals.os            IS 'OS';
