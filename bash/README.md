# start_hotspot.sh  

Sript brings wifi down and starts hotspot.  

At you malina create directory `$HOME/app/bash/` and copy files: dnsmasq.conf.hotspot, hostapd.conf.tmpl, start_hotspot.sh.  

!IMPORTANT!  
Before first run of the script, copy the following original files (already existing at your malina): /etc/dnsmasq.conf and /etc/default/hostapd.  
You will use them later to restore configuration when you don't need your hotspot anymore.  
In order to copy files type:  
``` 
sudo mv /etc/dnsmasq.conf /etc/dnsmasq.conf.orig
sudo mv /etc/default/hostapd /etc/default/hostapd.orig
```

Then run your script in the background by typing:
``` 
sudo /bin/bash $HOME/app/bash/start_hotspot.sh & 
```
