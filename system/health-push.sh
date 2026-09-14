#! /bin/bash 
# creating bash for uploading health check to git 
cd /home/jetson/companion 
git add -A
git diff --cached --quiet || git commit -m "Pushing health metrics at $(date)"
git push
