CREATE TABLE todos(
    "id" UUID PRIMARY KEY NOT NULL,
    "author_id" UUID NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "completed_at" TIMESTAMP WITH TIME ZONE,
    "message" TEXT NOT NULL
);
