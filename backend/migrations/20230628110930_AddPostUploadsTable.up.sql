-- Add up migration script here
CREATE TABLE IF NOT EXISTS "post_upload" (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  post_id uuid NOT NULL,
  upload_id uuid NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  CONSTRAINT fk_post_upload_post_id
    FOREIGN KEY (post_id)
      REFERENCES "post"(id),
  CONSTRAINT fk_post_upload_upload_id
    FOREIGN KEY (upload_id)
      REFERENCES "upload"(id)
);
