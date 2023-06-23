-- Add up migration script here
CREATE TABLE IF NOT EXISTS "post" (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  author_id uuid NOT NULL,
  stub varchar NOT NULL,
  title varchar NOT NULL,
  description varchar NOT NULL,
  content text NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now(),
  published_at timestamptz,
  deleted_at timestamptz,
  CONSTRAINT fk_author_id_user
    FOREIGN KEY(author_id)
      REFERENCES "user"(id)
);
