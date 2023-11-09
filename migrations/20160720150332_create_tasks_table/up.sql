CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description VARCHAR NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 0
);

CREATE TABLE "pallete" (
	"id" INTEGER NOT NULL DEFAULT 0 UNIQUE,
	"r"	INTEGER NOT NULL DEFAULT 0,
	"g"	INTEGER NOT NULL DEFAULT 0,
	"b"	INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY("id")
)

CREATE TABLE "pixels" (
	"x"	INTEGER NOT NULL UNIQUE,
	"y"	INTEGER NOT NULL UNIQUE,
	"color"	INTEGER NOT NULL DEFAULT 0,
	"user"	INTEGER NOT NULL DEFAULT -1,
	"time"	INTEGER NOT NULL,
	PRIMARY KEY("x","y")
)

CREATE TABLE "users" (
	"db_id"	INTEGER NOT NULL UNIQUE,
	"id"	TEXT NOT NULL UNIQUE,
	"kind"	TEXT,
	"preferred_username"	TEXT,
	"name"	TEXT,
	"inbox"	TEXT,
	"outbox"	TEXT,
	"public_key"	TEXT,
	PRIMARY KEY("db_id" AUTOINCREMENT)
)

INSERT INTO tasks (description) VALUES ("demo task");
INSERT INTO tasks (description) VALUES ("demo task2");
