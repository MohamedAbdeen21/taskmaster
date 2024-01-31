from tm.cron import Expression
from datetime import datetime

# supports:

# - Ranges (inclusive)
# --- (0-15 * * * *)

# - Intervals 
# --- (*/15 * * * *) every fifteen minutes
# --- (0-30/15 * * * *) every fifteen minutes from 0 to 30

# - Lists 
# --- (0 0,12 * * *) once at midnight and midday everyday

# - Exact values
# --- (0 0 1 1 *) once at start of year
# --- (0 0 * * 0) once every Sunday (Sunday = 0, Saturday = 6)

# - month/day-of-week names (case-insensitive)
# --- (0 0 1 jan-jun *)
# --- (0 0 1 * mon-thu)

# - combinations of all the above
# --- (40,0-30/15 * * * *)
# --- (* */12 2 jan,jun thu-sat)

# minutes 1,2 and every 15 minutes (0, 15, 30, 45)
# at hour 00
# first day of March
# and every Monday of March
expr = Expression("1-2,*/15 0 1 3 2")
t = datetime(2024,1,31,20)
t = expr.next(t)
print(t)
