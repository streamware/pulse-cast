-- Your SQL goes here
CREATE TABLE "devices" (
    "id" TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    "owner" TEXT NOT NULL UNIQUE REFERENCES "users"("id"),
    "device_name" TEXT NOT NULL,
    "device_type" TEXT NOT NULL,
    "device_token" TEXT NOT NULL,
    "os_version" TEXT NOT NULL,
    "enabled" BOOLEAN NOT NULL DEFAULT 1,
    "created_at" TEXT NOT NULL DEFAULT (datetime('now')),
    "updated_at" TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY ("owner") REFERENCES users("id")
);