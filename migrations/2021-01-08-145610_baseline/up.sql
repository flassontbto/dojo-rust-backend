CREATE TABLE users(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    pseudo VARCHAR(255) NOT NULL
);

CREATE TABLE bins(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    author_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    code TEXT NOT NULL,
    FOREIGN KEY (author_id) REFERENCES users(id) 
);

CREATE TABLE likes(
    user_id INTEGER NOT NULL,
    bin_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, bin_id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (bin_id) REFERENCES bins(id) 
);