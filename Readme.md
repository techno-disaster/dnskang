## DNSKang

A very barebones fork of [dns.toys](https://dns.toys) but in rust.

Made 2 hours before the [dns.toys talk](https://indiafoss.net/2023/confirmed/7601f0b971) at IndiaFOSS 3.0

```bash

dig @localhost -p 20053 dnskang // dig +short @localhost -p 20053 dnskang

; <<>> DiG 9.18.19 <<>> @localhost -p 20053 dnskang
; (2 servers found)
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 4416
;; flags:; QUERY: 0, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 0

;; ANSWER SECTION:
CUSTOM_DATA_QUESTION.   1       IN      CNAME   CUSTOM_DATA_ANSWER.

;; Query time: 0 msec
;; SERVER: 127.0.0.1#20053(localhost) (UDP)
;; WHEN: Mon Oct 30 13:31:42 IST 2023
;; MSG SIZE  rcvd: 64

```


### Refrences:
- dns.toys: https://github.com/knadh/dns.toys
- DNS primitives staright from: https://github.com/EmilHernvall/dnsguide/blob/master/chapter4.md

