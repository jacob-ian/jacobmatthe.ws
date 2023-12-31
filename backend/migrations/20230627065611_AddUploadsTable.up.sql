-- Add up migration script here
CREATE TABLE IF NOT EXISTS "upload" ( 
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  file_name varchar NOT NULL UNIQUE,
  file_type varchar NOT NULL,
  created_by uuid NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now(),
  uploaded_at timestamptz,
  deleted_at timestamptz,
  CONSTRAINT fk_upload_created_by
    FOREIGN KEY (created_by)
      REFERENCES "user"(id)
);
