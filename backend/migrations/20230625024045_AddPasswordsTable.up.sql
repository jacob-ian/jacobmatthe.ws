-- Add up migration script here
CREATE TABLE IF NOT EXISTS "user_credential" (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id uuid NOT NULL UNIQUE,
  password_hash varchar NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now(),
  CONSTRAINT fk_user_id_user
    FOREIGN KEY(user_id)
      REFERENCES "user"(id)
);

