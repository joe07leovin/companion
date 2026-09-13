# health check for HAl print a line 
# Created 13th Sept 2026
DISK=$(df -h / | awk 'NR==2 {print $5}')
TOTAL=$(free -h | awk 'NR==2 {print $2} ')
USED=$(free -h | awk 'NR==2 {print $3}')
CACHED=$(free -h | awk 'NR==2 {print $6} ')
UPTIME=$(awk '{print int($1)}' /proc/uptime)
HOURS=$((UPTIME / 3600 ))
SEC=$((UPTIME % 60 ))
MIN=$(( (UPTIME % 3600 ) / 60 ))
POWER=$(nvpmodel -q  | awk 'NR==1 {print $4}')
WIFI=$(iw dev wlP1p1s0 link | awk 'NR==6 {print $2}') 
echo "disk:$DISK | ram-total:$TOTAL | ram-used:$USED | ram-cached:$CACHED | uptime:$HOURS : $MIN : $SEC | power: $POWER | wifi: $WIFI dbm"  >> /home/jetson/companion/system/SYS_HEALTH.md

