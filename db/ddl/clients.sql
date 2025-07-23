-- public.clients definition

-- Drop table

-- DROP TABLE clients;

CREATE TABLE clients (
    vpn_id uuid NOT NULL,
    terminal_id uuid NOT NULL,
    allowed_ip inet NULL,
    public_key text NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP NULL,
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP NULL,
    is_deleted bool DEFAULT false NOT NULL,
    PRIMARY KEY(vpn_id, terminal_id)
);
COMMENT ON TABLE public.clients IS 'クライアント';

-- public.clients foreign keys
ALTER TABLE public.clients ADD CONSTRAINT clients_terminals_fk FOREIGN KEY (terminal_id) REFERENCES terminals(terminal_id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE public.clients ADD CONSTRAINT vpns_clients_fkey FOREIGN KEY (vpn_id) REFERENCES vpns(vpn_id) ON DELETE CASCADE ON UPDATE CASCADE;

-- Column comments

COMMENT ON COLUMN public.clients.vpn_id IS 'VPN ID';
COMMENT ON COLUMN public.clients.terminal_id IS '端末ID';
COMMENT ON COLUMN public.clients.allowed_ip IS 'IP';
COMMENT ON COLUMN public.clients.public_key IS 'パブリックキー';