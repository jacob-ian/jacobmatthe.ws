-- Add up migration script here
CREATE TABLE IF NOT EXISTS "email_verification" (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id uuid NOT NULL,
  code varchar UNIQUE NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  expires_at timestamptz NOT NULL DEFAULT now() + INTERVAL '5 minutes',
  CONSTRAINT fk_user_id_user
    FOREIGN KEY(user_id)
      REFERENCES "user"(id)
);
