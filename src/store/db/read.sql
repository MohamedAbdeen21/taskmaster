SELECT id, time, graph, status, updated_on
FROM log 
WHERE graph = ?
ORDER BY time DESC
LIMIT 1
