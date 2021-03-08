CREATE TABLE users (
    user_id INT GENERATED ALWAYS AS IDENTITY,
    name VARCHAR NOT NULL,
    CONSTRAINT pk_users PRIMARY KEY (user_id),
    UNIQUE(name)
);

CREATE TABLE channel_points (
    streamer_id INT,
    viewer_id INT,
    points INT DEFAULT 0,
    CONSTRAINT pk_channel_points PRIMARY KEY (streamer_id, viewer_id),
    CONSTRAINT fk_channel_points_streamer_id
      FOREIGN KEY(streamer_id) 
	    REFERENCES users(user_id),
    CONSTRAINT fk_channel_points_viewer_id
      FOREIGN KEY(viewer_id) 
	    REFERENCES users(user_id)
);
