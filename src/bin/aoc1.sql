DROP TABLE IF EXISTS pings;

CREATE TABLE pings(no serial, depth int);

\copy pings(depth) from 'aoc1.dat'

-- Part 1
WITH
  lags AS (SELECT lag(depth) OVER () AS prev, depth AS curr FROM pings OFFSET 1)
SELECT count(*) FROM lags WHERE curr > prev;

-- Part 2
WITH
  sums AS (SELECT sum(depth) OVER (ROWS 2 PRECEDING) FROM pings OFFSET 2),
  lags AS (SELECT sum AS curr, lag(sum) OVER () AS prev FROM sums)
SELECT count(*) FROM lags WHERE curr > prev;

