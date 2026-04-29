TCP - Transmission Control Protocol
UDP - User Datagram Protocol
TCP is connection oriented so is most reliable in those situations, examples being http/https, ssh and ftp (File Transfer Protocol)
UDP is connectionless, so things like a streaming service, dns, or voice over ip

TCP - Three-Way Handshake

Machine 1 (the client) sends a syn or synchronisation packet to Machine 2 (the server)
M1 > SYN > M2

Machine 2 then responds with a synchronisation acknowledgement packet or SYN ACK
M2 > SYN ACK > M1

And finally M1 Responds with an acknowledgement packet or just ACK
M1 > ACK > M2

This happens over ports so lets do this over with more complicated steps

M1 > SYN p80 > M2

M2 > SYN ACK p80 > M1

M1 > ACK p80 > M2

This is a simple TCP handshake through port 80 (http)

You can see this in real time by just passively capturing packets with wireshark
