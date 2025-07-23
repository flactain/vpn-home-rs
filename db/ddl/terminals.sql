-- public.terminals definition

-- Drop table

-- DROP TABLE terminals;

CREATE TABLE terminals (
    terminal_id UUID NOT NULL,
    terminal_name varchar(32) NOT NULL,
    owner_user_id varchar(32) NOT NULL,
    os varchar(32) NULL,
    is_deleted bool DEFAULT false NOT NULL,
    created_at timestamp NULL,
    updated_at timestamp NULL,
    PRIMARY KEY (terminal_id)
);
COMMENT ON TABLE terminals IS '端末';

ALTER TABLE public.terminals ADD CONSTRAINT terminals_users_fk FOREIGN KEY (owner_user_id) REFERENCES users(user_id)ON DELETE CASCADE ON UPDATE CASCADE;

COMMENT ON COLUMN terminals.terminal_id   IS '端末ID';
COMMENT ON COLUMN terminals.terminal_name IS '端末名';
COMMENT ON COLUMN terminals.owner_user_id IS '所有者ユーザーID';
COMMENT ON COLUMN terminals.os            IS 'OS';
