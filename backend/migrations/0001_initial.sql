CREATE TABLE "user"
(
    "id"            UUID DEFAULT uuid_generate_v4() NOT NULL,
    "username"      VARCHAR(255)                    NOT NULL,
    "chips"         BIGINT                          NOT NULL,
    "premium_chips" BIGINT                          NOT NULL,
    "xp"            BIGINT                          NOT NULL,
    "rank"          int                             NOT NULL
);

ALTER TABLE
    "user"
    ADD PRIMARY KEY ("id");

ALTER TABLE
    "user"
    ADD CONSTRAINT "user_username_unique" UNIQUE ("username");

create table "friendship"
(
    "id"      UUID DEFAULT uuid_generate_v4()                                     not null,
    requester UUID                                                                not null,
    responder UUID                                                                not null,
    "status"  VARCHAR(255) check ( "status" in ('pending', 'active', 'blocked') ) not null
);

ALTER TABLE
    "friendship"
    ADD PRIMARY KEY ("id");

alter table "friendship"
    add foreign key (requester) references "user" ("id");
alter table "friendship"
    add foreign key (responder) references "user" ("id");



CREATE TABLE "game"
(
    "id"       UUID     NOT NULL,
    "user_id"  UUID     NOT NULL,
    "net_gain" BIGINT   NOT NULL,
    "time"     INTERVAL NOT NULL,
    "folds"    INTEGER  NOT NULL,
    "raises"   INTEGER  NOT NULL,
    "wins"     INTEGER  NOT NULL,
    "losses"   INTEGER  NOT NULL
);

alter table "game"
    add foreign key (user_id) references "user" ("id");

ALTER TABLE
    "game"
    ADD PRIMARY KEY ("id");

CREATE TABLE "item"
(
    "id"          UUID                                                                             NOT NULL,
    owner_id      UUID                                                                             NULL,
    "item_id"     UUID                                                                             NOT NULL,
    "type"        VARCHAR(255) CHECK ("type" IN ('chip', 'avatar', 'emote', 'card', 'win_effect')) NOT NULL,
    "rarity"      VARCHAR(255) CHECK ("rarity" IN ('common', 'rare', 'epic', 'legendary',
                                                   'divine'))                                      NOT NULL,
    "season"      int                                                                              NOT null,
    "name"        VARCHAR(255)                                                                     NOT NULL,
    "description" VARCHAR(255)                                                                     NOT NULL,
    "price"       BIGINT                                                                           NULL
);

alter table "item"
    add foreign key (owner_id) references "user" ("id");

ALTER TABLE
    "item"
    ADD PRIMARY KEY ("id");
