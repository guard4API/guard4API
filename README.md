# guard4API
Guard4API is a platform for managing APIs. By fronting services with a proxy layer, it provides an abstraction or facade for backend service APIs and provides security, rate limiting, quotas, analytics, throttling and more.

### Http Request format
``` 
Method Request-URI HTTP-Version CRLF
headers CRLF
CRLF
message-body
```

### Http Response format
``` 
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
CRLF
message-body
```
### Echo Server 
```
netcat command to run a dummy server 
nc -l localhost 8888
```