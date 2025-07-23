-- public.servers definition

-- Drop table

-- DROP TABLE servers;

CREATE TABLE servers (
    vpn_id uuid NOT NULL, -- VPN ID
    terminal_id uuid NOT NULL, -- 端末ID
    public_ip inet NOT NULL, -- パブリックIP
    config_name text NULL, -- configファイル名
    public_key text NOT NULL, -- パブリックキー
    keep_alive_second int4 NULL, -- keep-alive時間
    post_up_command text NULL, -- postUpのコマンド
    post_down_command text NULL, -- post_downのコマン
    is_deleted bool DEFAULT false NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP NULL,
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP NULL,
    PRIMARY KEY(vpn_id)
);
COMMENT ON TABLE public.servers IS 'サーバー';

-- public.servers foreign keys

ALTER TABLE public.servers ADD CONSTRAINT servers_terminals_fk FOREIGN KEY (terminal_id) REFERENCES terminals(terminal_id)ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE public.servers ADD CONSTRAINT servers_vpns_fk FOREIGN KEY (vpn_id) REFERENCES vpns(vpn_id) ON DELETE CASCADE ON UPDATE CASCADE;

-- Column comments

COMMENT ON COLUMN public.servers.vpn_id IS 'VPN ID';
COMMENT ON COLUMN public.servers.terminal_id IS '端末ID';
COMMENT ON COLUMN public.servers.public_ip IS 'パブリックIP';
COMMENT ON COLUMN public.servers.config_name IS 'configファイル名';
COMMENT ON COLUMN public.servers.public_key IS 'パブリックキー';
COMMENT ON COLUMN public.servers.keep_alive_second IS 'keep-alive時間';
COMMENT ON COLUMN public.servers.post_up_command IS 'postUpのコマンド';
COMMENT ON COLUMN public.servers.post_down_command IS 'post_downのコマンド';