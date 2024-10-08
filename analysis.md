# Analysis

Most events involved a destination address of `10.41.245.114` (Bravo).

`167.86.122.137` (Echo) appears to be a malicious actor which made repeated web exploit attempts against the HTTP server hosted on `10.41.245.114` (Bravo). that resulted a exploit kit becoming active on `10.41.245.114` (Bravo), at which point `10.41.245.114` (Bravo) started making outbound connections to `85.93.0.32` (Kilo, known as malicious) starting at 21:27:33.

We think `185.86.77.12` (Uniform) is a remote C&C server to which data exfiltration has been attempted starting at 21:28:01, originating from `10.41.245.114` (Bravo). Furthermore, starting at 21:31:49, requests originating from `10.41.245.114` were made to another external C&C server (`192.210.137.123`, November).

`216.58.219.46` (Papa) made repeated attempts to upload a mailicious GIF file to `10.41.245.114` (Bravo).

# Advice

`10.41.245.114` (Bravo) seems to have been compromised. It must be isolated immediately and additional investigation is required.

## IP address overview

The following IP addresses were involved in malicious traffic between `2016-02-05 21:23:15` and `2016-02-05 21:35:48`:

| Address         | VirusTotal score | Identifier |
| --------------- | ---------------- | ---------- |
| 216.39.55.13    | safe             | Alpha      |
| 10.41.245.114   | safe             | Bravo      |
| 31.13.70.7      | safe (debated)   | Charlie    |
| 10.2.41.8       | safe             | Delta      |
| 167.86.122.137  | malicious        | Echo       |
| 54.231.142.56   | safe             | Foxtrot    |
| 80.247.235.238  | safe             | Golf       |
| 98.136.189.41   | safe             | Hotel      |
| 10.41.245.255   | safe             | India      |
| 10.2.41.7       | safe             | Juliett    |
| 85.93.0.32      | malicious        | Kilo       |
| 184.168.47.225  | malicious        | Lima       |
| 185.21.102.206  | safe             | Mike       |
| 192.210.137.123 | safe             | November   |
| 141.0.19.127    | safe             | Oscar      |
| 216.58.219.46   | safe             | Papa       |
| 54.231.130.3    | safe             | Quebec     |
| 185.21.102.206  | safe             | Romeo      |
| 206.190.37.99   | safe             | Sierra     |
| 204.79.197.200  | malicious        | Tango      |
| 185.86.77.12    | safe             | Uniform    |

## Events

```
2016-02-05 21:23:15           Bravo -> India           (3.2326) ET POLICY Reserved Internal IP Traffic
2016-02-05 21:23:18         Juliett -> Bravo           (3.2327) ET POLICY Reserved Internal IP Traffic
2016-02-05 21:23:18         Juliett -> Bravo           (3.2328) ET DNS Standard query response, Name Error
2016-02-05 21:23:18           Delta -> Bravo           (3.2329) ET POLICY Reserved Internal IP Traffic
2016-02-05 21:23:18           Delta -> Bravo           (3.2330) ET DNS Standard query response, Name Error
2016-02-05 21:24:00          Sierra -> Bravo           (3.2331) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:24:00          Sierra -> Bravo           (3.2332) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:24:03           Hotel -> Bravo           (3.2333) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:24:03           Hotel -> Bravo           (3.2334) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:24:03           Alpha -> Bravo           (3.2335) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:24:03           Alpha -> Bravo           (3.2336) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:24:03           Hotel -> Bravo           (3.2337) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:24:03           Hotel -> Bravo           (3.2338) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:24:03           Alpha -> Bravo           (3.2339) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:24:03           Alpha -> Bravo           (3.2340) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:24:03           Hotel -> Bravo           (3.2341) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:24:03           Hotel -> Bravo           (3.2342) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:24:03           Alpha -> Bravo           (3.2343) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:24:03           Alpha -> Bravo           (3.2344) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:24:03           Hotel -> Bravo           (3.2345) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:24:03           Hotel -> Bravo           (3.2346) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:25:31           Tango -> Bravo           (3.2347) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:25:31           Tango -> Bravo           (3.2348) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:25:31           Tango -> Bravo           (3.2349) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:25:31           Tango -> Bravo           (3.2350) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:25:31           Tango -> Bravo           (3.2351) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:25:31           Tango -> Bravo           (3.2352) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:25:31           Tango -> Bravo           (3.2353) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:25:31           Tango -> Bravo           (3.2354) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:25:36           Tango -> Bravo           (3.2355) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:25:36           Tango -> Bravo           (3.2356) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:12           Tango -> Bravo           (3.2363) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:12           Tango -> Bravo           (3.2364) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:12           Tango -> Bravo           (3.2365) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:12           Tango -> Bravo           (3.2366) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:12           Tango -> Bravo           (3.2367) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:12           Tango -> Bravo           (3.2368) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:12           Tango -> Bravo           (3.2369) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:27            Echo -> Bravo           (3.2370) SURICATA STREAM 3way handshake with ack in wrong dir
2016-02-05 21:27:27            Echo -> Bravo           (3.2371) ET SHELLCODE UTF-8/16 Encoded Shellcode
2016-02-05 21:27:27            Echo -> Bravo           (3.2372) ET SHELLCODE UTF-8/16 Encoded Shellcode
2016-02-05 21:27:27            Echo -> Bravo           (3.2373) ET SHELLCODE UTF-8/16 Encoded Shellcode
2016-02-05 21:27:27            Echo -> Bravo           (3.2374) ET SHELLCODE UTF-8/16 Encoded Shellcode
2016-02-05 21:27:27            Echo -> Bravo           (3.2375) ET SHELLCODE UTF-8/16 Encoded Shellcode
2016-02-05 21:27:27            Echo -> Bravo           (3.2376) ET SHELLCODE UTF-8/16 Encoded Shellcode
2016-02-05 21:27:27            Echo -> Bravo           (3.2377) ET WEB_CLIENT Possible Malicious String.fromCharCode with charCodeAt String
2016-02-05 21:27:27            Echo -> Bravo           (3.2378) ET WEB_CLIENT Possible Malicious String.fromCharCode with charCodeAt String
2016-02-05 21:27:27            Echo -> Bravo           (3.2379) ET WEB_CLIENT Possible Malicious String.fromCharCode with charCodeAt String
2016-02-05 21:27:27            Echo -> Bravo           (3.2380) ET WEB_CLIENT Possible Malicious String.fromCharCode with charCodeAt String
2016-02-05 21:27:27            Echo -> Bravo           (3.2381) ET WEB_CLIENT Possible Malicious String.fromCharCode with charCodeAt String
2016-02-05 21:27:27            Echo -> Bravo           (3.2382) ET WEB_CLIENT Possible Malicious String.fromCharCode with charCodeAt String
2016-02-05 21:27:28            Echo -> Bravo           (3.2383) FILE tracking PNG (1x1 pixel) (1)
2016-02-05 21:27:28            Echo -> Bravo           (3.2384) FILE tracking PNG (1x1 pixel) (1)
2016-02-05 21:27:29            Echo -> Bravo           (3.2385) ET WEB_CLIENT Possible String.FromCharCode Javascript Obfuscation Attempt
2016-02-05 21:27:29            Echo -> Bravo           (3.2386) ET WEB_CLIENT Possible String.FromCharCode Javascript Obfuscation Attempt
2016-02-05 21:27:29            Echo -> Bravo           (3.2387) ET WEB_CLIENT Possible String.FromCharCode Javascript Obfuscation Attempt
2016-02-05 21:27:29            Echo -> Bravo           (3.2388) ET WEB_CLIENT Possible String.FromCharCode Javascript Obfuscation Attempt
2016-02-05 21:27:29            Echo -> Bravo           (3.2389) ET WEB_CLIENT Possible String.FromCharCode Javascript Obfuscation Attempt
2016-02-05 21:27:29            Echo -> Bravo           (3.2390) ET WEB_CLIENT Possible String.FromCharCode Javascript Obfuscation Attempt
2016-02-05 21:27:29            Echo -> Bravo           (3.2391) ETPRO WEB_CLIENT Mozilla Firefox IFRAME Cross Site Scripting
2016-02-05 21:27:29            Echo -> Bravo           (3.2392) ETPRO WEB_CLIENT Mozilla Firefox IFRAME Cross Site Scripting
2016-02-05 21:27:29            Echo -> Bravo           (3.2393) ETPRO WEB_CLIENT Mozilla Firefox IFRAME Cross Site Scripting
2016-02-05 21:27:29            Echo -> Bravo           (3.2394) ETPRO WEB_CLIENT Mozilla Firefox IFRAME Cross Site Scripting
2016-02-05 21:27:29            Echo -> Bravo           (3.2395) ETPRO WEB_CLIENT Mozilla Firefox IFRAME Cross Site Scripting
2016-02-05 21:27:29            Echo -> Bravo           (3.2396) ETPRO WEB_CLIENT Mozilla Firefox IFRAME Cross Site Scripting
2016-02-05 21:27:32           Bravo -> Juliett         (3.2524) ET DNS Query to a .tk domain - Likely Hostile
2016-02-05 21:27:33         Charlie -> Bravo           (3.2532) ET SHELLCODE UTF-8/16 Encoded Shellcode
2016-02-05 21:27:33         Charlie -> Bravo           (3.2533) ET SHELLCODE UTF-8/16 Encoded Shellcode
2016-02-05 21:27:33         Charlie -> Bravo           (3.2534) ET SHELLCODE UTF-8/16 Encoded Shellcode
2016-02-05 21:27:33         Charlie -> Bravo           (3.2535) ET SHELLCODE UTF-8/16 Encoded Shellcode
2016-02-05 21:27:33         Charlie -> Bravo           (3.2536) ET SHELLCODE UTF-8/16 Encoded Shellcode
2016-02-05 21:27:33            Echo -> Bravo           (3.2537) FILE tracking GIF (1x1 pixel)
2016-02-05 21:27:33            Echo -> Bravo           (3.2538) FILE tracking GIF (1x1 pixel)
2016-02-05 21:27:33            Echo -> Bravo           (3.2539) GPL WEB_CLIENT web bug 0x0 gif attempt
2016-02-05 21:27:33          Quebec -> Bravo           (3.2540) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:33          Quebec -> Bravo           (3.2541) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:33          Quebec -> Bravo           (3.2542) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:33          Quebec -> Bravo           (3.2543) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:33           Bravo -> Kilo            (3.2544) ET CURRENT_EVENTS SUSPICIOUS Likely Neutrino EK or other EK IE Flash reques
2016-02-05 21:27:33           Bravo -> Kilo            (3.2545) ET CURRENT_EVENTS Possible Evil Redirector Leading to EK Nov 09 2015 M1
2016-02-05 21:27:33           Bravo -> Kilo            (3.2546) ET POLICY HTTP Request to a *.tk domain
2016-02-05 21:27:33         Foxtrot -> Bravo           (3.2563) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:33         Foxtrot -> Bravo           (3.2566) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:33         Foxtrot -> Bravo           (3.2567) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:33         Foxtrot -> Bravo           (3.2568) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:33         Foxtrot -> Bravo           (3.2569) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:33         Foxtrot -> Bravo           (3.2570) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:34           Bravo -> Kilo            (3.2600) ET CURRENT_EVENTS Possible Evil Redirector Leading to EK Nov 09 2015 M2
2016-02-05 21:27:34           Bravo -> Kilo            (3.2601) ET POLICY HTTP Request to a *.tk domain
2016-02-05 21:27:34           Bravo -> Juliett         (3.2602) ET DNS Query to a *.pw domain - Likely Hostile
2016-02-05 21:27:34           Oscar -> Bravo           (3.2619) GPL WEB_CLIENT web bug 0x0 gif attempt
2016-02-05 21:27:35           Bravo -> Lima            (3.2620) ETPRO CURRENT_EVENTS Possible Angler EK Landing URI Struct Aug 5 M1 T1
2016-02-05 21:27:35           Bravo -> Lima            (3.2621) ET INFO HTTP Request to a *.pw domain
2016-02-05 21:27:36         Foxtrot -> Bravo           (3.2622) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:37         Foxtrot -> Bravo           (3.2623) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:37         Foxtrot -> Bravo           (3.2624) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:37            Lima -> Bravo           (3.2625) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:38         Foxtrot -> Bravo           (3.2626) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:38         Foxtrot -> Bravo           (3.2627) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:38         Foxtrot -> Bravo           (3.2628) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:38         Foxtrot -> Bravo           (3.2629) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:38         Foxtrot -> Bravo           (3.2630) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:38           Bravo -> Lima            (3.2631) ET INFO HTTP Request to a *.pw domain
2016-02-05 21:27:38         Foxtrot -> Bravo           (3.2632) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:27:38           Bravo -> Lima            (3.2633) ETPRO CURRENT_EVENTS Possible Angler EK IE DHE Post M3
2016-02-05 21:27:38           Bravo -> Lima            (3.2634) ETPRO CURRENT_EVENTS Possible Angler EK IE DHE Post M3
2016-02-05 21:27:38           Bravo -> Lima            (3.2635) ET INFO HTTP Request to a *.pw domain
2016-02-05 21:27:38           Bravo -> Lima            (3.2636) ET INFO HTTP Request to a *.pw domain
2016-02-05 21:27:38           Bravo -> Lima            (3.2637) ETPRO CURRENT_EVENTS Angler or Nuclear EK Flash Exploit (IE) Jun 16 M1 T2
2016-02-05 21:27:38           Bravo -> Lima            (3.2638) ET CURRENT_EVENTS SUSPICIOUS Likely Neutrino EK or other EK IE Flash reques
2016-02-05 21:27:38           Bravo -> Lima            (3.2639) ET INFO HTTP Request to a *.pw domain
2016-02-05 21:27:38            Lima -> Bravo           (3.2640) ETPRO CURRENT_EVENTS Angler or Nuclear EK Flash Exploit M2
2016-02-05 21:27:42           Bravo -> Lima            (3.2644) ET INFO HTTP Request to a *.pw domain
2016-02-05 21:27:50           Bravo -> Uniform         (3.2646) ET TROJAN Ursnif Variant CnC Beacon
2016-02-05 21:27:50           Bravo -> Uniform         (3.2647) ET TROJAN Ursnif Variant CnC Beacon 4
2016-02-05 21:28:01           Bravo -> Uniform         (3.2649) ET TROJAN Ursnif Variant CnC Data Exfil
2016-02-05 21:28:44         Juliett -> Bravo           (3.2650) ET DNS Standard query response, Name Error
2016-02-05 21:28:46            Papa -> Bravo           (3.2651) FILE tracking GIF (1x1 pixel)
2016-02-05 21:28:46            Papa -> Bravo           (3.2652) GPL WEB_CLIENT web bug 0x0 gif attempt
2016-02-05 21:28:46            Papa -> Bravo           (3.2653) FILE tracking GIF (1x1 pixel)
2016-02-05 21:28:46            Papa -> Bravo           (3.2654) GPL WEB_CLIENT web bug 0x0 gif attempt
2016-02-05 21:29:47           Bravo -> Juliett         (3.2655) ET POLICY Reserved Internal IP Traffic
2016-02-05 21:29:47         Juliett -> Bravo           (3.2656) ET POLICY Reserved Internal IP Traffic
2016-02-05 21:29:52           Bravo -> Uniform         (3.2657) ET TROJAN Ursnif Variant CnC Beacon
2016-02-05 21:29:52           Bravo -> Uniform         (3.2658) ET TROJAN Ursnif Variant CnC Beacon 4
2016-02-05 21:30:03           Bravo -> Uniform         (3.2659) ET TROJAN Ursnif Variant CnC Data Exfil
2016-02-05 21:31:31            Golf -> Bravo           (3.2662) FILE tracking GIF (1x1 pixel)
2016-02-05 21:31:31            Golf -> Bravo           (3.2663) FILE tracking GIF (1x1 pixel)
2016-02-05 21:31:31            Golf -> Bravo           (3.2664) GPL WEB_CLIENT web bug 0x0 gif attempt
2016-02-05 21:31:49           Bravo -> November        (3.2723) ET TROJAN Ursnif Variant CnC Beacon
2016-02-05 21:32:26           Bravo -> November        (3.2733) ET TROJAN Ursnif Variant CnC Data Exfil
2016-02-05 21:32:26           Bravo -> November        (3.2734) ET TROJAN Ursnif Variant CnC Data Exfil
2016-02-05 21:33:20         Juliett -> Bravo           (3.3471) ET DNS Standard query response, Name Error
2016-02-05 21:33:48            Papa -> Bravo           (3.3472) FILE tracking GIF (1x1 pixel)
2016-02-05 21:33:48            Papa -> Bravo           (3.3473) GPL WEB_CLIENT web bug 0x0 gif attempt
2016-02-05 21:34:25           Bravo -> November        (3.3523) ET TROJAN Ursnif Variant CnC Data Exfil
2016-02-05 21:34:25           Bravo -> November        (3.3524) ET TROJAN Ursnif Variant CnC Data Exfil
2016-02-05 21:34:25           Bravo -> November        (3.3525) ET TROJAN Ursnif Variant CnC Data Exfil
2016-02-05 21:34:25           Bravo -> November        (3.3526) ET TROJAN Ursnif Variant CnC Data Exfil
2016-02-05 21:34:25           Bravo -> November        (3.3527) ET TROJAN Ursnif Variant CnC Data Exfil
2016-02-05 21:34:31           Romeo -> Bravo           (3.3528) SURICATA STREAM reassembly segment before base seq
2016-02-05 21:34:31           Romeo -> Bravo           (3.3529) SURICATA STREAM reassembly segment before base seq
2016-02-05 21:34:31           Romeo -> Bravo           (3.3530) SURICATA STREAM reassembly segment before base seq
2016-02-05 21:34:31           Romeo -> Bravo           (3.3531) SURICATA STREAM reassembly segment before base seq
2016-02-05 21:34:31           Romeo -> Bravo           (3.3532) SURICATA STREAM reassembly segment before base seq
2016-02-05 21:34:31           Romeo -> Bravo           (3.3533) SURICATA STREAM reassembly segment before base seq
2016-02-05 21:34:31           Romeo -> Bravo           (3.3534) SURICATA STREAM ESTABLISHED retransmission packet before last ack
2016-02-05 21:34:31           Romeo -> Bravo           (3.3535) SURICATA STREAM reassembly segment before base seq
2016-02-05 21:35:48            Papa -> Bravo           (3.3536) FILE tracking GIF (1x1 pixel)
2016-02-05 21:35:48            Papa -> Bravo           (3.3537) GPL WEB_CLIENT web bug 0x0 gif attempt
```

### 184.168.47.225 (Alpha - 2 events)

```
Alpha -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Alpha -> Bravo (ETPRO CURRENT_EVENTS Angler or Nuclear EK Flash Exploit M2)
```

### 54.231.130.3 (Charlie - 4 events)

```
Charlie -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Charlie -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Charlie -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Charlie -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
```

### 31.13.70.7 (Delta - 5 events)

```
Delta -> Bravo (ET SHELLCODE UTF-8/16 Encoded Shellcode)
Delta -> Bravo (ET SHELLCODE UTF-8/16 Encoded Shellcode)
Delta -> Bravo (ET SHELLCODE UTF-8/16 Encoded Shellcode)
Delta -> Bravo (ET SHELLCODE UTF-8/16 Encoded Shellcode)
Delta -> Bravo (ET SHELLCODE UTF-8/16 Encoded Shellcode)
```

### 216.58.219.46 (Echo - 8 events)

```
Echo -> Bravo (FILE tracking GIF (1x1 pixel))
Echo -> Bravo (GPL WEB_CLIENT web bug 0x0 gif attempt)
Echo -> Bravo (FILE tracking GIF (1x1 pixel))
Echo -> Bravo (GPL WEB_CLIENT web bug 0x0 gif attempt)
Echo -> Bravo (FILE tracking GIF (1x1 pixel))
Echo -> Bravo (GPL WEB_CLIENT web bug 0x0 gif attempt)
Echo -> Bravo (FILE tracking GIF (1x1 pixel))
Echo -> Bravo (GPL WEB_CLIENT web bug 0x0 gif attempt)
```

### 80.247.235.238 (Foxtrot - 3 events)

```
Foxtrot -> Bravo (FILE tracking GIF (1x1 pixel))
Foxtrot -> Bravo (FILE tracking GIF (1x1 pixel))
Foxtrot -> Bravo (GPL WEB_CLIENT web bug 0x0 gif attempt)
```

### 10.2.41.7 (Golf - 5 events)

```
Golf -> Bravo (ET POLICY Reserved Internal IP Traffic)
Golf -> Bravo (ET DNS Standard query response, Name Error)
Golf -> Bravo (ET DNS Standard query response, Name Error)
Golf -> Bravo (ET POLICY Reserved Internal IP Traffic)
Golf -> Bravo (ET DNS Standard query response, Name Error)
```

### 204.79.197.200 (Hotel - 17 events)

```
Hotel -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Hotel -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Hotel -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Hotel -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Hotel -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Hotel -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Hotel -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Hotel -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Hotel -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Hotel -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Hotel -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Hotel -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Hotel -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Hotel -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Hotel -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Hotel -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Hotel -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
```

### 167.86.122.137 (India - 30 events)

```
India -> Bravo (SURICATA STREAM 3way handshake with ack in wrong dir)
India -> Bravo (ET SHELLCODE UTF-8/16 Encoded Shellcode)
India -> Bravo (ET SHELLCODE UTF-8/16 Encoded Shellcode)
India -> Bravo (ET SHELLCODE UTF-8/16 Encoded Shellcode)
India -> Bravo (ET SHELLCODE UTF-8/16 Encoded Shellcode)
India -> Bravo (ET SHELLCODE UTF-8/16 Encoded Shellcode)
India -> Bravo (ET SHELLCODE UTF-8/16 Encoded Shellcode)
India -> Bravo (ET WEB_CLIENT Possible Malicious String.fromCharCode with charCodeAt String)
India -> Bravo (ET WEB_CLIENT Possible Malicious String.fromCharCode with charCodeAt String)
India -> Bravo (ET WEB_CLIENT Possible Malicious String.fromCharCode with charCodeAt String)
India -> Bravo (ET WEB_CLIENT Possible Malicious String.fromCharCode with charCodeAt String)
India -> Bravo (ET WEB_CLIENT Possible Malicious String.fromCharCode with charCodeAt String)
India -> Bravo (ET WEB_CLIENT Possible Malicious String.fromCharCode with charCodeAt String)
India -> Bravo (FILE tracking PNG (1x1 pixel) (1))
India -> Bravo (FILE tracking PNG (1x1 pixel) (1))
India -> Bravo (ET WEB_CLIENT Possible String.FromCharCode Javascript Obfuscation Attempt)
India -> Bravo (ET WEB_CLIENT Possible String.FromCharCode Javascript Obfuscation Attempt)
India -> Bravo (ET WEB_CLIENT Possible String.FromCharCode Javascript Obfuscation Attempt)
India -> Bravo (ET WEB_CLIENT Possible String.FromCharCode Javascript Obfuscation Attempt)
India -> Bravo (ET WEB_CLIENT Possible String.FromCharCode Javascript Obfuscation Attempt)
India -> Bravo (ET WEB_CLIENT Possible String.FromCharCode Javascript Obfuscation Attempt)
India -> Bravo (ETPRO WEB_CLIENT Mozilla Firefox IFRAME Cross Site Scripting)
India -> Bravo (ETPRO WEB_CLIENT Mozilla Firefox IFRAME Cross Site Scripting)
India -> Bravo (ETPRO WEB_CLIENT Mozilla Firefox IFRAME Cross Site Scripting)
India -> Bravo (ETPRO WEB_CLIENT Mozilla Firefox IFRAME Cross Site Scripting)
India -> Bravo (ETPRO WEB_CLIENT Mozilla Firefox IFRAME Cross Site Scripting)
India -> Bravo (ETPRO WEB_CLIENT Mozilla Firefox IFRAME Cross Site Scripting)
India -> Bravo (FILE tracking GIF (1x1 pixel))
India -> Bravo (FILE tracking GIF (1x1 pixel))
India -> Bravo (GPL WEB_CLIENT web bug 0x0 gif attempt)
```

### 185.21.102.206 (Juliett - 8 events)

```
Juliett -> Bravo (SURICATA STREAM reassembly segment before base seq)
Juliett -> Bravo (SURICATA STREAM reassembly segment before base seq)
Juliett -> Bravo (SURICATA STREAM reassembly segment before base seq)
Juliett -> Bravo (SURICATA STREAM reassembly segment before base seq)
Juliett -> Bravo (SURICATA STREAM reassembly segment before base seq)
Juliett -> Bravo (SURICATA STREAM reassembly segment before base seq)
Juliett -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Juliett -> Bravo (SURICATA STREAM reassembly segment before base seq)
```

### 10.41.245.114 (Bravo - 34 events)

```
Bravo -> Kilo (ET POLICY Reserved Internal IP Traffic)
Bravo -> Golf (ET DNS Query to a .tk domain - Likely Hostile)
Bravo -> Lima (ET CURRENT*EVENTS SUSPICIOUS Likely Neutrino EK or other EK IE Flash request to DYNDNS set non-standard filename)
Bravo -> Lima (ET CURRENT_EVENTS Possible Evil Redirector Leading to EK Nov 09 2015 M1)
Bravo -> Lima (ET POLICY HTTP Request to a *.tk domain)
Bravo -> Lima (ET CURRENT*EVENTS Possible Evil Redirector Leading to EK Nov 09 2015 M2)
Bravo -> Lima (ET POLICY HTTP Request to a *.tk domain)
Bravo -> Golf (ET DNS Query to a _.pw domain - Likely Hostile)
Bravo -> Alpha (ETPRO CURRENT_EVENTS Possible Angler EK Landing URI Struct Aug 5 M1 T1)
Bravo -> Alpha (ET INFO HTTP Request to a _.pw domain)
Bravo -> Alpha (ET INFO HTTP Request to a _.pw domain)
Bravo -> Alpha (ETPRO CURRENT_EVENTS Possible Angler EK IE DHE Post M3)
Bravo -> Alpha (ETPRO CURRENT_EVENTS Possible Angler EK IE DHE Post M3)
Bravo -> Alpha (ET INFO HTTP Request to a _.pw domain)
Bravo -> Alpha (ET INFO HTTP Request to a _.pw domain)
Bravo -> Alpha (ETPRO CURRENT_EVENTS Angler or Nuclear EK Flash Exploit (IE) Jun 16 M1 T2)
Bravo -> Alpha (ET CURRENT_EVENTS SUSPICIOUS Likely Neutrino EK or other EK IE Flash request to DYNDNS set non-standard filename)
Bravo -> Alpha (ET INFO HTTP Request to a _.pw domain)
Bravo -> Alpha (ET INFO HTTP Request to a \*.pw domain)
Bravo -> Mike (ET TROJAN Ursnif Variant CnC Beacon)
Bravo -> Mike (ET TROJAN Ursnif Variant CnC Beacon 4)
Bravo -> Mike (ET TROJAN Ursnif Variant CnC Data Exfil)
Bravo -> Golf (ET POLICY Reserved Internal IP Traffic)
Bravo -> Mike (ET TROJAN Ursnif Variant CnC Beacon)
Bravo -> Mike (ET TROJAN Ursnif Variant CnC Beacon 4)
Bravo -> Mike (ET TROJAN Ursnif Variant CnC Data Exfil)
Bravo -> November (ET TROJAN Ursnif Variant CnC Beacon)
Bravo -> November (ET TROJAN Ursnif Variant CnC Data Exfil)
Bravo -> November (ET TROJAN Ursnif Variant CnC Data Exfil)
Bravo -> November (ET TROJAN Ursnif Variant CnC Data Exfil)
Bravo -> November (ET TROJAN Ursnif Variant CnC Data Exfil)
Bravo -> November (ET TROJAN Ursnif Variant CnC Data Exfil)
Bravo -> November (ET TROJAN Ursnif Variant CnC Data Exfil)
Bravo -> November (ET TROJAN Ursnif Variant CnC Data Exfil)
```

### 98.136.189.41 (Oscar - 8 events)

```
Oscar -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Oscar -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Oscar -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Oscar -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Oscar -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Oscar -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Oscar -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Oscar -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
```

### 54.231.142.56 (Papa - 15 events)

```
Papa -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Papa -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Papa -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Papa -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Papa -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Papa -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Papa -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Papa -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Papa -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Papa -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Papa -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Papa -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Papa -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Papa -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Papa -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
```

### 206.190.37.99 (Quebec - 2 events)

```
Quebec -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Quebec -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
```

### 216.39.55.13 (Romeo - 6 events)

```
Romeo -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Romeo -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Romeo -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Romeo -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Romeo -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
Romeo -> Bravo (SURICATA STREAM ESTABLISHED retransmission packet before last ack)
```

### 141.0.19.127 (Sierra - 1 events)

```
Sierra -> Bravo (GPL WEB_CLIENT web bug 0x0 gif attempt)
```

### 10.2.41.8 (Tango - 2 events)

```
Tango -> Bravo (ET POLICY Reserved Internal IP Traffic)
Tango -> Bravo (ET DNS Standard query response, Name Error)
```
