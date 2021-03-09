CREATE TABLE users (
    user_id INT GENERATED ALWAYS AS IDENTITY,
    name VARCHAR NOT NULL,
    CONSTRAINT pk_users PRIMARY KEY (user_id),
    UNIQUE(name)
);

CREATE TABLE user_data (
    streamer_id INT,
    viewer_id INT,
    points INT DEFAULT 0 NOT NULL,
    CONSTRAINT pk_user_data PRIMARY KEY (streamer_id, viewer_id),
    CONSTRAINT fk_user_data_streamer_id
      FOREIGN KEY(streamer_id) 
	    REFERENCES users(user_id),
    CONSTRAINT fk_user_data_viewer_id
      FOREIGN KEY(viewer_id) 
	    REFERENCES users(user_id)
);
