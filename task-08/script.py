import os
import requests

request = requests.get('https://api.github.com/users/amfoss/repos?per_page=100')
repos = request.json()
print("amFOSS repositories\n")
for n in range(0,len(repos)):
  print( n+1, repos[n]['name'] ,"\n")
for n in range(0,len(repos)):
    perceval="perceval git --json-line " + repos[n]['html_url'] + ">> commits.json"
    os.system(perceval)
    
