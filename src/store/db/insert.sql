INSERT INTO log (time, updated_on, graph, status)
VALUES (?,?,?,?)
RETURNING id
